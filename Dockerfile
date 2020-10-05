FROM rust:1

WORKDIR /srv
COPY . .
RUN cargo install --path .
CMD ["app"]
