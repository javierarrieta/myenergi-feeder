# Build Stage
FROM rust:1.49.0 AS builder
WORKDIR /usr/src/
# RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y openssl

#RUN USER=root cargo build --release --verbose --workspace
WORKDIR /usr/src/myenergi-feeder
COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

COPY src ./src
RUN cargo build --release --workspace

# Bundle Stage
FROM ubuntu:20.04
RUN apt update && apt install -y openssl ca-certificates
COPY --from=builder /usr/src/myenergi-feeder/target/release/myenergi-feeder /app/myenergi-feeder
# USER 1000
# COPY target/release/myenergi-feeder /app/myenergi-feeder
# RUN ls -lh /app

CMD ["/app/myenergi-feeder"]
