#!/usr/bin/env bash

cargo build --release
cp target/release/qit ~/bin
