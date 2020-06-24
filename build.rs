
fn main() {
    println!("cargo:rustc-link-search=native=/home/andfoy/Documentos/Treble/pyduckling/lib/");
    println!("cargo:rustc-link-search=native=/home/andfoy/.stack/programs/x86_64-linux/ghc-tinfo6-8.6.5/lib/ghc-8.6.5/rts");
    println!("cargo:rustc-link-lib=dylib=ducklingffi");
    println!("cargo:rustc-link-lib=dylib=HSrts-ghc8.6.5");
    // println!("cargo:rustc-link-lib=static=ducklingffi");
    // println!("cargo:rustc-link-lib=static=HSbase-4.12.0.0");
}
