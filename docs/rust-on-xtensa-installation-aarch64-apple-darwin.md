# Rust on Xtensa Installation for macOS M1

Tested OS: macOS Big Sur M1

## Prerequisites

- rustup - https://rustup.rs/

## Commands

```
rustup toolchain install nightly

mkdir -p ~/.rustup/toolchains/esp

wget --continue https://dl.espressif.com/dl/idf-rust/dist/aarch64-apple-darwin/rust-1.53.0-dev-aarch64-apple-darwin.tar.xz
tar xvf rust-1.53.0-dev-aarch64-apple-darwin.tar.xz
./rust-1.53.0-dev-aarch64-apple-darwin/install.sh --destdir=~/.rustup/toolchains/esp --prefix="" --without=rust-docs

wget --continue https://dl.espressif.com/dl/idf-rust/dist/noarch/rust-src-1.53.0-dev.tar.xz
tar xvf rust-src-1.53.0-dev.tar.xz
./rust-src-1.53.0-dev/install.sh --destdir=~/.rustup/toolchains/esp --prefix=""

rustup default esp

wget --continue https://dl.espressif.com/dl/idf-rust/dist/aarch64-apple-darwin/xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz
tar xf xtensa-esp32-elf-llvm11_0_0-aarch64-apple-darwin.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget --continue https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip
unzip main.zip
cd rust-esp32-example-main
idf.py build
```

