# json-rs

JSON Parser ported from TypeScript to Rust 1.89.0.

## License

MIT

## Reference

[json.org](http://json.org)

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
cargo clippy -p cli -p shared_lib  --all-targets --all-features
```

## Test

```sh
cargo test --workspace
```

## Running the CLI

To build the CLI:

```sh
cargo build --package cli
```

To run the CLI with input from a file:

```sh
cargo run --package cli < input.json
```

Or, to run the CLI and enter JSON manually (press Ctrl+D to end input):

```sh
cargo run --package cli
```
