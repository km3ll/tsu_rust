FROM rust:1.85-alpine

RUN apk add musl-dev

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

# Dummy Rust file containing just the main function
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

CMD ./target/release/axum_web_server