SHELL:=/bin/bash

.DEFAULT_GOAL := default

RFLAGS=--release

format:
	cargo fmt

clippy:
	cargo clippy

build: format clippy
	cargo build $(RFLAGS)

wasm:
	wasm-pack build --target no-modules

test: build
	cargo test $(RFLAGS)

run: build
	i=1 ; while [[ $$i -le 25 ]] ; do \
		if [ -f "./src/bin/$$i.rs" ]; then \
			cargo run $(RFLAGS) --bin $$i ; \
		fi ; \
		((i = i + 1)) ; \
	done

default: run

clean:
	cargo clean