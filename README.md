# Rust ESP32 Example

An example project demonstrating integration with Rust for the ESP32-S2 and ESP32-C3 microcontrollers.

This example starts a FreeRTOS task to call a function in Rust and display the results in C. 

## Setup

First, install the ESP-IDF SDK as shown in the [Get Started](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html) guides. For best support of the ESP32-C3, install the SDK from the master branch.

### ESP32 and ESP32-S series

To support the Xtensa instruction set, build and install custom LLVM and Rust toolchains as shown in the [Rust On Xtensa](docs/rust-on-xtensa.md) guide.

### ESP32-C3

Install the RISCV target for Rust:

```sh
rustup target add riscv32i-unknown-none-elf
```

## Build Rust library

This will eventually be folded into the main CMake build, but for now, build the library separately.

### ESP32 or ESP32-S2

To build the Rust library, run:

```sh
cd rustlib
xargo build --release
cd ..
```

Build with manual configuration of the target:

```sh
export RUSTC=/usr/local/xtensa/rust/bin/rustc
export XARGO_RUST_SRC=/usr/local/xtensa/rust/library
cd rustlib
xargo build --release --target=xtensa-esp32-none-elf
cd ..
```

### ESP32-C3

To build the Rust library, run:

```sh
cd rustlib
cargo build --release
cd ..
```

Build with manual configuration of the target:

```sh
unset RUSTC
unset XARGO_RUST_SRC
cd rustlib
cargo build --release --target=riscv32i-unknown-none-elf
cd ..
```

## Configure

First ensure that the environment variables for the ESP32 SDK are properly set up. If you have followed the instructions in the Getting Started guide, activate the environment with the `get_idf` alias:

```sh
get_idf
```

Next, configure the project for the desired MCU.

For the ESP32:

```sh
idf.py set-target esp32
idf.py menuconfig
```

For the ESP32-S2:

```sh
idf.py set-target esp32s2
idf.py menuconfig
```

For the ESP32-C3:

```sh
idf.py set-target esp32c3
idf.py menuconfig
```

## Build

Build the project by running:

```sh
idf.py build
```

## Flash

Flash the compiled binary by running:

```sh
idf.py -p /dev/cu.SLAB_USBtoUART flash
```

## Monitor

Connect to the UART over USB port to monitor the application console:

```sh
idf.py -p /dev/cu.SLABtoUART monitor
```

To exit the monitor, press `Ctrl-]`.

## Debugging on ESP32-WROVER-KIT

```sh
openocd -f board/esp32-wrover-kit-3.3v.cfg
```
