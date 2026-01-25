FROM rust:1.76-alpine

RUN apk add musl-dev

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

# Dummy Rust file containing just the main function
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

CMD ./target/release/askama_todos