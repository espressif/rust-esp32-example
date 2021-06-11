# Rust on ESP32 (Xtensa)

## Quick start

The installation process of ready to use custom build of Rust and LLVM is described here: 

* [Linux Ubuntu x64/Mint x64](rust-on-xtensa-installation-x86_64-unknown-linux-gnu.md)
* [macOS Big Sur x64](rust-on-xtensa-installation-x86_64-apple-darwin.md)
* [macOS Big Sur arm64](rust-on-xtensa-installation-aarch64-apple-darwin.md)
* [Windows 10 x64](rust-on-xtensa-installation-x86_64-pc-windows-msvc.md)

Make sure to install Rust toolchain nightly, otherwise resolving of std crate might fail with following error message:

```
error[E0463]: can't find crate for `std`
  |
  = note: the `xtensa-esp32-none-elf` target may not be installed
```

`rustup toolchain install nightly`

Following text describes the build process when building LLVM and Rust from the scratch.

## Using Rust for ESP32 Development

Given the popularity of the ESP32 chip, there has been an interest in developing applications for it using the Rust programming language. The ESP32 has traditionally used the Xtensa instruction set, which is not officially supported by Rust.

A new version of the ESP32, the ESP32-C3, has recently been released. The new C3 variant is based on a RISC-V architecture. RISC-V is a supported architecture for LLVM and Rust. This document, however, will concern itself with the more popular and, at the time of writing, more powerful Xtensa-based systems.

## Build LLVM for Xtensa

Although the Xtensa instruction set is not supported by the LLVM project, Espressif has continued to maintain a fork. Unfortunately, these changes have not been migrated upstream into the main project. This would be a prerequisite before official support in Rust is possible.

The fork is hosted at [espressif/llvm-project](https://github.com/espressif/llvm-project)

The Rust for for Xtensa fork (see below) includes the LLVM repo as a submodule, so it is not necessary to work with this repository directly. However, it uses an older version of the code. To use the most up to date, we can build the toolchain separately from the main repository.

Note, however, that while clang will successfully compile code, the toolchain is not able to link final executables for either the host or target systems.

```
% git clone https://github.com/espressif/llvm-project.git
% cd llvm-project
% mkdir build
% cd build
% cmake ../llvm -G "Ninja" -DLLVM_TARGETS_TO_BUILD="X86" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="Xtensa" -DLLVM_ENABLE_PROJECTS="clang" -DCMAKE_INSTALL_PREFIX=/usr/local/xtensa/llvm -DCMAKE_BUILD_TYPE=Release
% ninja
% sudo ninja install
```

The Xtensa target support can be verified by printing the supported target architectures.

```
% /usr/local/xtensa/llvm/bin/llc --version
LLVM (http://llvm.org/):
  LLVM version 11.0.0
  Optimized build.
  Default target: x86_64-apple-darwin20.3.0
  Host CPU: cascadelake

  Registered Targets:
    x86    - 32-bit X86: Pentium-Pro and above
    x86-64 - 64-bit X86: EM64T and AMD64
    xtensa - Xtensa 32
```

The available CPUs and features for the architecture can also be queried.

```
% /usr/local/xtensa/llvm/bin/llc -march=xtensa -mattr=help
Available CPUs for this target:

  esp32    - Select the esp32 processor.
  esp32-s2 - Select the esp32-s2 processor.
  esp8266  - Select the esp8266 processor.
  generic  - Select the generic processor.

Available features for this target:

  atomctl      - Enable Xtensa ATOMCTL option.
  atomctl      - Enable Xtensa MEMCTL option.
  bool         - Enable Xtensa Boolean extension.
  coprocessor  - Enable Xtensa Coprocessor option.
  debug        - Enable Xtensa Debug option.
  density      - Enable Density instructions.
  dfpaccel     - Enable Xtensa Double Precision FP acceleration.
  div32        - Enable Xtensa Div32 option.
  exception    - Enable Xtensa Exception option.
  exception    - Enable Xtensa HighPriInterrupts option.
  extendedl32r - Enable Xtensa Extended L32R option.
  fp           - Enable Xtensa Single FP instructions.
  interrupt    - Enable Xtensa Interrupt option.
  loop         - Enable Xtensa Loop extension.
  mac16        - Enable Xtensa MAC16 instructions.
  miscsr       - Enable Xtensa Miscellaneous SR option.
  mul32        - Enable Xtensa Mul32 option.
  mul32high    - Enable Xtensa Mul32High option.
  nsa          - Enable Xtensa NSA option.
  prid         - Enable Xtensa Processor ID option.
  regprotect   - Enable Xtensa Region Protection option.
  rvector      - Enable Xtensa Relocatable Vector option.
  s32c1i       - Enable Xtensa S32C1I option.
  sext         - Enable Xtensa Sign Extend option.
  threadptr    - Enable Xtensa THREADPTR option.
  timerint     - Enable Xtensa Timer Interrupt option.
  windowed     - Enable Xtensa Windowed Register option.

Use +feature to enable a feature, or -feature to disable it.
For example, llc -mcpu=mycpu -mattr=+feature1,-feature2
```

## Build Rust for Xtensa

While it should be possible to skip the internal build of LLVM libraries and reuse the LLVM toolchain built above, that didn't work successfully. Instead, use the built in version of the toolchain.

```
% git clone https://github.com/MabezDev/rust-xtensa.git
% cd rust-xtensa
% git submodule update --init --recursive
% ./configure --experimental-targets=Xtensa --prefix=/usr/local/xtensa/rust
% python x.py build --stage=2
```

The install option for `x.py` runs into some problems if sudo permissions are needed (requiring cargo dependencies to be vendored), so create an install directory owned by the user.

```
% sudo mkdir -p /usr/local/xtensa/rust
% sudo chown `whoami` /usr/local/xtensa/rust
% python x.py install
```

Finally, the library source tree needed by `xargo` to build the core library crates needs to be copied into the installation as well.

```
% sudo cp Cargo.* /usr/local/xtensa/rust/
% sudo cp -r library /usr/local/xtensa/rust/
```

Finally, install `xargo` to wrap the normal `cargo` command.

```sh
% cargo install xargo
```

## Using Rust

To build a Rust library for Xtensa, set up the environment for `xargo` to find the custom Rust toolchain and build with `xargo` instead of `cargo`.

```sh
export RUSTC=/usr/local/xtensa/rust/bin/rustc
export XARGO_RUST_SRC=/usr/local/xtensa/rust/library
xargo build --release --target=xtensa-esp32-none-elf
```
