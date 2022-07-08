FROM rust:1.62 AS rust
RUN rustup target add aarch64-unknown-linux-musl
WORKDIR /tmp/build
COPY . ./
RUN pwd
RUN ls -la
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release -p textvid_api --target aarch64-unknown-linux-musl \
    && cp target/aarch64-unknown-linux-musl/release/textvid_api /tmp

FROM gcr.io/distroless/static
WORKDIR /
COPY --from=rust /tmp/textvid_api .
ENTRYPOINT [ "/textvid_api" ]
