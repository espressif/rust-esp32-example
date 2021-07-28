# Rust on Xtensa Installation for Linux x64

Tested OS: Ubuntu 18 x64, Ubuntu 20 x64, Mint 20 x64, OpenSUSE Thumbleweed

## Prerequisites

- rustup - installed with nightly toolchain - https://rustup.rs/

## Commands

```
sudo apt install gcc wget xz-utils

rustup toolchain install nightly

mkdir -p ~/.rustup/toolchains/esp

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/rust-1.53.0-dev-x86_64-unknown-linux-gnu.tar.xz
tar xvf rust-1.53.0-dev-x86_64-unknown-linux-gnu.tar.xz
./rust-1.53.0-dev-x86_64-unknown-linux-gnu/install.sh --destdir=~/.rustup/toolchains/esp --prefix="" --without=rust-docs

wget https://dl.espressif.com/dl/idf-rust/dist/noarch/rust-src-1.53.0-dev.tar.xz
tar xvf rust-src-1.53.0-dev.tar.xz
./rust-src-1.53.0-dev/install.sh --destdir=~/.rustup/toolchains/esp --prefix="" --without=rust-docs

rustup default esp

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz
tar xf xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget --continue https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip
unzip main.zip
cd rust-esp32-example-main
idf.py build
```
