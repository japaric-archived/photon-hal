#!/bin/bash

set -ex

main() {
    local tag=2016-04-11

    # FIXME don't clone the repository, use the one provided by the Travis builder
    docker run japaric/photon:$tag bash -ex -c '
        rustup default nightly
        git clone --depth 1 https://github.com/japaric/particle-hal
        cd particle-hal
        bash generate.sh
        cargo install xargo
        curl -O https://github.com/japaric/photon/raw/master/photon.json
        xargo build --target photon
    '
}

main
