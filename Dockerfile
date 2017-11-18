FROM rustlang/rust:nightly

RUN apt-get update && apt-get install -y \
    binutils-arm-none-eabi \
    cmake \
    gdb-arm-none-eabi \
    openocd

RUN cargo install \
    cargo-add \
    cargo-clone \
    xargo

RUN rustup component add rust-src && \
    rustup target add arm-unknown-linux-gnueabihf
