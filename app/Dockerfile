FROM rust:1.82.0-bullseye AS builder

WORKDIR /app
RUN apt update && apt install lld clang --yes
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt update --yes \
    && apt install --yes --no-install-recommends openssl ca-certificates \
    && apt autoremove --yes \
    && apt clean --yes \
    && rm --recursive --force /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
