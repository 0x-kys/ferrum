{
  description = "Ferrum Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux"; # Adjust according to your system 
      pkgs = import nixpkgs {
        inherit system;
        # config = { allowUnfree = true; }; # If you need unfree packages
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rustfmt
          pkgs.clippy
        ];

        shellHook = ''
          echo "Welcome to the Ferrum development environment!"
        '';
      };
    };
}
