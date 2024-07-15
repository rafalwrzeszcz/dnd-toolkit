FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.1

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
