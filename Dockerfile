FROM ubuntu:focal

RUN ["apt", "update"]
RUN ["apt", "install", "libssl1.1", "ca-certificates"]

COPY target/release/myenergi-feeder /app/myenergi-feeder

ENTRYPOINT ["/app/myenergi-feeder"]
