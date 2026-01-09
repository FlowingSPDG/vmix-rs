# vmix-http

HTTP API client for vMix. Provides async, request-response style communication with vMix instances.

## Features

- Async/await interface via `tokio`
- REST-style API access
- Built on top of `vmix-core`
- Helper methods for common operations

## Usage

```rust
use vmix_http::HttpVmixClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to vMix
    let client = HttpVmixClient::new(
        "127.0.0.1:8088".parse()?,
        Duration::from_secs(5)
    );

    // Get XML state
    let state = client.get_xml_state().await?;
    println!("vMix version: {}", state.version);

    // Execute function
    client.cut().await?;

    Ok(())
}
```

## License

MIT
