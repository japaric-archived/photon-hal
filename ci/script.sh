#!/bin/bash

set -ex

main() {
    local tag=2016-04-11 container

    docker run japaric/photon:$tag
    container=$(docker ps -a -q)

    docker cp $(pwd) $container:/home/particle/
    docker commit $container travis
    docker rm $container

    docker run travis bash -ex -c '
        cd particle-hal
        bash generate.sh
        cargo build --verbose
        cargo doc
    '
    container=$(docker ps -a -q)
    docker cp $container:/home/particle/$(basename $(pwd))/target .
}

main
