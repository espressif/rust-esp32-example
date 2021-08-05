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
$RustSrcDist="rust-src-${VERSION}"
$ToolchainDestinationDir="~/.rustup/toolchains/esp"

mkdir -p ~\.rustup\toolchains\esp

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/${Arch}/${RustDist}.tar.xz -OutFile ${RustDist}.tar.xz
7z e .\${RustDist}.tar.xz
7z x .\${RustDist}.tar
pushd ${RustDist}
cp -Recurse .\rustc\bin ~\.rustup\toolchains\esp\
cp -Recurse .\rustc\lib ~\.rustup\toolchains\esp\
cp -Recurse .\rustc\share ~\.rustup\toolchains\esp\
cp -ErrorAction SilentlyContinue -Recurse .\rust-std-${Arch}\lib\* ~\.rustup\toolchains\esp\lib\
popd

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/noarch/${RustSrcDist}.tar.xz -OutFile ${RustSrcDist}
7z e .\${RustSrcDist}.tar.xz
7z x .\${RustSrcDist}.tar
pushd ${RustSrcDist}
cp -ErrorAction SilentlyContinue -Recurse .\rust-src\lib\* ~\.rustup\toolchains\esp\lib\
popd

rustup default esp

Invoke-WebRequest https://dl.espressif.com/dl/idf-rust/dist/${Arch}/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip -OutFile xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip
7z x xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-win64.zip
$ClangPath=Join-Path -Path (Get-Location) -ChildPath xtensa-esp32-elf-clang\bin
$env:PATH="${ClangPath};$env:PATH"

Invoke-WebRequest https://github.com/espressif/rust-esp32-example/archive/refs/heads/main.zip -OutFile rust-esp32-example.zip
7z x rust-esp32-example.zip
cd rust-esp32-example-main
idf.py build
```
