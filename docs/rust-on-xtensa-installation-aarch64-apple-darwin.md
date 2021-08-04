# Rust on Xtensa Installation for macOS M1

Tested OS: macOS Big Sur M1

## Prerequisites

- rustup - https://rustup.rs/

## Commands

```
rustup toolchain install nightly

VERSION="1.54.0-dev"
ARCH="aarch64-apple-darwin"
RUST_DIST="rust-${VERSION}-${ARCH}"
RUST_SRC_DIST="rust-src-${VERSION}"
TOOLCHAIN_DESTINATION_DIR="~/.rustup/toolchains/esp"

mkdir -p ${TOOLCHAIN}

wget https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/${RUST_DIST}.tar.xz
tar xvf ${RUST_DIST}.tar.xz
./${RUST_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

wget https://dl.espressif.com/dl/idf-rust/dist/noarch/${RUST_SRC_DIST}.tar.xz
tar xvf ${RUST_SRC_DIST}.tar.xz
./${RUST_SRC_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

rustup default esp

wget --continue https://dl.espressif.com/dl/idf-rust/dist/aarch64-apple-darwin/xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz
tar xf xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget --continue https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip
unzip main.zip
cd rust-esp32-example-main
idf.py build
```

