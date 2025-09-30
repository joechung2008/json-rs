# Build image
FROM rust:1.90-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY api-actix/ ./api-actix/
COPY api-axum/ ./api-axum/
COPY api-rocket/ ./api-rocket/
COPY cli/ ./cli/
COPY shared-lib/ ./shared-lib/
RUN cargo build --release --package api-rocket

# Runtime stage
FROM debian:trixie-slim
WORKDIR /app
COPY --from=builder /app/target/release/api-rocket /usr/local/bin/api-rocket
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["api-rocket"]
