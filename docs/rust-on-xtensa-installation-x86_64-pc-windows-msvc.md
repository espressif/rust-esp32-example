# Rust on Xtensa Installation for Windows x64

Tested OS: Windows 10 x64

## Prerequisites

- Visual Studio - installed with option Desktop development with C++
- rustup - installed with nightly toolchain - https://rustup.rs/
- Chocolatey - https://chocolatey.org/

## Commands for PowerShell

```
choco install 7zip

rustup toolchain install nightly

$Version="1.54.0-dev"
$Arch="x86_64-pc-windows-msvc"
$RustDist="rust-${VERSION}-${ARCH}"

mkdir -p ~\.rustup\toolchains\ -ErrorAction SilentlyContinue
pushd ~\.rustup\toolchains\

Invoke-WebRequest "https://dl.espressif.com/dl/idf-rust/dist/${Arch}/${RustDist}.zip" -OutFile "${RustDist}.zip"
7z x .\${RustDist}.zip
popd

rustup default esp

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/${Arch}/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip -OutFile xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip
7z x xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip
$env:LIBCLANG_PATH=Join-Path -Path (Get-Location) -ChildPath xtensa-esp32-elf-clang\bin
$env:PATH+=";$env:LIBCLANG_PATH"

Invoke-WebRequest https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip -OutFile rust-esp32-example.zip
7z x rust-esp32-example.zip
cd rust-esp32-example-main
idf.py build
```
