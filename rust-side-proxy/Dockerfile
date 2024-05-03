FROM rust:1.77.0 as builder

WORKDIR /usr/src/rust-side-proxy

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/rust-side-proxy*

COPY . .
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/rust-side-proxy /usr/local/bin/rust-side-proxy

ENV DOCKER_ENV=true
EXPOSE 9091
EXPOSE 9092
EXPOSE 9093
EXPOSE 9094

CMD ["rust-side-proxy"]