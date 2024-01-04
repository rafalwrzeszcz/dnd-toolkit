FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.0

# install required software from Debian repositories
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libdbus-1-dev \
         libprotobuf-dev \
         protobuf-compiler

# cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*
