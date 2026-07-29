FROM rust:1.79 as builder
WORKDIR /app
COPY . .
# We would normally do sqlx prepare here or use offline mode, but for simplicity:
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/my_api /usr/local/bin/
COPY config/ /app/config/
ENV APP_ENVIRONMENT=production
CMD ["my_api"]
