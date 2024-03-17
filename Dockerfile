FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.1

ARG CARGO_APK_VERSION=0.10.0
ARG CARGO_MOBILE2_REV=f251416feaca39e831076e9fb84c87d77b15c4ee
ARG DIOXUS_VERSION=0.4.3

# install additional Rust components
RUN rustup target add \
    aarch64-linux-android \
    aarch64-apple-ios \
    aarch64-apple-ios-sim \
    armv7-linux-androideabi \
    i686-linux-android \
    x86_64-apple-ios \
    x86_64-linux-android \
    wasm32-unknown-unknown
RUN cargo install cargo-apk --version ${CARGO_APK_VERSION}
RUN cargo install --git https://github.com/tauri-apps/cargo-mobile2#${CARGO_MOBILE2_REV}
RUN cargo install dioxus-cli --version ${DIOXUS_VERSION}

# install required software from Debian repositories
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libayatana-appindicator3-dev \
        libdbus-1-dev \
        libgtk-3-dev \
        libgtk-4-1 \
        libgtk-4-dev \
        libprotobuf-dev \
        libwebkit2gtk-4.1-dev \
        protobuf-compiler

# cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*
