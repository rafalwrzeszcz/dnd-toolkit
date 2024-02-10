FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.0

ARG TRUNK_VERSION=0.18.8

# install additional Rust components
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --version ${TRUNK_VERSION}

# install required software from Debian repositories
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libdbus-1-dev \
        libgtk-4-1 \
        libgtk-4-dev \
        libprotobuf-dev \
        protobuf-compiler

# cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*
