FROM rust:1.62.1 as builder

RUN apt-get update -y && apt-get upgrade -y

WORKDIR sakura

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY src /sakura/src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/sakura/target \
    cargo install --path .

CMD ["/usr/local/cargo/bin/sakura", "tinc", "setup", "syamimomo", "10.50.2.1"]
