FROM rust:1.85 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin lumina-node

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/lumina-node /usr/local/bin/lumina-node

EXPOSE 3000 4000

ENTRYPOINT ["lumina-node"]
CMD ["--validator", "--data-dir", "/data"]
