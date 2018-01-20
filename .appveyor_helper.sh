#!/bin/bash
set -e

PROJECT_NAME="hatch";

cd $PROJECT_NAME;

cargo build --verbose --all
cargo test --verbose --all