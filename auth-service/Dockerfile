FROM lukemathwalker/cargo-chef:latest-rust-1.91-alpine AS chef
WORKDIR /app
RUN apk add --no-cache lld clang openssl-dev openssl-libs-static

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin auth-service

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/auth-service auth-service
COPY auth-service/config config
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./auth-service"]
