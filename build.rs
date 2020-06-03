
fn main() {
    println!("cargo:rustc-link-search=native=/home/andfoy/Documentos/Treble/pyduckling/bin/");
    println!("cargo:rustc-link-lib=dylib=ducklingffi");
}
