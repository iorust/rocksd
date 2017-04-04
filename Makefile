proto:
	protoc --rust_out src proto/*.proto

clean:
	cargo clean

.PHONY: proto