use std::{env, path::PathBuf};

type ErrBox = Box<dyn std::error::Error>;
fn main() -> Result<(), ErrBox> {
    if let Some(morfeusz2_path_str) = env::var_os("MORFEUSZ2_PATH") {
        let morfeusz2_path = PathBuf::from(&morfeusz2_path_str);
        println!(
            "cargo:rustc-link-search=native={}/lib",
            morfeusz2_path.display()
        );

        println!("cargo:rerun-if-changed={}", morfeusz2_path.display());

        let mut morfeusz2_header = morfeusz2_path.clone();
        morfeusz2_header.push("include/morfeusz2.h");

        let bindings = bindgen::Builder::default()
            .header(morfeusz2_header.display().to_string())
            .enable_cxx_namespaces()
            .allowlist_file(".*morfeusz.*")
            .opaque_type("std::.*")
            .clang_args(&["-x", "c++", "-std=c++11"])
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()?;

        let out_path = PathBuf::from(env::var("OUT_DIR")?);
        eprintln!("written out to {}", out_path.display());
        bindings.write_to_file(out_path.join("bindings.rs"))?;
    } else {
        return Err(format!("Please specify libmorfeusz2 with MORFEUSZ2_PATH").into());
    }

    Ok(())
}
