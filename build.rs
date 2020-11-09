use cmake::Config;
use std::env;

fn main() {
    let mut build_zlib_flag = "OFF";
    let has_zlib = pkg_config::find_library("zlib").is_ok();
    let build_zlib = !has_zlib;
    if build_zlib {
        build_zlib_flag = "ON";
    }

    // NOTE: Rust always uses the release CRT on Windows
    //  so in order to avoid dealing with msvc bullshit, always use Release profile

    let dst = Config::new("assimp")
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_INSTALL_PDB", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ASSIMP_NO_EXPORT", "ON")
        .define("ASSIMP_BUILD_ZLIB", build_zlib_flag)
        .define("ASSIMP_BUILD_ALL_IMPORTERS_BY_DEFAULT", "OFF")
        .define("ASSIMP_BUILD_FBX_IMPORTER", "ON")
        .define("ASSIMP_BUILD_OBJ_IMPORTER", "ON")
        .define("ASSIMP_BUILD_GLTF_IMPORTER", "ON")
        //.define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
        .define("LIBRARY_SUFFIX", "")
        .profile("RelWithDebInfo")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=assimp");

    if build_zlib {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    }

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
