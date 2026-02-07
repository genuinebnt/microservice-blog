FROM lukemathwalker/cargo-chef:latest-rust-1.91-alpine AS chef
WORKDIR /app
RUN apk add --no-cache lld clang openssl-dev openssl-libs-static

FROM chef AS planner

COPY Cargo.toml Cargo.lock ./

COPY common/Cargo.toml common/Cargo.toml

COPY posts-service/Cargo.toml posts-service/Cargo.toml
COPY posts-service/migration/Cargo.toml posts-service/migration/Cargo.toml

COPY users-service/Cargo.toml users-service/Cargo.toml
COPY users-service/migration/Cargo.toml users-service/migration/Cargo.toml

COPY gateway-service/Cargo.toml gateway-service/Cargo.toml

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin posts-service \
  && cargo build --release -p posts-migration

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends \
  openssl \
  ca-certificates \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/posts-service ./posts-service
COPY --from=builder /app/target/release/posts-migration ./posts-migration

COPY posts-service/config config
COPY posts-service/entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./entrypoint.sh"]
