use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};
use mime_guess;
use rustls_pemfile;
use std::fs::File;
use std::io::BufReader;

use crate::config::config::AppConfig;
use crate::db::db_pool::DuckDBConnectionManager;
use crate::server::web_handlers::{api_get_arrow, api_get_datasets, api_get_json, serve_embedded};

/// Sets up the Actix app with shared routes and middleware, then starts the HTTP server.
/// This function conditionally serves static files from disk in debug builds and
/// uses embedded assets in release builds.
pub async fn run_server(
    pool: r2d2::Pool<DuckDBConnectionManager>,
    config_data: web::Data<AppConfig>,
) -> Result<()> {
    // Common app factory closure.
    let app_factory = {
        let pool = pool.clone();
        let config_data = config_data.clone();
        move || {
            let app = App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(config_data.clone())
                .wrap(Logger::default())
                .route("/api/data/json", web::get().to(api_get_json))
                .route("/api/data/arrow/{dataset}", web::get().to(api_get_arrow))
                .route("/api/datasets", web::get().to(api_get_datasets));

            // Conditionally add the frontend routes:
            // In debug builds, serve files from disk.
            #[cfg(debug_assertions)]
            let app = app.service(Files::new("/", "./static").index_file("index.html"));

            // In release builds, serve embedded assets.
            //#[cfg(not(debug_assertions))]
            let app = app.route("/{filename:.*}", web::get().to(serve_embedded));

            app
        }
    };

    // Set up the server: use TLS if enabled, otherwise plain HTTP.
    let server = if config_data.security.https.enabled {
        // ----- HTTPS Setup -----
        let mut certs_file = BufReader::new(
            File::open(&config_data.security.https.cert_path)
                .with_context(|| format!("Cannot open cert file {}", &config_data.security.https.cert_path))?
        );
        let mut key_file = BufReader::new(
            File::open(&config_data.security.https.key_path)
                .with_context(|| format!("Cannot open key file {}", &config_data.security.https.key_path))?
        );
        let tls_certs = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()?;
        let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .next().unwrap()?;
        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))?;

        println!("Starting HTTPS server on port 8443");
        HttpServer::new(app_factory)
            .bind_rustls_0_23(("0.0.0.0", 8443), tls_config)?
            .run()
    } else {
        // ----- Plain HTTP Setup -----
        println!("Starting HTTP server on port 8080");
        HttpServer::new(app_factory)
            .bind(("0.0.0.0", 8080))?
            .keep_alive(actix_web::http::KeepAlive::Os)
            .run()
    };

    server.await?;
    Ok(())
}


