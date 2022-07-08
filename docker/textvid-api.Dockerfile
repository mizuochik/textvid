FROM rust:1.62 AS rust
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl
WORKDIR /tmp/build
COPY . ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    TARGET=$(uname -m)-unknown-linux-musl \
    && cargo build --release -p textvid_api --target $TARGET \
    && cp target/$TARGET/release/textvid_api /tmp

FROM gcr.io/distroless/static
WORKDIR /
COPY --from=rust /tmp/textvid_api .
ENTRYPOINT [ "/textvid_api" ]
