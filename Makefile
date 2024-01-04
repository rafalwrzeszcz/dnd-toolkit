default: build

clean:
	cargo clean
	find . -name "*.profraw" -exec rm {} \;
	rm -rf coverage.lcov

build:
	cargo build --release

build-dev:
	cargo build

test:
	CARGO_INCREMENTAL=0 \
	RUSTFLAGS="-Cinstrument-coverage" \
	LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw" \
	cargo test --all-features --bins

fix:
	cargo fmt

check:
	cargo fmt --check
	cargo clippy
	cargo udeps

check-local:
	cargo audit

doc:
	cargo doc --no-deps

lcov:
	grcov . \
		--binary-path ./target/debug/deps/ \
		-s . \
		-t lcov \
		--branch \
		--ignore-not-existing \
		--ignore "../*" \
		--ignore "/*" \
		-o coverage.lcov

coverage:
	grcov . \
		--binary-path ./target/debug/deps/ \
		-s . \
		-t html \
		--branch \
		--ignore-not-existing \
		--ignore "../*" \
		--ignore "/*" \
		-o target/coverage
