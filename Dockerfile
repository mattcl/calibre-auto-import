FROM rust:1.92-alpine as builder

RUN apk add musl-dev

WORKDIR /urs/src/calibre-auto-import
COPY . .
RUN cargo install --locked --target-dir /target --path .

FROM alpine:3
COPY --from=builder /usr/local/cargo/bin/calibre-auto-import /usr/local/bin/calibre-auto-import
ENTRYPOINT ["calibre-auto-import"]
