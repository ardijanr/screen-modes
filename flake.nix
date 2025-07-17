{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        runtimeDeps= with pkgs; [
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];

        libPath = lib.makeLibraryPath runtimeDeps ;
        

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        src = ./.;

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = (crane.mkLib pkgs).overrideScope (final: prev: {
          rustc = rustToolchain;
          cargo = rustToolchain;
          rustfmt = rustToolchain;
        });

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };

        screen_modes = craneLib.buildPackage {
          inherit cargoArtifacts src;
        };

        screen_modes_wrapped = pkgs.symlinkJoin{
          name= "screen_modes";
          paths = [ screen_modes ];
          buildInputs =[ pkgs.makeWrapper ];
          postBuild = ''wrapProgram $out/bin/screen_modes --prefix LD_LIBRARY_PATH : ${libPath}'';
        };

       in {


        packages.screen_modes = screen_modes_wrapped;
        packages.default = screen_modes_wrapped;

        apps.screen_modes = flake-utils.lib.mkApp {
          drv = screen_modes_wrapped;
        };

        devShell = pkgs.mkShell {

          # inputsFrom = builtins.attrValues self.checks;

          buildInputs = with pkgs; [ ] ++ runtimeDeps;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            # cargo-tarpaulin
            # cargo-edit
            # pkg-config
          ];

          LD_LIBRARY_PATH = libPath;
        };
      });
}

