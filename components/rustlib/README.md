# rustlib

This component generates a static library from Rust source code and exposes functions that will be callable from the main IDF application from C.

This component is responsible for three separate functions:

* Export C bindings for Rust functions with `cbindgen`.
* Import Rust binding for C functions with `bindgen`.
* Build a top level static library from Rust code. The `cargo` build command is wrapped in a CMake `ExternalProject_Add` command.

Currently the `build.rs` script expects the `clib` component to export a header with the name `CApi.h`. If the name of the include file changes or if more files are added, the build script must be updated to process the latest headers.

Depending on the complexity of the project, the `cbindgen` and `bindgen` steps could be broken out into one or more IDF components or Rust crates. However, to prevent redundant duplication of the Rust runtime, all Rust crates should be compiled into a single static library that will be linked into the final IDF application.