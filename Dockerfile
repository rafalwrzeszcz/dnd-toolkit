FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.1

ARG DIOXUS_VERSION=0.4.3

# install additional Rust components
RUN rustup target add \
    wasm32-unknown-unknown
RUN cargo install dioxus-cli --version ${DIOXUS_VERSION}

# install required software from Debian repositories
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libayatana-appindicator3-dev \
        libdbus-1-dev \
        libgtk-3-dev \
        libprotobuf-dev \
        libwebkit2gtk-4.1-dev \
        protobuf-compiler

# cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*
