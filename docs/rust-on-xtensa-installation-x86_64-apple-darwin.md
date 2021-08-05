# Rust on Xtensa Installation for macOS x64

Tested OS: macOS Big Sur x64

## Prerequisites

- rustup - installed with nightly toolchain - https://rustup.rs/

## Commands

```
rustup toolchain install nightly

VERSION="1.54.0-dev"
ARCH="x86_64-apple-darwin"
RUST_DIST="rust-${VERSION}-${ARCH}"
RUST_SRC_DIST="rust-src-${VERSION}"
TOOLCHAIN_DESTINATION_DIR="~/.rustup/toolchains/esp"

mkdir -p ${TOOLCHAIN_DESTINATION_DIR}

wget "https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/${RUST_DIST}.tar.xz"
tar xvf ${RUST_DIST}.tar.xz
./${RUST_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

wget "https://dl.espressif.com/dl/idf-rust/dist/noarch/${RUST_SRC_DIST}.tar.xz"
tar xvf ${RUST_SRC_DIST}.tar.xz
./${RUST_SRC_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

rustup default esp

wget "https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/xtensa-esp32-elf-llvm11_0_0-x86_64-apple-darwin.tar.xz"
tar xf xtensa-esp32-elf-llvm11_0_0-x86_64-apple-darwin.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget --continue https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip
unzip main.zip
cd rust-esp32-example-main
idf.py build
```

