# Cargo and Workspace in Luminara
---

## 1. Overview
Cargo rust package manager and build tool. In Luminara, we use:
- ```Workspace``` to group multiple microservices and shared libraries.
- ```Crates``` as the unit of code distribution(services, shared libs, tools)
- ```Dependency resolution``` via the new ```resolver = "2"```for clean builds

---

## 2. Workspace Layout
### (Workspace level Cargo.toml)
```toml
[workspace]
resolver = "2"
members = [
    "services/*",
    "shared/*"
]
```
- members → Relative paths to each crate (service or library)
- resolver → Controls dependency resolution rules

### (Binary crate Cargo.toml)

```toml
[package]
name = "example-service"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web.workspace = true
serde.workspace = true
tokio.workspace = true
uuid = "1"  # service-specific dependency
```
- .workspace = true → Pulls dependency version from [workspace.dependencies]
- Service-specific dependencies are listed normally

### (Library crate Cargo.toml)
```toml
[package]
name = "db"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio.workspace = true
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
```

---
### (Workspace layout)
```bash
luminara
├── Cargo.toml
├── services
│   ├── service1
│   │   └── Cargo.toml
│   └── service2
│       └── Cargo.toml
└── shared
    └── shared_lib
        └── Cargo.toml
```

---

## Common Commands

```bash
# Build the entire workspace
cargo build

# Build a specific service
cargo build -p example-service

# Run a specific service
cargo run -p example-service

# Add dependencies to a specific service
cargo add example-dependency@4 --package example-service

# Add dependencies to all services
cargo add serde@1 --features derive --workspace

# Test a specific service
cargo test -p example-service

# Check formatting
cargo fmt

# Check for unused dependencies
cargo unused

# Update dependencies
cargo update
```