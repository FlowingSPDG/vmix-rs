# Tech Stack - vmix-rs

## Programming Language
- **Rust** (2021 edition)

## Core Dependencies
- **anyhow** (1.0.69): Flexible error handling and error propagation
- **serde** (1.0.152): Serialization/deserialization framework with derive features
- **serde-xml-rs** (0.6.0): XML parsing and serialization for vMix data structures
- **tokio** (1.25.0): Async runtime with features: rt, net, sync, macros, rt-multi-thread, io-util, io-std, time
- **urlencoding** (2.1.3): URL encoding utilities

## Architecture Patterns
- **Producer-Consumer Pattern**: Using separate reader/writer threads
- **Thread-based Communication**: Using `SyncSender`/`Receiver` for cross-thread messaging
- **Atomic Operations**: For graceful shutdown signaling
- **RAII**: Proper resource management with Drop trait implementation

## Communication Protocols
- **TCP**: Primary protocol for vMix communication
- **XML**: Data format for vMix state and configuration
- **HTTP**: Planned future addition (currently TODO)

## Development Tools Used
- Standard Rust toolchain (rustc, cargo)
- clippy (linter)
- rustfmt (formatter)
- No external testing frameworks (uses built-in Rust testing)