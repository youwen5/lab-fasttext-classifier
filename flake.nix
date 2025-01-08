{
  description = "Fasttext experiments";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          craneLib = crane.mkLib pkgs;
        in
        {
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;

            buildInputs = with pkgs; [
              fasttext
            ];
          };
        }
      );
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          craneLib = crane.mkLib pkgs;
        in
        {
          default = craneLib.devShell {
            inputsFrom = [ self.packages.${system}.default ];

            packages = with pkgs; [
              clippy
              rust-analyzer
            ];
          };
        }
      );
    };
}
