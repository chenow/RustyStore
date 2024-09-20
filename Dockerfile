### Build the binary ###
FROM rust:1.78.0-buster as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release


### Build the runtime image ###
FROM debian:bullseye-slim
WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rusty-store .

EXPOSE 6379
CMD ["./rusty-store"]
