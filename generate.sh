#!/bin/bash

# Generates low level bindings to particle's HAL

set -ex

td=$(mktemp -d)

mk_bindgen() {
    command -v bindgen >/dev/null 2>&1 || \
        cargo install --git https://github.com/crabtw/rust-bindgen
}

fetch_src() {
    curl -L https://github.com/spark/firmware/archive/latest.tar.gz | \
        tar --strip-components 1 -C $td -xz
}

gen_bindings() {
    # TODO Bind to more modules. Currently we just bind to delay and gpio
    local modules=( delay gpio)

    for module in "${modules[@]}"; do
        bindgen $td/hal/inc/${module}_hal.h \
                -I $(arm-none-eabi-gcc -print-search-dirs | grep install | cut -d':' -f2)/include | \
            sed 's/::std::os::raw::/::ty::/g' > src/$module.rs
    done

}

cleanup() {
    rm -r $td
}

main() {
    mk_bindgen
    fetch_src
    gen_bindings
    cleanup
}

main
