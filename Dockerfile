FROM debian:11-slim

COPY ./target/release/main-power-up-bot .
COPY ./config.json .

ENTRYPOINT [ "./main-power-up-bot"]
