FROM rust:1.43 as builder

COPY . /opt/repository
WORKDIR /opt/repository

RUN \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.11

COPY --from=builder /opt/repository/target/x86_64-unknown-linux-musl/release/kakapo /opt/bin/kakapo

ENTRYPOINT /opt/bin/kakapo
