FROM espressif/idf
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:/opt/rust/bin:/opt/xtensa-esp32-elf-clang/bin:$PATH

ARG VERSION="1.54.0-dev"
ARG ARCH="x86_64-unknown-linux-gnu"
ARG RUST_DIST="rust-${VERSION}-${ARCH}"
ARG RUST_SRC_DIST="rust-src-${VERSION}"
ARG TOOLCHAIN_DESTINATION_DIR="/opt/esp"

RUN curl https://sh.rustup.rs -sSf | bash -s -- --profile minimal --default-toolchain nightly  -y
WORKDIR /opt

#RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/${RUST_DIST}.tar.xz \
RUN wget -q https://github.com/georgik/idf-rust-build/releases/download/v${VERSION}/${RUST_DIST}.tar.xz \
    && tar xvf ${RUST_DIST}.tar.xz \
    && ./${RUST_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs \
    && rm -rf ${RUST_DIST} ${RUST_DIST}.tar.xz

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/noarch/${RUST_SRC_DIST}.tar.xz \
    && tar xvf ${RUST_SRC_DIST}.tar.xz \
    && ./${RUST_SRC_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs \
    && rm -rf ${RUST_SRC_DIST} ${RUST_SRC_DIST}.tar.xz \
    && rustup toolchain link esp /opt/esp \
    && rustup default esp

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz \
    && tar xf xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz \
    && rm xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz

