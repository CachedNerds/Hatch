#!/bin/bash
set -e

PROJECT_NAME="hatch";

cd $PROJECT_NAME;

travis-cargo build
travis-cargo test
travis-cargo bench