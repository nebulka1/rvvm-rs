use std::{
    env::{
        self,
        temp_dir,
    },
    path::PathBuf,
    process::Command,
};

const BUILDDIR_SUFFIX: &str = "rvvm-shared";
const LIB_NAME: &str = "rvvm";
static RVVM_PATH: &str = "rvvm-git";

fn main() {
    let build_dir: PathBuf = temp_dir().join(BUILDDIR_SUFFIX);

    println!("cargo:rerun-if-changed={RVVM_PATH}/src/rvvmlib.h");
    println!(
        "cargo:rustc-link-search={}",
        build_dir.as_os_str().to_str().unwrap()
    );
    println!("cargo:rustc-link-lib={LIB_NAME}");

    let status = Command::new("make")
        .arg("lib")
        .env("BUILDDIR", &build_dir)
        .current_dir(RVVM_PATH)
        .status()
        .expect("Failed to spawn make command");
    if !status.success() {
        panic!("Failed to build RVVM. Possibly make is not installed");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
