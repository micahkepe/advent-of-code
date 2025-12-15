# Run tests in all crates
test:
    #!/usr/bin/env bash
    for dir in crates/*; do
        if [[ -f "$dir/Cargo.toml" ]]; then
            pushd "$dir" >/dev/null
            cargo test --verbose
            popd >/dev/null
        fi
    done
    # Integration tests
    cargo test --verbose

# Format all Rust files
format:
    cargo fmt
