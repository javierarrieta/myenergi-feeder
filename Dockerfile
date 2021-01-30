FROM alpine:3.13

COPY target/release/myenergi-feeder /app

ENTRYPOINT ["/app/myenergi-feeder"]