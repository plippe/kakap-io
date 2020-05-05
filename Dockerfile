FROM rust:1.43-buster as builder

COPY . /opt/repository
WORKDIR /opt/repository

RUN \
    rustup update nightly && \
    rustup default nightly && \
    cargo build --release

# ---

FROM debian:buster-slim

COPY --from=builder /opt/repository/target/release/kakapo /opt/bin/kakapo
COPY --from=builder /opt/repository/public /opt/bin/public
WORKDIR /opt/bin

ENTRYPOINT ROCKET_PORT=${PORT} /opt/bin/kakapo
HEALTHCHECK --interval=1s --timeout=1s --retries=30 \
  CMD curl -f http://0.0.0.0:${PORT} || exit 1
