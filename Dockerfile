FROM rust:latest AS builder

WORKDIR /app

RUN rustup show && rustup default stable

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY src ./src

RUN cargo build --release


FROM ubuntu:22.04

WORKDIR /app

COPY --from=builder /app/target/release/BUProjectBackend .

EXPOSE 3000

CMD ["/app/BUProjectBackend"]
