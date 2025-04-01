# Stage 1: Build the Rust application
FROM rust:1.85.1-slim AS builder
ARG CARGO_PROFILE=release

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
COPY .cargo ./.cargo
COPY assets ./assets

RUN cargo build --profile=$CARGO_PROFILE

# Stage 2: Copy the binary to a minimal Debian image
FROM debian:stable-slim
LABEL authors="@josephrodriguez"

ARG CARGO_PROFILE=release

RUN useradd -m rust

WORKDIR /home/rust
COPY --from=builder /app/target/$CARGO_PROFILE/actixweb-rest-api-service .
COPY --from=builder /app/assets ../assets

RUN chmod +x actixweb-rest-api-service

EXPOSE 3000
USER rust

ENTRYPOINT ["./actixweb-rest-api-service"]