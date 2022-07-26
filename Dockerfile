FROM rust:1.62 AS chef

RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:11-slim
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates

COPY --from=builder /app/target/release/main-power-up-bot .
COPY ./config.json .

ENTRYPOINT [ "./main-power-up-bot"]
