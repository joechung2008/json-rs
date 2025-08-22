# Copilot Instructions for json-rs

## Project Overview

This is a Rust monorepo for a JSON parser ported from TypeScript. It consists of three main crates:

- **api-axum**: HTTP API server exposing JSON parsing via Axum. Provides a POST `/api/v1/parse` endpoint, accepts plain text JSON, and returns pretty-printed output or error JSON.
- **api-rocket**: HTTP API server exposing JSON parsing via Rocket.
- **cli**: Command-line interface for parsing JSON from stdin.
- **shared-lib**: Core library for parsing and representing JSON.

## Workspace Structure

- `api-axum/`: Provides a POST `/api/v1/parse` endpoint. Accepts plain text JSON, returns pretty-printed output or error JSON.
- `api-rocket/`: Provides a POST `/api/v1/parse` endpoint. Accepts plain text JSON, returns pretty-printed output or error JSON.
- `cli/`: Reads JSON from stdin, parses it using shared-lib, and prints a pretty-formatted result or error.
- `shared-lib/`: Contains parsing logic and types. Exposes `parse`, `Json`, `ValueToken`, and `pretty_print_token`.
- `testdata/`: Contains `.rest` files for API testing with VSCode REST Client.

## Build, Test, Format, Lint

- Build all crates: `cargo build --workspace`
- Test all crates: `cargo test --workspace`
- Format: `cargo fmt`
- Lint: `cargo clippy -p api-axum -p api-rocket -p cli -p shared-lib --all-targets --all-features`

## Usage

- **CLI**: `cargo run --package cli < input.json` or enter JSON manually.
- **Axum API**: `cargo run --package api-axum` (default: http://localhost:8080). Test with `.rest` files in `testdata/`.
- **Rocket API**: `cargo run --package api-rocket` (default: http://localhost:8000). Test with `.rest` files in `testdata/`.

## Coding Conventions

- Use types and parsing functions from shared-lib.
- Prefer workspace-level commands for build/test.
- Add new features as modules in shared-lib if they relate to JSON parsing.
- For API changes, update Rocket routes in api-rocket.
- For CLI changes, update main.rs in cli.

## Copilot Agent Guidance

- When processing Copilot requests, follow Rust best practices and workspace conventions.
- For parsing logic, use or extend shared-lib.
- For CLI/API features, ensure integration with shared-lib.
- Add tests in shared-lib/tests or appropriate crate.
- Use `.rest` files in testdata/ for API testing.
- Reference README.md for additional details.
