FROM alpine:3.15 AS builder

WORKDIR /mqrt

COPY mqrt-*-unknown-linux-musl*.tar.gz ./
RUN tar -xvf mqrt-*-$(cat /etc/apk/arch)-unknown-linux-musl*.tar.gz --strip-components=2

FROM gcr.io/distroless/static

COPY --from=builder /mqrt/bin/* /usr/local/bin/
COPY --from=builder /mqrt/etc/mqrt/* /etc/mqrt/

# Smoke test
RUN ["mqrt", "--version"]

ENTRYPOINT ["/usr/local/bin/mqrt"]
