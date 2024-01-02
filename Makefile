default: build

clean:
	cargo clean

build:
	cargo build --release

build-dev:
	cargo build

test:
	cargo tarpaulin --all-features --out Xml --bins

check:
	cargo fmt --check -- --config max_width=120,newline_style=Unix,edition=2021
	cargo clippy
	cargo udeps

check-local:
	cargo audit

doc:
	cargo doc --no-deps
