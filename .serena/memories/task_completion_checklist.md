# Task Completion Checklist - vmix-rs

## Before Completing Any Development Task

### 1. Code Quality Checks
Run these commands in order:
```bash
cargo check                 # Verify code compiles without errors
cargo clippy                # Check for linting issues and warnings  
cargo fmt                   # Ensure consistent code formatting
```

### 2. Testing
```bash
cargo test                  # Run all unit and integration tests
```

### 3. Build Verification
```bash
cargo build                 # Ensure debug build succeeds
cargo build --release       # Verify release build works
```

### 4. Example Verification (if applicable)
```bash
cargo run --example cli     # Test the CLI example still works
```

### 5. Documentation Check
- Ensure any new public APIs have proper documentation
- Update CLAUDE.md if architecture changes were made
- Verify all TODOs are still relevant

## When Making Changes to Core Components

### VmixApi Changes (`src/vmix.rs`)
- Verify thread safety and memory management
- Test connection handling and graceful shutdown
- Ensure Drop implementation still works correctly

### Protocol Changes (`src/commands.rs`)  
- Test parsing with malformed input
- Verify enum conversions work bidirectionally
- Check error handling for unknown commands

### Data Model Changes (`src/models.rs`)
- Verify XML parsing still works with vMix data
- Test serialization/deserialization roundtrips
- Check field mappings are correct

## Git Best Practices
- Create descriptive commit messages
- Keep commits focused and atomic
- Reference issue numbers where applicable
- Don't commit Cargo.lock (library project)

## Additional Checks
- No unwrap() calls in production code (use proper error handling)
- All thread communication uses channels appropriately
- Timeout values are reasonable for production use
- Memory leaks prevented through proper Drop implementations