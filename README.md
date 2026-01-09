# vmix-rs

A Rust library for interacting with vMix via TCP and HTTP APIs.

## Overview

This library provides client implementations for controlling vMix through its TCP and HTTP APIs. It supports real-time command execution, event streaming, and XML state parsing.

## Features

- **TCP API** (default): Synchronous client for real-time command/event communication
- **HTTP API** (optional): Async client for REST-style operations (requires `http` feature)
- Thread-safe client implementations
- XML state parsing with strongly-typed models
- Minimal dependencies for the TCP client

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vmix_rs = "0.1.0"
```

For HTTP API support:

```toml
[dependencies]
vmix_rs = { version = "0.1.0", features = ["http"] }
```

## Usage

See the [documentation](https://docs.rs/vmix_rs) for detailed API usage and examples.

Quick examples are available in the `examples/` directory:

- `cli.rs` - Interactive TCP client example
- `tcp_http_comparison.rs` - TCP and HTTP API comparison (requires `http` feature)
- `http_example.rs` - HTTP client example (requires `http` feature)

## Running Examples

TCP API example:
```bash
cargo run --example cli
```

HTTP API examples:
```bash
cargo run --example http_example --features http
cargo run --example tcp_http_comparison --features http
```

## Testing

Run the test suite:
```bash
cargo test
```

Run tests with HTTP features:
```bash
cargo test --features http
```

Note: Some tests require an actual vMix instance running and will be skipped if not available.

## License

MIT

## Author

Shugo Kawamura ([@FlowingSPDG](https://github.com/FlowingSPDG))

## Contributing

Pull requests are welcome.
