make:
	cargo build

make run:
	RUST_LOG=info cargo watch -w src/ -x run

make test:
	cargo nextest run

make deploy TAG:
	docker build -t chenow/rusty-store:$(TAG) .
	docker push chenow/rusty-store:$(TAG)
