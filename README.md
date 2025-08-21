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

### Basic Rust example

Basic usage in multi-stage `Containerfile`:

```text
FROM docker.io/library/rust:alpine as builder
WORKDIR /work
COPY . .
RUN \
    apk add musl-dev xz && \
    cargo build --target=x86_64-unknown-linux-musl --release && \
    # Assuming the bin is named 'my-app'
    xz -k -6 target/x86_64-unknown-linux-musl/release/my-app && \
    mv target/x86_64-unknown-linux-musl/release/my-app.xz my-app.xz

FROM ghcr.io/mydriatech/the-ground-up:latest
COPY --from=builder --chown=10001:0 /work/my-app.xz /the-ground-up.xz
# CMD is not required here, since the base image will run the app
```


### Custom Rust example

Multi-stage `Containerfile` with custom process name and arguments:

```text
FROM docker.io/library/rust:alpine as builder
WORKDIR /work
COPY . .
RUN \
    apk add musl-dev xz && \
    cargo build --target=x86_64-unknown-linux-musl --release && \
    # Assuming the bin is named 'my-app'
    xz -k -6 /work/target/x86_64-unknown-linux-musl/release/my-app && \
    mv target/x86_64-unknown-linux-musl/release/my-app.xz my-app.xz

FROM ghcr.io/mydriatech/the-ground-up:latest as tgu

FROM scratch
USER 10001:0
COPY --from=tgu     --chown=10001:0 /the-ground-up       /my-app
COPY --from=tgu     --chown=10001:0 /the-ground-up-bin   /my-app-bin
COPY --from=tgu     --chown=10001:0 /licenses-tgu.tar.xz /licenses-tgu.tar.xz
COPY --from=builder --chown=10001:0 /work/my-app.xz /my-app.xz
CMD ["/my-app", "--argument", "value"]
```

### Quarkus example

[Quarkus](https://quarkus.io/) native builds can also be compressed:

```
FROM docker.io/redhat/ubi9-minimal:latest as builder
WORKDIR /work
# This assumes that the native image has already been built.
COPY --chown=10001:0 target/*-runner /work/application
RUN \
    microdnf -y install xz && \
    xz -k -6 /work/application

FROM ghcr.io/mydriatech/the-ground-up:1.0.0 as tgu

FROM quay.io/quarkus/ubi9-quarkus-micro-image:2.0
WORKDIR /work/
COPY --from=tgu     --chown=10001:0 /licenses-tgu.tar.xz /licenses-tgu.tar.xz
COPY --from=tgu     --chown=10001:0 /the-ground-up       /work/application
COPY --from=tgu     --chown=10001:0 /the-ground-up-bin   /work/application-bin
COPY --from=builder --chown=10001:0 /work/application.xz /work/application.xz
USER 10001:0
EXPOSE 8080
CMD ["./application", "-Dquarkus.http.host=0.0.0.0"]
```


## Caveats

Decompression of the binary increases CPU usage, initialization time and memory
consumption at container startup.
For large binaries (like 100+ MiB development builds), this will be noticeable.

Compressing the binary could potentially be misinterpreted as obfuscation by
malware detection software.
Please report any such real world findings as a regular issue to this project,
since such false positive is not a vulnerability in this software.


## Default the-ground-up.xz

A small `the-ground-up.xz` is built into the image to show usage info if you failed to
shadow it during your build.

## License

Standard [MIT](./LICENSE) License.

## Governance model

This projects uses the [Benevolent Dictator Governance Model](http://oss-watch.ac.uk/resources/benevolentdictatorgovernancemodel) (site only seem to support plain HTTP).

See also [Code of Conduct](CODE_OF_CONDUCT.md) and [Contributing](CONTRIBUTING.md).

