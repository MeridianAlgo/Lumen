FROM rust:1.75
WORKDIR /app
COPY . .
RUN cargo build --release --bin lumina-node
CMD ["./target/release/lumina-node"]
