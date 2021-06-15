use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    run_cbindgen(&cargo_dir, &out_dir);
    run_bindgen(&target, &out_dir);
}

fn run_cbindgen(_cargo_dir: &Path, _out_dir: &Path) {
    // TODO: Run cbindgen
}

fn run_bindgen(target: &str, out_dir: &Path) {
    let mut builder = bindgen::Builder::default();
    builder = builder.header("../main/CApi.h");
    match target {
        "riscv32i-unknown-none-elf" => {
            builder = builder.clang_arg("--target=riscv32");
            builder = builder.use_core();
            builder = builder.ctypes_prefix("crate::ffi");
        }
        "xtensa-esp32-none-elf" => {
            // Make sure that LLVM_CONFIG_PATH has been set to point to the
            // Xtensa build of llvm-config.
            builder = builder.clang_arg("--target=xtensa-esp32-elf");
        }
        _ => {
            panic!("Unexpect target archtitecture: {}", &target);
        }
    }

    let bindings = builder.generate().expect("Couldn't generate bindings!");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't save bindings!");
}
