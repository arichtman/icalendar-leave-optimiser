{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
      ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.default = with pkgs; mkShell {
        buildInputs = [
          rust-bin.stable.latest.default
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
          zola
        ];
      };
    });
}
