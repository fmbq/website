# FMBQ Website

[![CI](https://github.com/sagebind/fmbq-website/actions/workflows/ci.yml/badge.svg)](https://github.com/sagebind/fmbq-website/actions/workflows/ci.yml)

New website for the Free Methodist Bible Quizzing program with smart features and integrations built-in. Currently in development.

## Building

The website and backends are written in [Rust](https://www.rust-lang.org/) and requires a Rust toolchain for compilation. Make sure you have Rust installed (recommended way is via [rustup](https://rustup.rs/) as recommended by the Rust organization), then you can simply run the following from the command line:

```sh
cargo build
```

This will also compile the frontend stylesheets for you.

## Running locally

You can run the website on your local machine for testing and development. First, configure how it will run by populating a `.env` file. You can copy the provided `.env.sample` for reasonable defaults for local development:

```sh
cp .env.sample .env
```

Now you can run the server with

```sh
cargo run
```

By default the website will listen at <http://localhost:5000>.
