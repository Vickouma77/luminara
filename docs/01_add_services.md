# Adding Services to Luminara:

## 1. Create Workspace root
```bash
mkdir luminara && cd luminara
echo '[workspace]\nmembers = ["services/*", "crates/*"]\nresolver = "2"' > Cargo.toml
```

## 2. Create a New Service Directory
```bash
cargo new services/<service_name> --name <service_name> --bin --vcs none  # Create a new service
```

## 3. Create a Shared Library
```bash
cargo new shared/<shared_name>/shared --lib --vcs none  # Create a shared library
```