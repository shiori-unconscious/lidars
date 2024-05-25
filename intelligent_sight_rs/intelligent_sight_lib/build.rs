use glob::glob;
use std::{env, fs, path::Path, process::Command};

fn main() {
    let nested_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&nested_path);

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("cam_op/c_src/*").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("trt_op/cxx_src/*").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("cuda_op/cu_src/*").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("xmake.lua").display()
    );

    let clib_path = manifest_dir.join("clibs");
    println!("cargo:rustc-link-search=native={}", clib_path.display());

    println!("cargo:rustc-link-lib=dylib=camera_wrapper");
    println!("cargo:rustc-link-lib=dylib=cuda_wrapper");

    let target = env::var("TARGET").unwrap();

    if !target.contains("windows") && !target.contains("linux") {
        panic!("unsupported platform");
    }

    let result = Command::new("xmake")
        .status()
        .expect("failed to build clibs, please check installation of XMake");

    if !result.success() {
        panic!("failed to build clibs, please check installation of XMake");
    }

    let profile = env::var("PROFILE").unwrap();
    let out_dir_path = manifest_dir
        .parent()
        .unwrap()
        .join("target")
        .join(profile)
        .join("deps");

    for entry in glob(clib_path.join("*").to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name().expect("Failed to get file name");
                let destination_path = out_dir_path.join(file_name);
                fs::copy(entry, destination_path)
                    .expect("Failed to copy clibs to output directory");
            }
            Err(err) => {
                panic!("Failed to read entry: {}", err);
            }
        }
    }
}
