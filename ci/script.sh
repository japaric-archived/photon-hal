set -euxo pipefail

main() {
    xargo build --target $TARGET
}

main
