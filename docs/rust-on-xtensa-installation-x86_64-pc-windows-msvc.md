# Rust on Xtensa Installation for Windows x64

Following instructions are specific for ESP32 and ESP32-S series based on Xtensa architecture.

Instructions for ESP-C series based on RISC-V architecture are described in document for [ESP32-C3](../README.md#esp32-c3).

Tested OS: Windows 10 x64

## Prerequisites

- Visual Studio - installed with option Desktop development with C++
- rustup - installed with nightly toolchain - https://rustup.rs/
- Chocolatey - https://chocolatey.org/

## Installation commands for PowerShell

```sh
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

Invoke-WebRequest https://github.com/espressif/llvm-project/releases/download/esp-12.0.1-20210823/xtensa-esp32-elf-llvm12_0_1-esp-12.0.1-20210823-win64.zip -OutFile xtensa-esp32-elf-llvm12_0_1-esp-12.0.1-20210823-win64.zip
7z x xtensa-esp32-elf-llvm12_0_1-esp-12.0.1-20210823-win64.zip
$env:LIBCLANG_PATH=Join-Path -Path (Get-Location) -ChildPath xtensa-esp32-elf-clang\bin
$env:PATH+=";$env:LIBCLANG_PATH"

Invoke-WebRequest https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip -OutFile rust-esp32-example.zip
7z x rust-esp32-example.zip
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
