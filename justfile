build:
    cargo fmt
    cargo build --release
commit message:
    cargo fmt
    git add .
    git commit -m "{{ message }}"
    git push origin main
