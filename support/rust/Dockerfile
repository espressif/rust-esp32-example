FROM ubuntu:bionic
WORKDIR /opt
RUN apt update \
    && apt install -y gcc cmake ccache ninja-build git python3 curl g++ pkg-config libssl-dev

RUN git clone https://github.com/ivmarkov/rust.git \
    && cd rust \
    && git checkout stable \
    && ./configure --experimental-targets=Xtensa  --enable-extended --tools=rustfmt --disable-docs --disable-compiler-docs --set="build.python=python3" \
    && python3 x.py dist --stage 2 || echo "Build failed - silencing error to persist current build artifacts"

# speed-up rebuild: --set="local-rebuild=true"