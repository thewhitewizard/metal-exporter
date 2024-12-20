FROM debian:stable-20241111-slim

RUN apt-get update && apt-get install -y ca-certificates curl
WORKDIR /app
COPY target/aarch64-unknown-linux-gnu/release/metal-exporter /app/
EXPOSE 8080

CMD ["/app/metal-exporter"]