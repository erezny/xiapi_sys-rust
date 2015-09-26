extern crate bindgen;

//use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;


use bindgen::{Bindings, BindgenOptions, LinkType, Logger};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dir = Path::new(&out_dir).join("lib/");
    let dest_path = Path::new(&lib_dir).join("xiapi.rs");

    fs::create_dir(&lib_dir);

    let mut bindings = bindgen::builder();
    bindings.forbid_unknown_types();

     // //environment specific
    let h_path = String::from("/Library/Frameworks/m3api.framework/Headers/xiApi.h");

    bindings.link_framework("m3api");
    bindings.match_pat("m3Identify.h");
    bindings.match_pat("xiApi.h");
    bindings.match_pat("wintypedefs.h");
    bindings.header(h_path);

    let bindings = bindings.generate();
    let bindings = bindings.unwrap();
    bindings.write_to_file(&dest_path).unwrap();

    println!("cargo:include={}", &dest_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=framework=m3api");
}
