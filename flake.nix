{
  description = "Development environment for natlint";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: inputs.flake-utils.lib.eachSystem
    [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ]
    (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import inputs.rust-overlay)
          ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            just
            rust-bin.stable.latest.default
          ];
        };
      }
    );
}
