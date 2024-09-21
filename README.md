
# Rusty Store
![Rust Logo](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Redis Logo](https://img.shields.io/badge/redis-CC0000.svg?&style=for-the-badge&logo=redis&logoColor=white)
![Docker Logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

Rusty Store is a Redis clone implemented in Rust. It leverages the RESP protocol via a TCP server to provide a Redis-compatible key-value store.

## Features
- **Redis-Compatible**: Works with `redis-cli` and Redis client libraries.
- **Asynchronous Multithreading**: Handles multiple clients concurrently.
- **Commands Supported**: `SET`, `GET`, and more.
- **Docker Support**: Available as a Docker image.

## Getting Started

### Prerequisites
- [Docker](https://www.docker.com/get-started) (for containerized usage)
- [Nix](https://nixos.org/) (optional, for local installation with `flake.nix`)

### Installation

#### Docker
To pull and run Rusty Store using Docker, run:
```bash
docker pull chenow/rusty-store
docker run -p 6379:6379 chenow/rusty-store
```

#### Nix (Optional)
For Nix users, you can install Rusty Store locally using the `flake.nix` file:
```bash
nix develop
make run
```

### Running Tests
To ensure everything is working correctly, you can run the provided tests:
```bash
make test
```

## Usage

Once Rusty Store is running, you can interact with it using `redis-cli` or any Redis-compatible client library.

#### Example:
```bash
redis-cli
127.0.0.1:6379> SET mykey myvalue
OK
127.0.0.1:6379> GET mykey
"myvalue"
```
