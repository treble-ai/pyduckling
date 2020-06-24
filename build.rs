
use std::env;

fn main() {
    let dir_path = env::current_dir().unwrap();
    let path = dir_path.to_str().unwrap();
    let home = env::var("HOME").unwrap();
    println!("cargo:rustc-link-search=native={}/ext_lib/", path);
    println!("cargo:rustc-link-search=native={}/.stack/programs/x86_64-linux/ghc-tinfo6-8.6.5/lib/ghc-8.6.5/rts", home);
    println!("cargo:rustc-link-lib=dylib=ducklingffi");
    println!("cargo:rustc-link-lib=dylib=HSrts-ghc8.6.5");
    // println!("cargo:rustc-link-lib=static=ducklingffi");
    // println!("cargo:rustc-link-lib=static=HSbase-4.12.0.0");
}
