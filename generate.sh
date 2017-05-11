set -ex

main() {
    local version=v0.6.2

    # TODO add the other modules
    local modules=(
        delay
        gpio
    )

    local td=$(mktemp -d)
    curl -L https://github.com/spark/firmware/archive/$version.tar.gz | \
        tar --strip-components=1 -xz -C $td

    for module in ${modules[@]}; do
        # this no longer seems to be necessary?
        # -- -I $(arm-none-eabi-gcc -print-search-dirs | grep install | cut -d':' -f2)/include \

        bindgen --use-core \
                --ctypes-prefix ::ctypes \
                $td/hal/inc/${module}_hal.h \
                > src/$module.rs
    done

    rm -rf $td
}

main
