use std::{
    env::{
        self,
        temp_dir,
    },
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

const BUILDDIR_SUFFIX: &str = "rvvm-lib";
const LIB_NAME: &str = "rvvm";
static RVVM_PATH: &str = "rvvm-git";

fn main() {
    let build_dir: PathBuf = temp_dir().join(BUILDDIR_SUFFIX);
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir)
            .expect("Failed to cleanup previous build");
    }

    let kind = if env::var("CARGO_FEATURE_DYNAMIC").is_ok() {
        "dylib"
    } else {
        build_static(&build_dir);
        "static"
    };

    println!("cargo:rerun-if-changed={RVVM_PATH}/src/rvvmlib.h");
    println!("cargo:rustc-link-lib={kind}={LIB_NAME}");

    if kind == "static" {
        println!(
            "cargo:rustc-link-search={}",
            build_dir.to_str().unwrap()
        );
        println!("cargo:rustc-link-lib={kind}=rvjit");
        println!("cargo:rustc-link-lib={kind}={LIB_NAME}_cpu32");
        println!("cargo:rustc-link-lib={kind}={LIB_NAME}_cpu64");
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

fn build_static(build_dir: &Path) {
    let status = Command::new("cmake")
        .args(["-S", ".", "-B", build_dir.to_str().unwrap()])
        .current_dir(RVVM_PATH)
        .status()
        .expect("Failed to spawn cmake command");

    if !status.success() {
        panic!(
            "Failed to build RVVM staticlib. Possibly cmake is not \
             installed"
        );
    }
    let status = Command::new("cmake")
        .args(["--build", "."])
        .current_dir(build_dir)
        .status()
        .expect("Failed to spawn cmake command");
    if !status.success() {
        panic!("Failed to build static lib RVVM");
    }
}
