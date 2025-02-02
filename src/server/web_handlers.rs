use std::borrow::Cow;
use std::ops::Deref;
use crate::config::config::AppConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use arrow::ipc::writer::StreamWriter;
use r2d2::Pool;
use crate::server::web_embed::Frontend;

pub async fn api_get_arrow(
    path: web::Path<String>,
    data: web::Data<Pool<crate::db::db_pool::DuckDBConnectionManager>>,
) -> impl Responder {
    let table_name = path.into_inner();

    // Get a connection from the pool.
    let conn = match data.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().body("Error getting connection");
        }
    };

    // Prepare and execute the SELECT query.
    let query = format!("SELECT * FROM \"{}\"", table_name);
    let mut stmt = match conn.prepare(&query) {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("Error preparing query for {}: {:?}", table_name, e);
            return HttpResponse::InternalServerError().body("Error preparing query");
        }
    };

    // Get result as a single Arrow batch.
    let arrow_batch = match stmt.query_arrow([]) {
        Ok(batch) => batch,
        Err(e) => {
            eprintln!("Error executing query_arrow for {}: {:?}", table_name, e);
            return HttpResponse::InternalServerError().body("Error executing query_arrow");
        }
    };

    let schema = arrow_batch.get_schema();

    // Collect the Arrow batch into a Vec of RecordBatch.
    let record_batch = arrow_batch.collect::<Vec<_>>().to_vec();

    // Serialize the RecordBatch to an Arrow IPC stream.
    let mut buffer = Vec::new();

    // If there are no record batches (e.g., empty table), handle that case appropriately.
    if record_batch.is_empty() {
        return HttpResponse::Ok()
            .content_type("application/vnd.apache.arrow.stream")
            .body(buffer);
    }

    let mut stream_writer = match StreamWriter::try_new(&mut buffer, schema.deref()) {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Error creating StreamWriter for {}: {:?}", table_name, e);
            return HttpResponse::InternalServerError().body("Error creating Arrow stream");
        }
    };

    for batch in record_batch {
        if let Err(e) = stream_writer.write(&batch) {
            eprintln!("Error writing batch for {}: {:?}", table_name, e);
            return HttpResponse::InternalServerError().body("Error writing Arrow batch");
        }
    }

    if let Err(e) = stream_writer.finish() {
        eprintln!("Error finishing Arrow stream for {}: {:?}", table_name, e);
        return HttpResponse::InternalServerError().body("Error finishing Arrow stream");
    }

    let buff = buffer;

    // Return the serialized Arrow IPC stream as the HTTP response body.
    HttpResponse::Ok()
        .content_type("application/vnd.apache.arrow.stream")
        .body(buff)
}


pub async fn api_get_datasets(config: web::Data<AppConfig>) -> impl Responder {
    let dataset_names: Vec<String> = config.datasets.iter()
        .map(|d| d.name.clone())
        .collect();
    HttpResponse::Ok().json(dataset_names)
}

/// Example API endpoint: returns a simple JSON response.
pub async fn api_get_json() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "id": 1,
        "value": "Hello from Rust!"
    }))
}

/// Handler for serving embedded frontend assets (used in release builds).
pub async fn serve_embedded(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("filename");
    let path = if path.is_empty() { "index.html" } else { path };

    match Frontend::get(path) {
        Some(content) => {
            let body: Cow<[u8]> = content.data.into();
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime_type.as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
