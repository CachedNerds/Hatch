#!/bin/bash
set -e

PROJECT_NAME="hatch";

cd $PROJECT_NAME;


pip install 'travis-cargo<0.2' --user && export PATH=$HOME/.local/bin:$PATH

travis-cargo build
travis-cargo test
travis-cargo bench

travis-cargo coveralls --no-sudo --verify