FROM liuchong/rustup:nightly-musl as build

WORKDIR /

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo test
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN rm src/*.rs

COPY ./src ./src
RUN rm -rf ./target/x86_64-unknown-linux-musl/release/deps/hello*

RUN cargo test
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/hello-rocket /hello-rocket

ENTRYPOINT [ "/hello-rocket" ]