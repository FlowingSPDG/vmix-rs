# vmix-rs

A Rust library for interacting with vMix via TCP and HTTP APIs.

[![Crates.io](https://img.shields.io/crates/v/vmix_rs.svg)](https://crates.io/crates/vmix_rs)
[![Documentation](https://docs.rs/vmix_rs/badge.svg)](https://docs.rs/vmix_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

This library is organized into separate crates for different use cases:

- **vmix-core**: Core data structures and XML parsing (`no_std` compatible)
- **vmix-tcp**: TCP API client
- **vmix-http**: HTTP API client (async)
- **vmix-rs**: Convenience wrapper (this crate)

## Installation

```toml
[dependencies]
# For desktop applications with both TCP and HTTP support
vmix_rs = { version = "0.1.0", features = ["full"] }

# TCP only
vmix_rs = { version = "0.1.0", features = ["tcp"] }

# HTTP only
vmix_rs = { version = "0.1.0", features = ["http"] }

# For embedded/WebAssembly (no_std)
vmix-core = "0.1.0"
```

## Usage

### Desktop Applications

Use `vmix-rs` with TCP/HTTP features:

```rust
use vmix_rs::{VmixApi, HttpVmixClient};
```

### Embedded/WebAssembly (no_std)

Use `vmix-core` directly for XML parsing only. You'll need to handle network communication yourself:

```rust
#![no_std]
extern crate alloc;

use vmix_core::{Vmix, from_str};

// Parse XML data (obtained from your own HTTP/TCP client)
let vmix_state: Vmix = from_str(xml_string)?;
```

## Examples

```bash
# TCP client
cargo run --example cli --features tcp

# HTTP client
cargo run --example http_example --features http

# TCP/HTTP comparison
cargo run --example tcp_http_comparison --features full
```

## License

MIT

## Author

Shugo Kawamura ([@FlowingSPDG](https://github.com/FlowingSPDG))
