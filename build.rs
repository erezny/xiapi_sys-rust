

//use std::process::Command;
// use std::env;
// use std::path::Path;

fn main() {
//    let out_dir = env::var("OUT_DIR").unwrap();
//    let dest_path = Path::new(&out_dir).join("xiapi.rs");
//    let dest_path_str = dest_path.to_str().expect("dest_path invalid");

    // println!("Dest Path: {}", dest_path_str);
    // //environment specific
    //
    //
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


    println!("cargo:rustc-link-lib=framework=m3api");
}
