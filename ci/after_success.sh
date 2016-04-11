#!/bin/bash

set -ex

if [ "$TRAVIS_PULL_REQUEST" = "false" ]; then
    git clone --depth 1 https://github.com/davisp/ghp-import
    ./ghp-import/ghp_import.py -n target/doc
    set +x
    git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
    set -x
fi
