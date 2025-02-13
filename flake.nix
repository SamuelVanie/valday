{
  description = "Valentine's Day App with Yew";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        overlay = rust-overlay.overlays.default;
        pkgs' = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };

        rustToolchain = pkgs'.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.llvmPackages_latest.clang
            pkgs.nodejs_20
            pkgs.trunk
            pkgs.rust-analyzer
            pkgs.cargo-generate
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
          shellHook = ''
            echo "Yew Valentine's App Dev Shell"
          '';
        };

      }
    );
}
