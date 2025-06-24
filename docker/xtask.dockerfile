FROM rust:1.76-slim-bullseye as builder

WORKDIR /usr/src/app

RUN apt-get update && apt install -y --no-install-recommends \
    ca-certificates \
    curl \
    build-essential \
    protobuf-compiler \
    libclang-dev \
    git \
    pkg-config \
    libssl-dev

COPY . .

RUN cargo build --release --bin xtask

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt install -y --no-install-recommends \
    ca-certificates \
    curl \
    build-essential \
    protobuf-compiler \
    libclang-dev \
    git \
    pkg-config \
    libssl-dev

COPY --from=builder /usr/src/app/target/release/xtask /usr/local/bin

ENTRYPOINT ["xtask"]
