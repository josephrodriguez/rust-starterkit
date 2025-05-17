FROM rust:1.87.0 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY proto ./proto
COPY build.rs ./

RUN cargo build --release

FROM debian:buster-slim
ARG CARGO_PROFILE=release

COPY --from=builder /usr/src/app/target/release/your_binary_name /usr/local/bin/your_binary_name

EXPOSE 50051

CMD ["/usr/local/bin/your_binary_name"]
