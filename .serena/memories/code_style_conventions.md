# Code Style and Conventions - vmix-rs

## General Style
- **Language**: Rust 2021 edition
- **Formatting**: Standard rustfmt formatting
- **Naming**: snake_case for variables and functions, PascalCase for types and enums

## Specific Conventions Observed

### Enum Design
- Enums use UPPER_CASE for simple variants (e.g., `OK`, `ER`, `OFF`, `PROGRAM`)
- Complex enums use PascalCase with data (e.g., `Length(u64)`, `Detail(String)`)

### Error Handling
- Uses `anyhow::Result<T>` for flexible error propagation
- Error messages include context (e.g., "Failed to connect to {}: {}")
- Graceful error handling without panicking

### Type Definitions
- Type aliases for domain concepts: `pub type InputNumber = u16; // 0~1000`
- Comments included for important ranges and constraints

### Documentation
- Inline comments for complex logic and constraints
- Function-level documentation where needed
- Architecture documentation in CLAUDE.md

### Serde Usage
- Extensive use of `#[serde(rename = "...")]` for XML field mapping
- Derives `Serialize` and `Deserialize` for data models
- Snake_case Rust fields mapped to camelCase/PascalCase XML fields

### Thread Safety
- Uses `Arc<AtomicBool>` for shared shutdown signaling
- Proper cloning of streams for multi-threaded access
- Channels (`SyncSender`/`Receiver`) for thread communication

### Resource Management
- Custom `Drop` implementation for graceful cleanup
- Explicit thread joining to prevent resource leaks
- Timeout handling for network operations

### Import Organization
- Standard library imports first
- External crate imports second  
- Local module imports last
- Grouped by functionality