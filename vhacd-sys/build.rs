use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);

    bindgen::Builder::default()
        .header("src/bridge.h")
        .clang_args(&["-I", "../v-hacd/include/"])
        .clang_args(&["-x", "c++", "-std=c++11"])
        .derive_copy(false)
        .disable_name_namespacing()
        .generate_inline_functions(true)
        .allowlist_function("IVHACD_.*")
        .allowlist_function("VHACD::CreateVHACD")
        .allowlist_recursively(true)
        .allowlist_type("VHACD::IVHACD")
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new()
        .include("../v-hacd/include/")
        .flag("-std=c++11")
        .flag("-Wno-ignored-qualifiers")
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-unused-parameter")
        .files(&[
            "src/bridge.cpp",
        ])
        .compile("v-hacd");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    Ok(())
}
