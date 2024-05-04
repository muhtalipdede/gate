FROM rust:1.77.0 as builder

WORKDIR /usr/src/rust-gate

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/rust-gate*

COPY . .
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/rust-gate /usr/local/bin/rust-gate
COPY ./services.json /usr/local/bin/services.json


ENV DOCKER_ENV=true

EXPOSE 8080

CMD ["rust-gate"]