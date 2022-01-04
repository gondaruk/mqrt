FROM debian:bullseye-slim AS builder

COPY mqrt-*.deb ./
RUN dpkg -i mqrt-*-$(dpkg --print-architecture).deb

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata systemd && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/bin/mqrt /usr/bin/mqrt
COPY --from=builder /etc/mqrt /etc/mqrt

# Smoke test
RUN ["mqrt", "--version"]

ENTRYPOINT ["/usr/bin/mqrt"]
