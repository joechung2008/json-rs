# Build stage
FROM rust:1.90-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY api-actix/ ./api-actix/
COPY api-axum/ ./api-axum/
COPY api-rocket/ ./api-rocket/
COPY cli/ ./cli/
COPY shared-lib/ ./shared-lib/
RUN cargo build --release --package api-actix

# Runtime stage
FROM debian:trixie-slim
WORKDIR /app
COPY --from=builder /app/target/release/api-actix /usr/local/bin/api-actix
EXPOSE 8000 8080
CMD ["api-actix"]
