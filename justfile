build:
    cargo fmt
    cargo build --release
test:
    cargo fmt
    cargo test
    cargo clippy -- -D warnings
commit message: test
    git add .
    git commit -m "{{ message }}"
    git push origin main
