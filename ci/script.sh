#!/bin/bash

set -ex

main() {
    local tag=2016-04-11

    # The particle user has id = 1000, but this may not match the travis user id. To workaround this
    # issue, make everything world write-able.
    chmod -R a+w .

    docker run -v $(pwd):/mnt -w /mnt japaric/photon:$tag bash -ex -c '
        bash generate.sh
        cargo build --verbose
        cargo doc
    '
}

main
