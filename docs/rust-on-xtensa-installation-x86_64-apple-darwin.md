# Rust on Xtensa Installation for macOS x64

Tested OS: macOS Big Sur x64

## Commands

```
mkdir -p ~/.rustup/toolchains/xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-apple-darwin/rust-1.50.0-dev-x86_64-apple-darwin.tar.xz
tar xvf rust-1.50.0-dev-x86_64-apple-darwin.tar.xz
pushd rust-1.50.0-dev-x86_64-apple-darwin
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-apple-darwin/rust-src-1.50.0-dev.tar.xz
tar xvf rust-src-1.50.0-dev.tar.xz
pushd rust-src-1.50.0-dev
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

rustup default xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-apple-darwin/xtensa-esp32-elf-llvm11_0_0-x86_64-apple-darwin.tar.xz
tar xf xtensa-esp32-elf-llvm11_0_0-x86_64-apple-darwin.tar.xz
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget https://dl.espressif.com/dl/idf-rust/rust-esp32-example-0.2.tar.gz
tar xvzf rust-esp32-example-0.2.tar.gz
cd rust-esp32-example/rustlib
cargo build --release
cd ..
idf.py build
```

