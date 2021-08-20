use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = PathBuf::from(env::var("CARGO_BUILD_TARGET_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    run_cbindgen(&cargo_dir, &target_dir);
    run_bindgen(&target, &out_dir);
}

fn run_cbindgen(cargo_dir: &Path, target_dir: &Path) {
    let out = target_dir.join("RustApi.h");

    cbindgen::Builder::new()
        .with_crate(cargo_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&out);

    println!("cargo:rerun-if-changed={}", out.display());
}

fn run_bindgen(target: &str, out_dir: &Path) {
    let header = "../clib/include/CApi.h";
    let out = out_dir.join("bindings.rs");

    let mut builder = bindgen::Builder::default();
    builder = builder.header(header);
    match target {
        "riscv32imc-esp-espidf" => {
            builder = builder.clang_arg("--target=riscv32");
            builder = builder.use_core();
            builder = builder.ctypes_prefix("crate::ffi");
        }
        "xtensa-esp32-espidf" => {
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
        .write_to_file(&out)
        .expect("Couldn't save bindings!");

    println!("cargo:rerun-if-changed={}", header);
    println!("cargo:rerun-if-changed={}", out.display());
}
