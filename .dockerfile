FROM rust:1.67 as builder
WORKDIR /usr/src/run_app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/run_app/target/release/backend ./backend
COPY --from=builder /usr/src/run_app/.env ./.env
CMD ["./backend"]