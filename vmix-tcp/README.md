# vmix-tcp

TCP API client for vMix. Provides synchronous, real-time communication with vMix instances.

## Features

- Real-time command/event streaming
- Thread-safe client implementation
- Support for all vMix TCP commands
- Built on top of `vmix-core`

## Usage

```rust
use vmix_tcp::{VmixApi, SendCommand};
use std::time::Duration;

// Connect to vMix
let client = VmixApi::new(
    "127.0.0.1:8099".parse()?,
    Duration::from_secs(5)
)?;

// Send command
client.send_command(SendCommand::TALLY)?;

// Receive response
let response = client.try_receive_command(Duration::from_secs(1))?;
```

## License

MIT
