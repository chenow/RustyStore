make:
	cargo build

make run:
	RUST_LOG=info cargo watch -w src/ -x run

make test:
	cargo nextest run
