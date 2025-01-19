{
  lib,
  wayland,
  libglvnd,
  pkg-config,
  libGL,
  libxkbcommon,
  xorg,
  vulkan-loader,
  makeRustPlatform,
  makeBinaryWrapper,
  rust-bin,
  xwayland,
  wayland-protocols,
}:
let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
    rustc = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
  };
in
rustPlatform.buildRustPackage rec {
  pname = "scranner";
  version = "0.0.1";

  src = ./.;

  nativeBuildInputs = [
    pkg-config
    makeBinaryWrapper
  ];

  RUSTFLAGS = map (a: "-C link-arg=${a}") [
    "-lEGL"
    "-lwayland-client"
  ];

  buildInputs = [
    wayland
    vulkan-loader
    libglvnd
    libGL
    libxkbcommon
    xorg.libX11
    xorg.libxcb
    xwayland
    wayland-protocols
  ];

   postInstall = ''
    wrapProgram "$out/bin/scranner" \
      --prefix LD_LIBRARY_PATH : ${
        lib.makeLibraryPath ([
          libxkbcommon
          wayland
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
        ] ++ buildInputs)
      }
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
