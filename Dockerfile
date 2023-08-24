FROM rust:1.71.0-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    openssl \
    libssl-dev \
    pkg-config \
    ffmpeg \
    python3 \
    python3-pip

RUN pip3 install --upgrade yt-dlp

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/app/target/release/ryuko_bot_discord"]