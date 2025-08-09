# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library for interacting with vMix (video mixing software) via TCP and HTTP APIs. The library provides both asynchronous and synchronous communication with vMix instances for real-time video production control.

## Development Commands

### Building and Testing
```bash
cargo build                 # Build the project
cargo build --release      # Build optimized release version
cargo test                 # Run all tests
cargo run --example cli    # Run the CLI example
```

### Development Tools
```bash
cargo check                # Quick compile check without generating binaries
cargo clippy              # Run linter (install: rustup component add clippy)
cargo fmt                 # Format code (install: rustup component add rustfmt)
```

## Architecture Overview

### Core Components

**VmixApi** (`src/vmix.rs`): The main TCP connection handler that manages bidirectional communication with vMix. Uses thread-based architecture with:
- Atomic shutdown signaling for graceful cleanup
- Separate reader and writer threads with proper error handling
- Memory leak prevention through proper Drop implementation
- Connection health monitoring

**Command System** (`src/commands.rs`): Defines the protocol layer with:
- `SendCommand` enum: Commands to send to vMix (TALLY, FUNCTION, XML, etc.)
- `RecvCommand` enum: Responses received from vMix
- Protocol parsing with robust error handling for malformed data
- Stream conversion traits for TCP communication

**Data Models** (`src/models.rs`): Comprehensive Serde-based XML deserialization structures for vMix state, including inputs, overlays, audio settings, and transitions.

**Activators** (`src/acts.rs`): Handles vMix activator data parsing for real-time state changes (input switching, audio levels, recording state, etc.).

### Communication Pattern

The library uses a producer-consumer pattern:
1. Writer thread receives commands via `SyncSender<SendCommand>`
2. Reader thread parses incoming TCP data and sends via `Receiver<RecvCommand>`
3. Connection lifecycle managed through atomic shutdown signals
4. Timeout handling prevents hanging on connection issues

### Error Handling Strategy

- Uses `anyhow::Error` for flexible error propagation
- Connection timeouts are handled gracefully without crashing
- Parser errors for unknown commands are logged but don't terminate connections
- Thread safety ensured through proper synchronization primitives

## Key Implementation Notes

- TCP streams use blocking I/O with timeouts rather than async for thread isolation
- XML parsing handles vMix's specific format quirks (custom boolean enum, optional fields)
- Input numbers are typed as `u16` with range 0-1000 matching vMix limits
- Commands are converted to byte vectors with proper CRLF termination for vMix protocol
- Memory management prevents leaks through explicit Drop implementation and thread joining

## vMix Integration Points

- Default vMix TCP API port: 8099
- Supports subscription model for real-time state updates (TALLY, ACTS)
- XML state queries provide full vMix configuration snapshots
- Function commands allow direct control (CUT, FADE, input switching, etc.)