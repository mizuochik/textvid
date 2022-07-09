FROM rust:1.62 AS rust
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl
WORKDIR /tmp/build
COPY . ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    TARGET=$(uname -m)-unknown-linux-musl \
    && cargo build --release -p textvid-api --target $TARGET \
    && cp target/$TARGET/release/textvid_api /tmp

FROM alpine
WORKDIR /
ADD https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie .
RUN chmod +x aws-lambda-rie
COPY scripts/start_api.sh .
COPY --from=rust /tmp/textvid_api .
ENTRYPOINT [ "/start_api.sh" ]
