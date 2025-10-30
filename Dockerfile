############### frontend builder
FROM node:20 as frontend_builder

WORKDIR /build

COPY frontend/package.json frontend/package-lock.json ./

# cache node_modules dependencies
RUN npm install

COPY frontend /build/

RUN npm run build

############### rust builder
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    &&\
    apt-get clean

WORKDIR /build

COPY Cargo.toml Cargo.lock ./

# cache dependencies
RUN cargo fetch

COPY . .

RUN cargo build --release

############### runner container
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && update-ca-certificates && apt-get clean

ENV RUST_LOG=info

WORKDIR /runner

COPY --from=frontend_builder /build/build/ ./static

COPY --from=builder /build/target/release/autowds-backend ./autowds-backend

COPY ./config ./config

COPY ./templates ./templates

EXPOSE 8080

ENTRYPOINT ["/runner/autowds-backend"]
