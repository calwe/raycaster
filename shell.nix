{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
      pkgs.alsaLib
      pkgs.udev
      pkgs.vulkan-loader
      pkgs.wayland
      pkgs.libxkbcommon
    ]}"
  '';
  buildInputs = [
    cargo
    rust-analyzer

    # fast compiler
    lld
    clang

    # bevy dep packages
    pkgconfig
    udev
    alsaLib
    #lutris
    wayland
    libxkbcommon

    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
  ];
}