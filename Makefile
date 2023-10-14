default:
	mkdir -p ./build
	cargo build --release
	cp $$(cargo metadata --no-deps 2>/dev/null | jq -r '.target_directory + "/release/" + .packages[0].name') ./build/sql-format
	upx --best --lzma ./build/sql-format

clean:
	rm -rf ./build

distclean:
	make clean
	cargo clean

format:
	cargo +nightly fmt --all

lint:
	cargo clippy --all-features -- -D warnings
