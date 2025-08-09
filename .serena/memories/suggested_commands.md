# Suggested Development Commands - vmix-rs

## Essential Development Commands

### Building and Compilation
```bash
cargo build                 # Build the project in debug mode
cargo build --release       # Build optimized release version
cargo check                 # Quick compile check without generating binaries
```

### Testing
```bash
cargo test                  # Run all unit and integration tests
```

### Code Quality and Formatting
```bash
cargo clippy                # Run linter (install: rustup component add clippy)
cargo fmt                   # Format code (install: rustfmt component add rustfmt)
```

### Running Examples
```bash
cargo run --example cli     # Run the CLI example application
```

### Other Useful Cargo Commands
```bash
cargo clean                 # Clean build artifacts
cargo doc                   # Generate documentation
cargo tree                  # Display dependency tree
```

## System Commands (Linux WSL2)

### File Operations
```bash
ls                          # List directory contents
find . -name "*.rs"         # Find Rust files
grep -r "pattern" src/      # Search for patterns in source
```

### Git Operations
```bash
git status                  # Check repository status
git add .                   # Stage all changes
git commit -m "message"     # Commit changes
git push                    # Push to remote repository
```

## IDE Integration
The project includes GitHub Actions workflow for Claude Code integration, triggered by "@claude" mentions in issues and PRs.

## Notes
- No specific test framework is configured - uses Rust's built-in testing
- The project follows standard Rust conventions and Cargo workflows
- Development on Linux WSL2 environment