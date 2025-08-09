# Codebase Structure - vmix-rs

## Directory Layout
```
vmix-rs/
├── src/
│   ├── lib.rs              # Library root - module declarations
│   ├── vmix.rs             # Main VmixApi struct and TCP connection handling
│   ├── commands.rs         # Command enums and protocol parsing
│   ├── models.rs           # Serde data structures for vMix XML data
│   └── acts.rs             # Activator data parsing for real-time state changes
├── examples/
│   └── cli.rs              # CLI example demonstrating library usage
├── .github/
│   └── workflows/
│       └── claude.yml      # Claude Code GitHub Action integration
├── Cargo.toml              # Project configuration and dependencies
├── CLAUDE.md               # Comprehensive development documentation
├── README.md               # Basic project information
└── LICENSE                 # MIT license
```

## Core Module Architecture

### `src/lib.rs`
- Simple module declarations exposing the public API
- Exports: acts, commands, models, vmix modules

### `src/vmix.rs` - Main API
- **VmixApi struct**: Core connection handler
- **Drop trait**: Graceful cleanup with thread joining
- **Threading**: Separate reader/writer threads with shared shutdown signaling
- **Connection management**: TCP stream handling with timeouts

### `src/commands.rs` - Protocol Layer
- **SendCommand enum**: Commands to send to vMix (TALLY, FUNCTION, XML, etc.)
- **RecvCommand enum**: Responses received from vMix  
- **Protocol parsing**: Robust parsing with error handling
- **Type definitions**: InputNumber (u16, range 0-1000), Status enum, TallyResponse

### `src/models.rs` - Data Models
- **Serde structures**: Complete XML deserialization for vMix state
- **Vmix struct**: Root structure with inputs, overlays, audio, transitions
- **Field mapping**: XML names mapped to Rust snake_case fields

### `src/acts.rs` - Real-time Data
- **ActivatorsData**: Handles vMix activator parsing
- **Real-time updates**: Input switching, audio levels, recording state

### `examples/cli.rs` - Usage Example
- **Complete example**: Shows async usage with tokio
- **Error handling**: Proper Result usage and connection monitoring
- **Multi-threading**: Demonstrates concurrent command sending/receiving

## Communication Pattern
1. **Writer thread**: Receives commands via `SyncSender<SendCommand>`
2. **Reader thread**: Parses TCP data, sends via `Receiver<RecvCommand>`  
3. **Lifecycle management**: Atomic shutdown signals
4. **Error resilience**: Timeout handling without crashes