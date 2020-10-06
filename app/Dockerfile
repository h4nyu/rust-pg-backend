FROM rust:1.46-slim AS dev

RUN rustup component add rustfmt \
    && cargo install cargo-watch

WORKDIR /srv
COPY . .
RUN cargo fetch \
    && cargo install --path .
