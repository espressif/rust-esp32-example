# Rust on Xtensa Installation for Windows x64

Tested OS: Windows 10 x64

## Prerequisites

- Visual Studio - installed with option Desktop development with C++
- rustup - https://rustup.rs/
- Chocolatey - https://chocolatey.org/

## Commands for PowerShell

```
choco install 7zip

mkdir -p ~/.rustup/toolchains/xtensa

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/rust-1.50.0-dev-x86_64-pc-windows-msvc.tar.xz -OutFile rust-1.50.0-dev-x86_64-pc-windows-msvc.tar.xz
7z e .\rust-1.50.0-dev-x86_64-pc-windows-msvc.tar.xz
7z x .\rust-1.50.0-dev-x86_64-pc-windows-msvc.tar
pushd rust-1.50.0-dev-x86_64-pc-windows-msvc
cp -Recurse .\rustc\bin ~\.rustup\toolchains\xtensa\
cp -Recurse .\rustc\lib ~\.rustup\toolchains\xtensa\
cp -Recurse .\rustc\share ~\.rustup\toolchains\xtensa\
cp -Recurse .\rust-std-x86_64-pc-windows-msvc\lib\* ~\.rustup\toolchains\xtensa\lib\
popd

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/rust-src-1.50.0-dev.tar.xz -OutFile rust-src-1.50.0-dev.tar.xz
7z e .\rust-src-1.50.0-dev.tar.xz
7z x .\rust-src-1.50.0-dev.tar
pushd rust-src-1.50.0-dev
cp -Recurse .\rust-src\lib\* ~\.rustup\toolchains\xtensa\lib\
popd

rustup default xtensa

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-win64.zip -OutFile xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-win64.zip
7z x xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-win64.zip
$ClangPath=Join-Path -Path (Get-Location) -ChildPath xtensa-esp32-elf-clang\bin
$env:PATH="${ClangPath};$env:PATH"

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/x86_64-pc-windows-msvc/llvm-patch-0.1.x86_64-pc-windows-msvc.tar.gz -OutFile llvm-patch-0.1.x86_64-pc-windows-msvc.tar.gz
tar xzf llvm-patch-0.1.x86_64-pc-windows-msvc.tar.gz
$ClangPatchPath=Join-Path -Path (Get-Location) -ChildPath llvm-patch\bin
$env:PATH="${ClangPatchPath};$env:PATH"

Invoke-WebRequest https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip -OutFile rust-esp32-example.zip
7z x rust-esp32-example.zip
cd rust-esp32-example-main/rustlib
cargo build --release
cd ..
idf.py build
```
