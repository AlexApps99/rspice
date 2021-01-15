#[cfg(feature = "generate")]
fn main() {
    println!("cargo:rustc-link-search=native=cspice/");
    println!("cargo:rustc-link-lib=static=cspice");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut wk_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    wk_dir.pop();

    bindgen::Builder::default()
        .header(wk_dir.join("cspice/SpiceUsr.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function(".*_c")
        .whitelist_type("Spice.*")
        .whitelist_var("SPICE_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("cspice.rs"))
        .expect("Unable to save bindings");
}

#[cfg(not(feature = "generate"))]
fn main() {
    println!("cargo:rustc-link-search=native=cspice/");
    println!("cargo:rustc-link-lib=static=cspice");
}