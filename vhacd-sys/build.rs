use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);

    bindgen::Builder::default()
        .header("../v-hacd/src/VHACD_Lib/public/VHACD.h")
        .header("src/bridge.h")
        .clang_args(&["-I", "../v-hacd/src/VHACD_Lib/public"])
        .clang_args(&["-x", "c++", "-std=c++11"])
        .disable_name_namespacing()
        .generate_inline_functions(true)
        .whitelist_function("IVHACD_.*")
        .whitelist_function("VHACD::CreateVHACD")
        .whitelist_recursively(true)
        .whitelist_type("VHACD::IVHACD")
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new()
        .include("../v-hacd/src/VHACD_Lib/inc")
        .include("../v-hacd/src/VHACD_Lib/public")
        .flag("-std=c++11")
        .flag("-Wno-ignored-qualifiers")
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-unused-parameter")
        .files(&[
            "../v-hacd/src/VHACD_Lib/src/VHACD.cpp",
            "../v-hacd/src/VHACD_Lib/src/vhacdRaycastMesh.cpp",
            "../v-hacd/src/VHACD_Lib/src/vhacdICHull.cpp",
            "../v-hacd/src/VHACD_Lib/src/vhacdManifoldMesh.cpp",
            "../v-hacd/src/VHACD_Lib/src/btConvexHullComputer.cpp",
            "../v-hacd/src/VHACD_Lib/src/vhacdVolume.cpp",
            "../v-hacd/src/VHACD_Lib/src/VHACD-ASYNC.cpp",
            "../v-hacd/src/VHACD_Lib/src/vhacdMesh.cpp",
            "../v-hacd/src/VHACD_Lib/src/FloatMath.cpp",
            "../v-hacd/src/VHACD_Lib/src/btAlignedAllocator.cpp",
            "src/bridge.cpp",
        ])
        .compile("v-hacd");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    Ok(())
}
