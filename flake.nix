# Dev environment (replaces devbox) and package build for the TAAMS journal site
{
  description = "TAAMS journal site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      # Pin to the same version as the Docker builder stage
      rustVersion = "1.96.0";

      mkPkgs = system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      toolchain = pkgs:
        pkgs.rust-bin.stable.${rustVersion}.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
    in
    {
      devShells = forAllSystems (system:
        let pkgs = mkPkgs system; in {
          default = pkgs.mkShell {
            packages = [
              (toolchain pkgs)
              pkgs.bacon
              pkgs.sqlite
              pkgs.pkg-config
            ];
            RUST_SRC_PATH = "${(toolchain pkgs)}/lib/rustlib/src/rust/library";
          };
        });

      packages = forAllSystems (system:
        let pkgs = mkPkgs system; in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "journal-site";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            cargo = toolchain pkgs;
            rustc = toolchain pkgs;

            nativeBuildInputs = [ pkgs.pkg-config ];
          };
        });
    };
}
