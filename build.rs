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

   //let dest_path_str = dest_path.to_str().expect("dest_path invalid");

    // println!("Dest Path: {}", dest_path_str);
    // //environment specific
    //
    //

    let mut bindings = bindgen::builder();
    bindings.forbid_unknown_types();



    let h_path = String::from("/Library/Frameworks/m3api.framework/Headers/xiApi.h");

    bindings.link_framework("m3api");
    bindings.match_pat("m3Identify.h");
    bindings.match_pat("xiApi.h");
    bindings.match_pat("wintypedefs.h");
    bindings.header(h_path);

    let bindings = bindings.generate();
    let bindings = bindings.unwrap();
    bindings.write_to_file(&dest_path).unwrap();
    // let output = Command::new("../rust-bindgen/target/debug/bindgen")
    //         .args(&["-l", "m3api"])
    //         .args(&["-match", "m3Identify.h", "-match" , "xiApi.h"])
    //         .args(&["-match", "wintypedefs.h"])
    //         .args(&["-o", dest_path_str])
    //         .args(&[ "/Library/Frameworks/m3api.framework/Headers/xiApi.h"])
    //         .output().unwrap_or_else(|e| {
    //              panic!("failed to execute process: {}", e)
    //          });
    //
    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    //
    // let rust_output = Command::new("rustc")
    //     .args(&["--out-dir", &out_dir])
    //     .args( &[dest_path_str])
    //     .output().unwrap_or_else( |e| {
    //         panic!("failed to execute process: {}", e)
    //         });
    //
    //
    // println!("status: {}", rust_output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&rust_output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&rust_output.stderr));


    println!("cargo:include={}", &dest_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=framework=m3api");
}
