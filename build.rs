extern crate pkg_config;

use std::process::Command;
use std::env;
use std::path::Path;

use std::fs::File;
use std::io::Write;

fn main() {
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut status = File::create("/tmp/cargo.log").unwrap();
    status.write_fmt(format_args!("Starting\n")).unwrap();

    let lib_dir = Path::new(&cur_dir)
        .join("ccv")
        .join("lib");
    status.write_fmt(format_args!("lib: {:?}\n", lib_dir)).unwrap();

    let configure_cmd = lib_dir.join("configure");
    status.write_fmt(format_args!("configure: {:?}\n", configure_cmd)).unwrap();
    Command::new(configure_cmd)
        .current_dir(lib_dir.clone())
        .status()
        .expect("Error in lib/configure");

    let make_cmd = "make";
    status.write_fmt(format_args!("make: {:?}\n", make_cmd)).unwrap();

    Command::new(make_cmd)
        .args(&["-C", lib_dir.to_str().unwrap()])
        .current_dir(lib_dir.clone())
        .status()
        .expect("Error in make");

    println!("cargo:rustc-link-search=native={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=ccv");
    println!("cargo:rustc-link-lib=ccv");

    // Now handle libpng
    let lib = pkg_config::Config::new().statik(false).atleast_version("1.6").probe("libpng").expect("Could not find libpng");

    // Link path
    for path in &lib.link_paths {
        println!("cargo:rustc-link-search={}", path.to_str().expect("Could not convert path to str"));
    }

    for lib in &lib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

}