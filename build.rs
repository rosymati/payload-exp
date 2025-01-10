use std::path::PathBuf;

fn main() {
    let libpayload_dir =
        std::env::var("LIBPAYLOAD_DIR").expect("Please set LIBPAYLOAD_DIR environment variable");
    let libpayload_path = PathBuf::from(libpayload_dir);

    println!(
        "cargo:rustc-link-search={}",
        libpayload_path.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=payload");

    println!(
        "cargo:rustc-link-arg=-T{}",
        libpayload_path.join("lib/libpayload.ldscript").display()
    );
}
