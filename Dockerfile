FROM node:14.2-buster as client

COPY client /opt/repository
WORKDIR /opt/repository

RUN \
    npm install && \
    npm run test:unit && \
    npm run lint && \
    npm run build

# ---

FROM rust:1.43-buster as server

COPY server /opt/repository
WORKDIR /opt/repository

RUN \
    rustup update nightly && \
    rustup default nightly && \
    rustup component add rustfmt && \
    cargo test && \
    cargo fmt -- --check && \
    cargo build --release

# ---

FROM debian:buster-slim

COPY --from=client /opt/repository/dist /opt/bin/public
COPY --from=server /opt/repository/target/release/server /opt/bin/server
WORKDIR /opt/bin

ENTRYPOINT ROCKET_PORT=${PORT} /opt/bin/server
HEALTHCHECK --interval=1s --timeout=1s --retries=30 \
  CMD curl -f http://0.0.0.0:${PORT} || exit 1
