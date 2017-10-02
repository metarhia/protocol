#!/bin/bash

set -e
set -u

if [ "$TRAVIS_OS_NAME" == "linux" ]; then
  cargo install clippy --force

  cd crates/mhp
  cargo clippy

  cd ../mhp-node
  cargo clippy

  cd ../mhp-napi
  cargo clippy

  cd ../..
fi
