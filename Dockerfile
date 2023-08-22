FROM rust:1.71.1-alpine AS builder

WORKDIR /build

COPY . /build/

RUN cargo build --release


### Runtime Container
FROM alpine:latest

WORKDIR /app

COPY --from=builder /build/target/release/autowds-backend /app/instance

RUN ["/app/instance"]