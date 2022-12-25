use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    if cfg!(target_os = "macos") {
        build_iobluetooth();
    }
    tauri_build::build()
}

fn build_iobluetooth() {
    /* TODO: Rerun if server.rs changes */
    // println!("cargo:rerun-if-changed=../bluetooth-lib/iobluetooth/src/server.rs"); Does not work

    let mut build = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg("../bluetooth-lib/iobluetooth/Cargo.toml")
        .arg("--bin")
        .arg("soundcoremanager-iobtserver")
        .spawn()
        .expect("Failed to build iobluetooth server");

    if build.wait().unwrap().success() {
        let mut basedir = PathBuf::from(env::current_dir().unwrap());
        basedir.pop();
        let src = Path::join(&basedir, "bluetooth-lib/iobluetooth/target/release/soundcoremanager-iobtserver");
        /* Add target triple to server - Required by Tauri to bundle it */
        let target = std::env::var("TARGET").unwrap();
        std::fs::rename(
            src,
            Path::join(
                &basedir,
                format!("bluetooth-lib/iobluetooth/target/release/soundcoremanager-iobtserver-{}", target),
            ),
        )
        .expect("Failed to rename iobluetooth server");
    } else {
        panic!("Failed to build iobluetooth server!");
    }
}

/* https://github.com/rust-lang/cargo/issues/1759 */
fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}
