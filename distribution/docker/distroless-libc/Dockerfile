FROM debian:bullseye-slim AS builder

COPY mqrt-*.deb ./
RUN dpkg -i mqrt-*-$(dpkg --print-architecture).deb

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/bin/mqrt /usr/bin/mqrt
COPY --from=builder /etc/mqrt /etc/mqrt

# Smoke test
RUN ["mqrt", "--version"]

ENTRYPOINT ["/usr/bin/mqrt"]
