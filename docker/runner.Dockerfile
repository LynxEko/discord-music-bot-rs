FROM alpine:latest


RUN apk add --no-cache pkgconfig openssl openssl-libs-static cmake clang make gcc g++ linux-headers perl yt-dlp

CMD ["discord-music-bot"]
