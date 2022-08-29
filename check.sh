# Test
cargo test --all -- --nocapture

# Lint
cargo clippy --all-targets --all-features -- -D warnings -A clippy::needless_return

# Format
cargo fmt --all -- --check
