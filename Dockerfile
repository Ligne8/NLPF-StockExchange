FROM rust:1.82 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release && rm -rf src
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/nlpf-StockExchange /usr/local/bin/nlpf-StockExchange

EXPOSE 5151

CMD ["nlpf-StockExchange"]