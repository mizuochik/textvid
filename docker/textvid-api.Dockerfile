FROM rust:1.62 AS rust

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /build
COPY . ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    TARGET=$(uname -m)-unknown-linux-musl \
    && cargo build --release -p textvid-api --target $TARGET \
    && mv target/$TARGET/release/textvid-api /tmp

FROM public.ecr.aws/lambda/provided:al2

WORKDIR /app
COPY scripts/start_api.sh .
COPY --from=rust /tmp/textvid-api .
ENTRYPOINT [ "/app/start_api.sh" ]
