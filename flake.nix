{
  description = "Development environment for natlint";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        solcBuildsListJson = let
          solcBinBaseUrl = "https://raw.githubusercontent.com/argotorg/solc-bin/26fc3fd";
          listJsons = {
            "x86_64-linux" = pkgs.fetchurl {
              url = "${solcBinBaseUrl}/linux-amd64/list.json";
              hash = "sha256-0a8i5v0wf5fmwsfhmvw7ws5ydc0lwjrn7f8znpx9lpzbd0k1hrql";
            };
            "aarch64-darwin" = pkgs.fetchurl {
              url = "${solcBinBaseUrl}/macosx-amd64/list.json";
              hash = "sha256-120ycgkq2wbljjiypkcr6jv58qikcyg2fcvadwr3y6gv5xq44pqd";
            };
          };
        in listJsons.${system} or null;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "natlint";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          env = pkgs.lib.optionalAttrs (solcBuildsListJson != null) {
            SVM_RELEASES_LIST_JSON = solcBuildsListJson;
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            just
            rust-bin.stable.latest.default
          ];
        };
      }
    );
}
