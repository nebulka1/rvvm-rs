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
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir: PathBuf = out_path.join(BUILDDIR_SUFFIX);

    let is_dynamic = env::var("CARGO_FEATURE_DYNAMIC").is_ok();
    let kind = if is_dynamic {
        "dylib"
    } else {
        build_static(&build_dir);
        // ty: Nebulka <arapun@proton.me>
        "static:+whole-archive,-bundle"
    };

    println!("cargo:rerun-if-changed={RVVM_PATH}/src/rvvmlib.h");
    println!("cargo:rustc-link-lib={kind}={LIB_NAME}");

    if is_dynamic {
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
