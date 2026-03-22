# kaiseki — APK Analysis Workbench

Complete APK teardown. Consumes all format parser traits from andro-core.

## Build & Test

```bash
cargo build
cargo test
cargo run -- analyze <apk>
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
