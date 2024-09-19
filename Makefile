make:
	cargo build

make run:
	cargo watch -w src/ -x run

make test:
	cargo nextest run
