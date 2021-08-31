# Rust on Xtensa Installation for macOS M1

Following instructions are specific for ESP32 and ESP32-S series based on Xtensa architecture.

Instructions for ESP-C series based on RISC-V architecture are described in document for [ESP32-C3](../README.md#esp32-c3).

Tested OS: macOS Big Sur M1

## Prerequisites

- rustup - https://rustup.rs/

## Installation commands

```sh
rustup toolchain install nightly

VERSION="1.54.0-dev"
ARCH="aarch64-apple-darwin"
RUST_DIST="rust-${VERSION}-${ARCH}"
RUST_SRC_DIST="rust-src-${VERSION}"
TOOLCHAIN_DESTINATION_DIR="~/.rustup/toolchains/esp"

mkdir -p ${TOOLCHAIN_DESTINATION_DIR}

curl "https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/${RUST_DIST}.tar.xz" -o "${RUST_DIST}.tar.xz"
tar xvf ${RUST_DIST}.tar.xz
./${RUST_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

curl "https://dl.espressif.com/dl/idf-rust/dist/noarch/${RUST_SRC_DIST}.tar.xz" -o "${RUST_SRC_DIST}.tar.xz"
tar xvf ${RUST_SRC_DIST}.tar.xz
./${RUST_SRC_DIST}/install.sh --destdir=${TOOLCHAIN_DESTINATION_DIR} --prefix="" --without=rust-docs

rustup default esp

curl "https://dl.espressif.com/dl/idf-rust/dist/${ARCH}/xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz" -o "xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz"
tar xf xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

curl -L https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip -o main.zip
unzip main.zip
cd rust-esp32-example-main
```

## Select architecture for the build

For the ESP32 - default (Xtensa architecture):

```sh
idf.py set-target esp32
```

For the ESP32-S2 (Xtensa architecture):

```sh
idf.py set-target esp32s2
```

For the ESP32-S3 (Xtensa architecture):

```sh
idf.py set-target esp32s3
```

## Build and flash

```sh
idf.py build flash
```
