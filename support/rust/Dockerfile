FROM ubuntu:bionic
WORKDIR /opt
RUN apt update \
    && apt install -y gcc cmake ccache ninja-build git python3 curl g++ pkg-config libssl-dev

RUN git clone https://github.com/esp-rs/rust.git \
    && cd rust \
    && ./configure --experimental-targets=Xtensa  --enable-extended --tools=rustfmt \
    && python3 x.py dist --stage 2 || echo "Build failed - silencing error to persist current build artifacts"

