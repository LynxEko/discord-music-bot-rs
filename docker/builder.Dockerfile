FROM rust:alpine

COPY . .

RUN apk add --no-cache musl-dev pkgconfig openssl openssl-dev openssl-libs-static cmake clang clang-dev make gcc g++ libc-dev linux-headers perl yt-dlp

# RUN wget https://github.com/LynxEko/discord-music-bot-rs/archive/main.tar.gz

# RUN tar -zxf main.tar.gz

# WORKDIR ./discord-music-bot-rs-main

RUN cargo build --release

# RUN cp ./target/release/discord-music-bot-rs ./discord-music-bot-rs

# WORKDIR ../

RUN rm -rf ./discord-music-bot-rs-main


CMD ["discord-music-bot"]
