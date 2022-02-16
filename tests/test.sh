#!/bin/bash

# SPDX-FileCopyrightText: 2022 localthomas
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

pandoc -o test.json test.md

cd ..
nix build
cd tests

../result/bin/pandoc-drawio pdf < test.json
echo "" # newline after JSON output
echo "Exit code: $($?)"

# actually use pandoc
pandoc --filter ../result/bin/pandoc-drawio -o test.html test.md
#pandoc --filter ../result/bin/pandoc-drawio -o test.pdf test.md
