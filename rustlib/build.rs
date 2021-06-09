use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    // let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // fs::remove_dir_all(&out_dir).unwrap();
    // fs::create_dir_all(&out_dir).unwrap();

    // run_cbindgen(&cargo_dir, &out_dir);
    run_bindgen(&target, &out_dir);
}

// fn run_cbindgen(_cargo_dir: &Path, _out_dir: &Path) {
    // TODO: Run cbindgen
// }

fn run_bindgen(target: &str, out_dir: &Path) {
    // if target == "xtensa-esp32-none-elf" {
    //     env::set_var("LLVM_CONFIG_PATH", "/usr/local/xtensa/llvm/bin/llvm-config");
    // }

    let mut builder = bindgen::Builder::default();
    builder = builder.header("../main/CApi.h");
    builder = builder.use_core();
    builder = builder.ctypes_prefix("ffi");

    println!("Target: {}", target);
    match target {
        "riscv32i-unknown-none-elf" => {
            builder = builder.clang_arg("--target=riscv32");
        }
        "xtensa-esp32-none-elf" => {
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
