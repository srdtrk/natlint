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
              hash = "sha256-FGcYJmjrX5r6tR+5Y7PkFLDmi+aH7wqd5tUVx8EuESk=";
            };
            "aarch64-darwin" = pkgs.fetchurl {
              url = "${solcBinBaseUrl}/macosx-amd64/list.json";
              hash = "sha256-DV9CcC/7GT8yb2ozJ55nM2JUtjSZzeujlHRxgedjHog=";
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
