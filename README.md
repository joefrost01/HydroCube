[![CI Pipeline](https://github.com/joefrost01/HydroCube/actions/workflows/build.yml/badge.svg)](https://github.com/joefrost01/HydroCube/actions/workflows/build.yml)
[![Quarto Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://joefrost01.github.io/HydroCube/)

# HydroCube

**HydroCube** is a **near-real-time OLAP server** that ingests data from multiple sources (CSV, Parquet, Kafka, and more) and runs lightning-fast queries in **DuckDB**. Written in **Rust** and bundled with a **FINOS Perspective** UI, HydroCube offers a single-binary deployment that’s **secure**, **scalable**, and **easy to use**—no extra components required.

## Highlights

- **Continuous Ingestion**  
  Pull data from CSV, Parquet, JSON, or Kafka.
- **Blazing-Fast Queries**  
  Relies on **DuckDB** for vectorized execution and minimal memory overhead.
- **Embeddable UI**  
  Built-in **FINOS Perspective** for pivoting, charting, and slicing data in real time.
- **One Binary**  
  Rust-based and self-contained—just download and run, or drop into a **Docker** container.
- **Security & Auth**  
  Supports **HTTPS** by default, plus **OAuth** for authentication.
- **Multi-User**  
  Share up-to-date analytics across your team with real-time WebSocket updates.

## Documentation

For **installation**, **configuration**, and **how-to guides**, check out the **[official docs](https://joefrost01.github.io/HydroCube/)**.

## Quick Installation

1. **Grab the Latest Release**  
   From [GitHub Releases](https://github.com/joefrost01/HydroCube/releases) for Linux, macOS, or Windows.
2. **Run the Binary**  
   Place it in your `$PATH` or reference it directly.

## Building from Source

```bash
git clone https://github.com/joefrost01/HydroCube.git
cd HydroCube
cargo build --release
```

This produces a `hydrocube` (or `hydrocube.exe`) in `target/release`.

## Usage

Run HydroCube with a simple YAML config:

```bash
./hydrocube --config my_config.yaml
```

By default, it serves the Perspective UI at `http://localhost:8080`.  
Check the docs for advanced ingestion setups, security, and deployment best practices.

## Roadmap

- **Custom Aggregator Queries** – Advanced SQL transformations beyond simple group-bys.
- **Alerts & Triggers** – Real-time notifications when aggregates cross thresholds.
- **Additional Data Sources** – Postgres, ClickHouse, CDC streams, etc.
- **Natural Language Querying** – LLM-based interface to let users ask “plain English” questions.
- **Multi-Tenant Security** – Role-based permissions and dataset-level access control.

View the full roadmap in the [docs](https://joefrost01.github.io/HydroCube/) or our [GitHub Issues](https://github.com/joefrost01/HydroCube/issues).

---

© 2025 HydroCube – **Lightweight, Real-Time Analytics**. Contributions welcome! Check the [Contributor Guide](https://joefrost01.github.io/HydroCube/) for details.
