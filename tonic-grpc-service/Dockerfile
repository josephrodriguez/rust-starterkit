FROM rust:1.87.0-slim as builder
ARG CARGO_PROFILE=release

WORKDIR /app
RUN apt-get update && apt-get install protobuf-compiler -y --no-install-recommends

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY proto ./proto
COPY build.rs ./

RUN cargo build --profile=$CARGO_PROFILE

FROM debian:stable-slim
ARG CARGO_PROFILE=release

WORKDIR /app
COPY --from=builder /app/target/$CARGO_PROFILE/tonic-grpc-service .

EXPOSE 50051
ENTRYPOINT ["./tonic-grpc-service"]
