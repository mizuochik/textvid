FROM rust:1.62 AS rust
WORKDIR /tmp/build
COPY . ./
RUN echo $(uname -m)-unknown-linux-musl > target.txt
RUN rustup target add $(cat target.txt)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release -p textvid_api --target $(cat target.txt) \
    && cp target/$(cat target.txt)/release/textvid_api /tmp

FROM gcr.io/distroless/static
WORKDIR /
COPY --from=rust /tmp/textvid_api .
ENTRYPOINT [ "/textvid_api" ]
