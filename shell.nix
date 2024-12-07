{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
  ];

  shellHook = ''
    echo "Welcome to the Ferrum development environment!"
  '';

  # Optional: environment variables
  # RUSTFLAGS = ["-C", "link-arg=-fuse-ld=lld"];
}
