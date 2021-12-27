use std::path::{Path, PathBuf};

extern crate bindgen;

fn compile_gns() -> PathBuf {
    let build_type = if Ok("release".to_owned()) == std::env::var("PROFILE") {
        "Release"
    } else {
        "Debug"
    };

    let mut conf = cmake::Config::new("GameNetworkingSockets");

    let dst = conf
        .define("CMAKE_BUILD_TYPE", build_type)
        .build();

    dst
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let out_dir = compile_gns();
    let lib_dir = out_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=GameNetworkingSockets_s");

    println!("cargo:rustc-link-lib=dylib=stdc++");

    let gns_deps = vec![
        "protobuf",
        "openssl",
    ];

    for dep in gns_deps {
        for lib in pkg_config::probe_library(dep).unwrap().libs {
            println!("cargo:rustc-link-lib={}", lib)
        }
    }

    let include_dir = out_dir.join("include");

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingtypes.h").to_string_lossy())
        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingsockets.h").to_string_lossy())
        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingutils.h").to_string_lossy())
        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingmessages.h").to_string_lossy())
        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingsockets.h").to_string_lossy())
        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingsockets_flat.h").to_string_lossy())
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .clang_arg(format!("-I{}", include_dir.join("GameNetworkingSockets/steam").display()))
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true
        })
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(
            if env::var("GNS_MAKE_BINDINGS").is_ok() {
                Path::new("src/bindings.rs").to_owned()
            } else {
                out_path.join("bindings.rs")
            }
        )
        .expect("Couldn't write bindings!");

    Ok(())
}

//use std::path::{Path, PathBuf};
//
//extern crate bindgen;
//
//fn compile_gns() -> PathBuf {
//    let build_type = if Ok("release".to_owned()) == std::env::var("PROFILE") {
//        "Release"
//    } else {
//        "Debug"
//    };
//
//    let mut conf = cmake::Config::new("GameNetworkingSockets");
//
//    let dst = conf
//        .define("CMAKE_BUILD_TYPE", build_type)
//        .build();
//
//    dst
//}
//
//
//#[cfg(feature = "docs-only")]
//fn main() {}
//
//#[cfg(not(feature = "docs-only"))]
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    use std::env;
//    //use std::path::{Path, PathBuf};
//    //use std::fs::{self};
//
//    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//    //let sdk_loc = env::var("STEAM_SDK_LOCATION")
//    //    .expect("STEAM_SDK_LOCATION must be set");
//    //let sdk_loc = Path::new(&sdk_loc);
//    //println!("cargo:rerun-if-env-changed=STEAM_SDK_LOCATION");
//
//    //let triple = env::var("TARGET").unwrap();
//    //let mut lib = "steam_api";
//    //let mut link_path = sdk_loc.join("redistributable_bin");
//    //if triple.contains("windows") {
//    //    if !triple.contains("i686") {
//    //        lib = "steam_api64";
//    //        link_path.push("win64");
//    //    }
//    //} else if triple.contains("linux") {
//    //    if triple.contains("i686") {
//    //        link_path.push("linux32");
//    //    } else {
//    //        link_path.push("linux64");
//    //    }
//    //} else if triple.contains("darwin") {
//    //    link_path.push("osx");
//    //} else {
//    //    panic!("Unsupported OS");
//    //};
//
//    //if triple.contains("windows") {
//    //    let dll_file = format!("{}.dll", lib);
//    //    let lib_file = format!("{}.lib", lib);
//    //    fs::copy(link_path.join(&dll_file), out_path.join(dll_file))?;
//    //    fs::copy(link_path.join(&lib_file), out_path.join(lib_file))?;
//    //} else if triple.contains("darwin") {
//    //    fs::copy(link_path.join("libsteam_api.dylib"), out_path.join("libsteam_api.dylib"))?;
//    //} else if triple.contains("linux") {
//    //    fs::copy(link_path.join("libsteam_api.so"), out_path.join("libsteam_api.so"))?;
//    //}
//
//    //println!("cargo:rustc-link-search={}", out_path.display());
//    //println!("cargo:rustc-link-lib=dylib={}", lib);
//
//
//    let out_dir = compile_gns();
//    let lib_dir = out_dir.join("lib");
//
//    println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
//    println!("cargo:rustc-link-lib=static=GameNetworkingSockets_s");
//
//    println!("cargo:rustc-link-lib=dylib=stdc++");
//
//    let gns_deps = vec![
//        "protobuf",
//        "openssl",
//    ];
//
//    for dep in gns_deps {
//        for lib in pkg_config::probe_library(dep).unwrap().libs {
//            println!("cargo:rustc-link-lib={}", lib)
//        }
//    }
//
//    let include_dir = out_dir.join("include");
//
//    let bindings = bindgen::Builder::default()
//        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingtypes.h").to_string_lossy())
//        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingsockets.h").to_string_lossy())
//        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingutils.h").to_string_lossy())
//        .header(include_dir.join("GameNetworkingSockets/steam/isteamnetworkingmessages.h").to_string_lossy())
//        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingsockets.h").to_string_lossy())
//        .header(include_dir.join("GameNetworkingSockets/steam/steamnetworkingsockets_flat.h").to_string_lossy())
//        .clang_arg("-xc++")
//        .clang_arg("-std=c++11")
//        .clang_arg(format!("-I{}", include_dir.join("GameNetworkingSockets/steam").display()))
//        .rustfmt_bindings(true)
//        .default_enum_style(bindgen::EnumVariation::Rust {
//            non_exhaustive: true
//        })
//        .generate()
//        .expect("Unable to generate bindings");
//
//    bindings
//        .write_to_file(
//            if env::var("GNS_MAKE_BINDINGS").is_ok() {
//                Path::new("src/bindings.rs").to_owned()
//            } else {
//                out_path.join("bindings.rs")
//            }
//        )
//        .expect("Couldn't write bindings!");
//
//    Ok(())
//}
