FROM ubuntu:focal

RUN ["apt", "update"]
RUN ["apt", "install", "-y", "libssl1.1", "ca-certificates"]

COPY target/release/myenergi-feeder /app/myenergi-feeder

ENTRYPOINT ["/app/myenergi-feeder"]
