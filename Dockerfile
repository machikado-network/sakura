#
# ビルド用ステージ
#
# See: https://hub.docker.com/_/rust
#
FROM rust:1.63-slim-buster as build_env

RUN apt-get update -y && apt-get upgrade -y

WORKDIR sakura

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY src /sakura/src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/sakura/target \
    cargo install --path .

#
# 実行用ステージ
#
FROM debian:buster-slim

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -y update && \
    apt-get -y install systemd && \
    apt-get clean

COPY --from=build_env /usr/local/cargo/bin/sakura /usr/local/sbin/sakura

STOPSIGNAL SIGRTMIN+3
CMD [ "/sbin/init" ]
