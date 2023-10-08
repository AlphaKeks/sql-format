#!/bin/sh

cargo build --release
cp `cargo metadata --no-deps 2>/dev/null | jq -r '.target_directory + "/release/" + .packages[0].name'` ./sql-format
upx --best --lzma ./sql-format
