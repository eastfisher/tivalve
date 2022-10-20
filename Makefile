.PHONY: build clean

NAME=tivalve
TARGET=wasm32-wasi

build: build/$(NAME).wasm

build/$(NAME).wasm: target/$(TARGET)/release/$(NAME).wasm
	mkdir -p build && cp $< $@

target/$(TARGET)/release/$(NAME).wasm:
	cargo build --target $(TARGET) --release

clean:
	cargo clean
	rm -Rf build

build-on-mac:
	OPENSSL_DIR=/usr/local/opt/openssl@1.1/ cargo build --target wasm32-wasi
