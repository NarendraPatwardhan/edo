SHELL:=/bin/bash
BINARY_PATH:=target/release/edo

test:
	@cargo test

fmt:
	@cargo fmt

check: fmt
	@cargo clippy

build: fmt
	@cargo build --release

min: build
	@upx ${BINARY_PATH}

run: min
	@time ./${BINARY_PATH}