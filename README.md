# json-rs

JSON Parser ported from TypeScript to Rust 1.89.0.

## License

MIT

## Reference

[json.org](http://json.org)

## Prerequisites (Windows)

To ensure `cargo install` works for all dependencies, you must install `mingw-w64` and the required toolchain:

1. Download and install [MSYS2](https://www.msys2.org/).

2. Open the MSYS2 terminal and run:

```sh
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain
```

3. Ensure the MSYS2 `mingw64` environment is in your PATH, or use the `MSYS2 MinGW 64-bit` terminal for building Rust projects.

This will provide `dlltool.exe` and other tools needed for compiling certain crates.

## Build

```sh
cargo build --workspace
```

## Format

```sh
cargo fmt
```

## Lint

```sh
cargo clippy -p api-actix -p api-axum -p api-rocket -p cli -p shared-lib --all-targets --all-features
```

## Test

```sh
cargo test --workspace
```

## Running the CLI

To build the CLI:

```sh
cargo build -p cli
```

To run the CLI with input from a file:

```sh
cargo run -p cli < input.json
```

Or, to run the CLI and enter JSON manually (press Ctrl+D to end input):

```sh
cargo run -p cli
```

## Running the Actix Web API

```sh
cargo run -p api-actix
```

The server will start and listen for HTTP requests (default: http://localhost:8000).

## Running the Axum API

```sh
cargo run -p api-axum
```

The server will start and listen for HTTP requests (default: http://localhost:8080).

## Running the Rocket API & Testing with REST Client

### Run the Rocket API

```sh
cargo run -p api-rocket
```

The server will start and listen for HTTP requests (default: http://localhost:8000).

### Test the API with REST Client

1. Install the [REST Client extension](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) in VSCode.
2. Open any `.rest` file in the `testdata/` directory (e.g., `testdata/all.rest`).
3. Click "Send Request" above a request to send it to the running API server.
4. View the response directly in VSCode.

You can modify or add `.rest` files in `testdata/` to create custom requests for testing.
