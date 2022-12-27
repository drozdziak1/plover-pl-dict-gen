use std::{
    env,
    path::{Path, PathBuf},
};

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
        morfeusz2_header.push("include");

        let mut b =
            autocxx_build::Builder::new("src/lib.rs", &[&morfeusz2_header, Path::new("src")])
                .build()?;

        b.flag_if_supported("-std=c++14").compile("morfeusz2-sys");
        println!("cargo:rerun-if-changed=src/lib.rs");

        println!("cargo:rustc-link-lib=morfeusz2");
    } else {
        return Err(format!("Please specify libmorfeusz2 with MORFEUSZ2_PATH").into());
    }

    Ok(())
}
