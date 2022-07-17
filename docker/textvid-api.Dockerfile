FROM rust:1.62 AS rust

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /build

# Cache dependencies
COPY Cargo.lock .
COPY textvid-api/Cargo.toml .
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target $(uname -m)-unknown-linux-musl

COPY . ./
RUN cat Cargo.toml
RUN cargo build --release -p textvid-api --target $(uname -m)-unknown-linux-musl \
    && mv target/$(uname -m)-unknown-linux-musl/release/textvid-api textvid-api.bin

FROM alpine

WORKDIR /app
COPY --from=rust /build/textvid-api.bin textvid-api
ENTRYPOINT [ "/app/textvid-api" ]
