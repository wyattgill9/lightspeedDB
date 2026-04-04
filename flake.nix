{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    fenix,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-darwin"];
    eachSystem = f:
      nixpkgs.lib.genAttrs systems (system:
        f {
          inherit system;
          pkgs = nixpkgs.legacyPackages.${system};
          toolchain = fenix.packages.${system}.complete.toolchain;
        });
  in {
    devShells = eachSystem ({
      pkgs,
      toolchain,
      ...
    }: {
      default = pkgs.mkShell {
        buildInputs = [
          toolchain
          pkgs.cargo-nextest
        ];

        shellHook = ''
          echo "Cargo: $(cargo --version)"
          echo "Rust:  $(rustc --version)"
        '';
      };
    });
  };
}
