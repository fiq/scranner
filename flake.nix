{
  description = "Rust scranner dev";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      in {
        packages.default = pkgs.callPackage ./package.nix { };
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [ 
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            libglvnd
            wayland
          ];

          RUSTFLAGS = map (a: "-C link-arg=${a}") [
              "-lEGL"
              "-lwayland-client"
          ];

          LD_LIBRARY_PATH =  with pkgs; lib.makeLibraryPath [
            libxkbcommon
          ];
        };
      }
    );
}
