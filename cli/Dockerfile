# Build stage
FROM rust:1.90-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY api-actix/ ./api-actix/
COPY api-axum/ ./api-axum/
COPY api-rocket/ ./api-rocket/
COPY cli/ ./cli/
COPY shared-lib/ ./shared-lib/
RUN cargo build --release --package cli --target x86_64-unknown-linux-musl

# Runtime stage
FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cli /cli
CMD ["/cli"]
