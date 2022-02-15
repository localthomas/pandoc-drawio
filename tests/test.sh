#!/bin/bash

cargo build && \
#pandoc -o test.json test.md && \
#RUST_BACKTRACE=1 ../target/debug/pandoc-drawio test < test.json
pandoc --filter ../target/debug/pandoc-drawio -o test.html test.md
