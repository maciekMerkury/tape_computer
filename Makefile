default: clear test

.PHONY: test run full_build clean clear fmt

all: fmt test full_build

clear:
	clear

test:
	@cargo test

run: 
	cargo run

full_build:
	cargo build
	cargo build --release

clean:
	cargo clean

fmt:
	cargo fmt

check:
	cargo check

