FROM rust:1.74.0

WORKDIR /app/src
RUN cargo install cargo-watch

COPY . .
