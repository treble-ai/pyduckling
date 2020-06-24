
use std::{env, str};
use std::path::Path;
use std::process::Command;


fn command_ok(cmd: &mut Command) -> bool {
    cmd.status().ok().map_or(false, |s| s.success())
}

fn command_output(cmd: &mut Command) -> String {
    str::from_utf8(&cmd.output().unwrap().stdout)
      .unwrap()
      .trim()
      .to_string()
}


fn main() {
    if command_ok(Command::new("stack").arg("--version")) {
        let ghc_lib = command_output(Command::new("stack").args(&["exec", "--", "ghc", "--print-libdir"]));
        let ghc_version = command_output(Command::new("stack").args(&["exec", "--", "ghc", "--numeric-version"]));
        let dir_path = env::current_dir().unwrap();
        let path = dir_path.to_str().unwrap();
        let ghc_lib_path = Path::new(&ghc_lib);
        let rts_path = ghc_lib_path.join("rts");
        println!("cargo:rustc-link-search=native={}/ext_lib/", path);
        println!("cargo:rustc-link-search=native={}", rts_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=dylib=ducklingffi");
        println!("cargo:rustc-link-lib=dylib=HSrts-ghc{}", ghc_version);
    }
    else {
        panic!("Stack was not found in the PATH")
    }
}
