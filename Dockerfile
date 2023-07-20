FROM rust:1.71.0-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    openssl \
    libssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["/bin/ryuko_bot_discord"]