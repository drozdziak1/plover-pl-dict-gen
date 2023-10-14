{
  description = "The Polish machine stenography dictionary generator for Plover";

  inputs = {
    flake-utils.url = github:numtide/flake-utils;
    nixpkgs.url = github:NixOS/nixpkgs/release-22.11;
    rust-overlay.url = github:oxalica/rust-overlay;
    cargo2nix = {
      url = github:cargo2nix/cargo2nix;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };


  outputs = { self, nixpkgs, flake-utils, rust-overlay, cargo2nix }: flake-utils.lib.eachDefaultSystem (system:
    let
      rustVersion = "1.65.0";
      pkgs = import nixpkgs {
        overlays = [ rust-overlay.overlays.default cargo2nix.overlays.default];
        inherit system;
      };
      rustPkgs = pkgs.rustBuilder.makePackageSet {
        inherit rustVersion;
        packageFun = import ./rust/Cargo.nix;
      };
    in
    {
      devShell = pkgs.mkShell
        {
          nativeBuildInputs = with pkgs; [ pkg-config stdenv.cc llvmPackages.libstdcxxClang libclang.dev libclang.lib ];
          MORFEUSZ2_PATH="${self.packages.${system}.morfeusz2}";
          LIBCLANG_PATH="${pkgs.libclang.lib}/lib";
          buildInputs = [
            self.packages.${system}.morfeusz2
            pkgs.rust-bin.stable.${rustVersion}.default # 
            cargo2nix.outputs.packages.${system}.cargo2nix
            (pkgs.libsForQt5.callPackage ./nix/packages/plover.nix { }).dev
          ];
        };
      packages = {
        plover-pl-dict-gen-rs = (rustPkgs.workspace.plover-pl-dict-gen-rs { }).bin;
        default = self.packages.${system}.plover-pl-dict-gen-rs;
        morfeusz2 = pkgs.callPackage ./nix/packages/morfeusz2.nix { };
      };
    });
}
