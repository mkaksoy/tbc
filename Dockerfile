# Build stage
FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM ubuntu:latest

COPY --from=builder /app/target/release/tbc /usr/local/bin/tbc

CMD ["tbc"]
