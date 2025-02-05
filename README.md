# The Ground Up

OCI container base image that enables distribution of compressed binaries in the
LMZA2 (`.xz`) format.

## Why

Frequently updated and downloaded containerized microservices consume a lot of
storage space and network bandwidth over time.
Compressing the binary can reduce the resource consumption considerably.

Uncertainty about the [UPX](https://github.com/upx/upx) license motivated
creation of this small project.

## Name

Instead of building containers `FROM scratch` you can now build them
`FROM ghcr.io/mydriatech/the-ground-up:latest`...

## Example usage

Basic usage in multi-stage `Containerfile`:

```text
FROM docker.io/library/rust:alpine as builder
WORKDIR /work
COPY . .
RUN \
    apk add musl-dev xz && \
    cargo build --target=x86_64-unknown-linux-musl --release && \
    # Assuming the bin is named 'my_app'
    xz -k -6 target/x86_64-unknown-linux-musl/release/my_app && \
    mv target/x86_64-unknown-linux-musl/release/my_app.xz app.xz

FROM ghcr.io/mydriatech/the-ground-up:latest
COPY --from=builder --chown=10001:0 /work/app.xz /app.xz
# NOTE: CMD is not required, since the base image will run the app
```

Multi-stage `Containerfile` with custom process name and arguments

```text
FROM docker.io/library/rust:alpine as builder
WORKDIR /work
COPY . .
RUN \
    apk add musl-dev xz && \
    cargo build --target=x86_64-unknown-linux-musl --release && \
    # Assuming the bin is named 'my_app'
    xz -k -6 /work/target/x86_64-unknown-linux-musl/release/my_app && \
    mv target/x86_64-unknown-linux-musl/release/my_app.xz app.xz

FROM ghcr.io/mydriatech/the-ground-up:latest as runner

FROM scratch
USER 10001:0
COPY --from=runner  --chown=10001:0 /the-ground-up /my_app
COPY --from=runner  --chown=10001:0 /app /app
COPY --from=runner  --chown=10001:0 /licenses-the-ground-up.tar.xz /licenses-tgu.tar.xz
COPY --from=builder --chown=10001:0 /work/app.xz /app.xz
CMD ["/my_app", "--argument", "value"]
```

## Caveats

Decompression of the binary increases CPU usage, initialization time and memory
consumption at container startup.
For large binaries (like 100+ MiB development builds), this will be noticeable.

Compressing the binary could potentially be misinterpreted as obfuscation by
malware detection software.
Please report any such real world findings as a regular issue to this project,
since such false positive is not a vulnerability in this software.

## Default app.xz

A small `app.xz` is built into the image to show usage info if you failed to
shadow it during your build.

## License

Standard [MIT](./LICENSE) License.

## Governance model

This projects uses the [Benevolent Dictator Governance Model](http://oss-watch.ac.uk/resources/benevolentdictatorgovernancemodel) (site only seem to support plain HTTP).

See also [Code of Conduct](CODE_OF_CONDUCT.md) and [Contributing](CONTRIBUTING.md).

