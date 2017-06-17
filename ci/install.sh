set -euxo pipefail

main() {
    cargo install xargo || true
    rustup component add rust-src || true
}

main
