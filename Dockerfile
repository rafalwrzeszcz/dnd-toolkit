FROM rafalwrzeszczwrzasqpl/build-rust:nightly-v0.2.0

# install required software from Debian repositories
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libdbus-1-dev \
        libgtk-4-1 \
        libgtk-4-dev \
        libprotobuf-dev \
        protobuf-compiler \
        qml6-module-qtqml-workerscript \
        qml6-module-qtquick \
        qml6-module-qtquick-controls \
        qml6-module-qtquick-templates \
        qml6-module-qtquick-window \
        qt6-base-dev \
        qt6-declarative-dev

# cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*
