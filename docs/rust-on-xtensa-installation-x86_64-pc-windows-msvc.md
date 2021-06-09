# Rust on Xtensa Installation for Windows x64

Tested OS: Windows 10 x64

## Commands

```
mkdir -p ~/.rustup/toolchains/xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/rust-1.50.0-dev-x86_64-pc-windows-msvc.tar.xz
tar xvf rust-1.50.0-dev-x86_64-pc-windows-msvc.tar.xz
pushd rust-1.50.0-dev-x86_64-pc-windows-msvc
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/rust-src-1.50.0-dev.tar.xz
tar xvf rust-src-1.50.0-dev.tar.xz
pushd rust-src-1.50.0-dev
./install.sh --destdir=~/.rustup/toolchains/xtensa --prefix="" --without=rust-docs
popd

rustup default xtensa

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-win64.zip
unzip xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-win64.zip
export PATH="`pwd`/xtensa-esp32-elf-clang/bin/:$PATH"

wget https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/llvm-patch-0.1.x86_64-pc-windows-msvc.tar.gz
tar xzf llvm-patch-0.1.x86_64-pc-windows-msvc.tar.gz
export PATH="`pwd`/llvm-patch/bin/:$PATH"

wget https://dl.espressif.com/dl/idf-rust/rust-esp32-example-0.2.tar.gz
tar xvzf rust-esp32-example-0.2.tar.gz
cd rust-esp32-example/rustlib
cargo build --release
cd ..
idf.py build
```
