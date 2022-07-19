FROM rust:1.62 AS builder

WORKDIR /main-power-up-bot
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY ./src ./src
RUN cargo build --release

FROM debian:11-slim

COPY --from=builder /main-power-up-bot/target/release/main-power-up-bot .
COPY ./config.json .

ENTRYPOINT [ "./main-power-up-bot"]
