use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mesa = manifest.join("../../mesa");
    let build = mesa.join("builddir");
    let targets = mesa.join("testgen-targets");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", targets.display());
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let targets = fs::read_to_string(&targets)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", targets.display()));
    for line in targets.lines() {
        let (name, target) = line
            .split_once('=')
            .unwrap_or_else(|| panic!("the target {line:?} has no name"));
        let path = build.join(target.replace("$ARCH", &arch));
        println!("cargo::rustc-env={name}={}", path.display());
        if name == "GBM_BACKEND_LIBRARY" {
            let backends = path.parent().unwrap();
            println!("cargo::rustc-env=GBM_BACKENDS={}", backends.display());
        }
    }
}
