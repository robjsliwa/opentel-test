FROM rust:1.48.0 as cargo-build

WORKDIR /var/opt/app

RUN rustup component add rustfmt --toolchain 1.48.0-x86_64-unknown-linux-gnu

COPY Cargo.toml Cargo.toml

# COPY Cargo.lock Cargo.lock

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --bin nameserver

RUN rm -f target/debug/deps/nameserver*

COPY . .

RUN cargo build --bin nameserver

RUN mkdir -p /build-out

RUN cp target/debug/nameserver /build-out/

# ------------------------------------------------------------------------------

FROM ubuntu:18.04

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get -y install libcurl4-openssl-dev

COPY --from=cargo-build /build-out/nameserver /usr/local/bin

CMD /usr/local/bin/nameserver
