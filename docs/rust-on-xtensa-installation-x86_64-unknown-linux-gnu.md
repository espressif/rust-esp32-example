# Rust on Xtensa Installation for Linux x64

Tested OS: Ubuntu 20 x64, Mint x64

Limitation: Does not work with Ubuntu 18 Bionic - GLIBC problem in cargo: `/lib/x86_64-linux-gnu/libm.so.6: version `GLIBC_2.29' not found.`

## Prerequisites

- rustup - installed with nightly toolchain - https://rustup.rs/

## Commands

```
sudo apt install gcc wget xz-utils

rustup toolchain install nightly

mkdir -p ~/.rustup/toolchains/xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/rust-1.50.0-dev-x86_64-unknown-linux-gnu.tar.xz
tar xvf rust-1.50.0-dev-x86_64-unknown-linux-gnu.tar.xz
pushd rust-1.50.0-dev-x86_64-unknown-linux-gnu
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/rust-src-1.50.0-dev.tar.xz
tar xvf rust-src-1.50.0-dev.tar.xz
pushd rust-src-1.50.0-dev
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

rustup default xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-linux-amd64.tar.xz
tar xf xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-linux-amd64.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/llvm-patch-0.1.x86_64-unknown-linux-gnu.tar.gz
tar xzf llvm-patch-0.1.x86_64-unknown-linux-gnu.tar.gz
export PATH="`pwd`/llvm-patch/bin/:$PATH"

wget --continue https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip
unzip main.zip
cd rust-esp32-example-main
idf.py build
```
