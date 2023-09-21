{
  description = "shell for building cbt";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      naersk' = pkgs.callPackage naersk {};
      buildInputs = [
        pkgs.pkgconfig
        pkgs.udev
        pkgs.libinput
        pkgs.xorg.libX11
        pkgs.xorg.libXcursor
        pkgs.xorg.libXi
        pkgs.xorg.libXrandr
        pkgs.xorg.libXtst
        pkgs.xdotool
      ];
      libPath = pkgs.lib.makeLibraryPath [
        pkgs.libGL
        pkgs.xorg.libX11
        pkgs.xorg.libXcursor
        pkgs.xorg.libXi
        pkgs.xorg.libXrandr
        pkgs.xorg.libXtst
      ];
      shell = pkgs.mkShell {
        inherit buildInputs;
        LD_LIBRARY_PATH = libPath;
      };
      cbt = naersk'.buildPackage {
        inherit buildInputs;
        src = ./.;
      };
    in {
      defaultPackage.x86_64-linux = cbt;
      devShell.x86_64-linux = shell;
      packages.x86_64-linux = {
        inherit cbt shell;
      };
    };
}
