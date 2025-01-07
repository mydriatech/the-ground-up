FROM docker.io/library/rust:alpine as builder
WORKDIR /work
COPY . .
# RUSTFLAGS="-lc" for app is replaced by build.rs output
RUN \
    apk add musl-dev gcc binutils curl xz && \
    cargo update && \
    cargo build --target=x86_64-unknown-linux-musl --release && \
    xz -k -6 /work/target/x86_64-unknown-linux-musl/release/app && \
    touch /work/empty && \
    chmod +x /work/empty && \
    ./bin/extract-third-party-licenses.sh && \
    tar cJf licenses.tar.xz licenses/

FROM scratch

LABEL org.opencontainers.image.source="https://github.com/mydriatech/the-ground-up"
LABEL org.opencontainers.image.description="Runtime extraction of compressed application binary base image."
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.vendor="MydriaTech AB"

COPY --from=builder --chown=10001:0 /work/target/x86_64-unknown-linux-musl/release/the-ground-up /the-ground-up
COPY --from=builder --chown=10001:0 /work/target/x86_64-unknown-linux-musl/release/app.xz /app.xz
COPY --from=builder --chown=10001:0 /work/empty /app
COPY --from=builder --chown=10001:0 --chmod=770 /work/licenses.tar.xz /licenses-the-ground-up.tar.xz

WORKDIR /

USER 10001:0

CMD ["/the-ground-up"]
