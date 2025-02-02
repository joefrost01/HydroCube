[![CI Pipeline](https://github.com/joefrost01/HydroCube/actions/workflows/build.yml/badge.svg)](https://github.com/joefrost01/HydroCube/actions/workflows/build.yml)
[![Quarto Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://joefrost01.github.io/hydrocube/)

# HydroCube

**HydroCube** is a near realtime OLAP server that allows you to ingest data from multiple sources and query it in a fast and memory-efficient way. It is designed to be easy to use, secure, and scalable. It is written in Rust and uses DuckDB for the query engine. It has a full UI built with FINOS Perspective.

## Documentation

Check out the **[official documentation](https://joefrost01.github.io/HydroCube/)** for a quick start and usage guides.

## Features
- **Continuous data ingest** From multiple formats (CSV, Parquet, JSON)
- **Fast and Memory-Efficient**: Uses DuckDB for the query engine
- **Full UI**: Uses FINOS Perspective for UI
- **Single Binary**: Runs from a single binary, no dependencies, just point it at your data and run!
- **Written in Rust**: Fast, safe, and concurrent
- **Oauth for Authentication**: Bring the authentication method of your choice
- **Secure**: Uses HTTPS by default
- **Docker Ready**: No dependencies, ideally suited for distroless containers
- **Multi User**: Supports multiple users, ideal for teams that need up the minute data

## Installation

1. **Download the latest release** from [GitHub Releases](https://github.com/joefrost01/HydroCube/releases).
    - Linux: `x86_64-unknown-linux-gnu`
    - macOS: `x86_64-apple-darwin` or `aarch64-apple-darwin`
    - Windows: `x86_64-pc-windows-msvc`
2. **Place the executable** in your `$PATH` (e.g., `/usr/local/bin`) or reference it directly.

### Building from Source

Alternatively, clone this repository and build:

```bash
git clone https://github.com/joefrost01/hydrocube.git
cd hydrocube/frontend
npm install
npm run build
cd ../backend
cargo build --release
```

## Basic Usage

```bash
hydrocube --config config.yaml
```

## Roadmap

Below are planned features and improvements:

1. **Additional data formats**
    - Avro, Orc, Delta Lake, Arrow, and more.

2. **Kafka ingest**
    - Support for ingesting straight from Kafka topics.

3. **Ingest filter**
    - Limit the data ingested by filtering.

4. **Save reports**
    - Serialise your reports for later use.

