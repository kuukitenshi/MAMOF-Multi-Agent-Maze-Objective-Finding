{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        devShells.default = pkgs.mkShell rec {
          packages = with pkgs; [
            pkg-config
            alsa-lib
            wayland
            vulkan-tools
            vulkan-headers
            vulkan-loader
            vulkan-validation-layers
            clang
            lld
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            xorg.libxkbfile
            udev
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
          ];
          
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath packages}";
        };
      }
    );
}
