# 1: Build
FROM rust:1.62.0 as builder

# 1a: Prepare toolchain
RUN apt update && \
    apt install -y musl-tools musl-dev && \
    rustup target add wasm32-unknown-unknown && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies using fake source code and store as a separate Docker layer
WORKDIR /home/appuser/app

COPY .docker/main.rs src/bin/operator.rs
COPY .docker/lib.rs src/lib.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --target x86_64-unknown-linux-musl --release

# 1c: Build the application using the real source code
COPY src/ src/

RUN touch src/lib.rs
RUN cargo build --target x86_64-unknown-linux-musl --release

# 2: Copy the excutable and extra files to an empty Docker image
FROM scratch

USER 10001:10001

ENV TUNNEL_OPERATOR__HTTP_SERVER__HOST=0.0.0.0
ENV TUNNEL_OPERATOR__HTTP_SERVER__PORT=8080

EXPOSE 8080

COPY --chown=0:0 --from=builder /home/appuser/app/target/x86_64-unknown-linux-musl/release/tunnel-operator tunnel-operator

CMD [ "/tunnel-operator" ]
