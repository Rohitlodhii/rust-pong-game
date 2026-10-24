use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let Ok(vcpkg_root) = env::var("VCPKG_ROOT") else {
        return;
    };
    let triplet = env::var("VCPKGRS_TRIPLET").unwrap_or_else(|_| String::from("x64-windows"));

    let dll = PathBuf::from(vcpkg_root)
        .join("installed")
        .join(triplet)
        .join("bin")
        .join("SDL2.dll");

    if !dll.exists() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is always set by Cargo"));
    // OUT_DIR is <target>/<profile>/build/<package>-<hash>/out
    let Some(target_dir) = out_dir.ancestors().nth(3) else {
        return;
    };

    for dir in [target_dir.to_path_buf(), target_dir.join("deps")] {
        if dir.is_dir() {
            let dest = dir.join("SDL2.dll");
            fs::copy(&dll, &dest).unwrap_or_else(|err| {
                panic!(
                    "failed to copy {} to {}: {err}",
                    dll.display(),
                    dest.display()
                )
            });
        }
    }
}
