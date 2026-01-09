# vmix-core

Core data structures and XML parsing for vMix API. This crate is `no_std` compatible and can be used in embedded environments and WebAssembly.

## Features

- `no_std` compatible (requires `alloc`)
- Strongly-typed XML data structures for vMix state
- XML parsing and serialization via `quick-xml`
- Zero network dependencies

## Usage

```rust
use vmix_core::{Vmix, from_str};

// Parse XML from vMix
let xml = r#"<vmix><version>1.0</version>...</vmix>"#;
let vmix_state: Vmix = from_str(xml)?;

println!("vMix version: {}", vmix_state.version);
println!("Active input: {}", vmix_state.active);
```

## Features

- `std` (optional): Enable standard library support

## License

MIT
