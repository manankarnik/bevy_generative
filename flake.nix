{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        with pkgs;
        {
          devShells.default = mkShell rec {
            nativeBuildInputs = [
              pkg-config
            ];
            buildInputs = [ udev alsa-lib vulkan-loader cairo gtk3 ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
        }
      );
}
