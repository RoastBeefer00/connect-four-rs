{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    pkg-config
    alsa-lib
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    udev
    clang
    lld
    trunk
    # If on x11
    xorg.libX11
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    # If on wayland
    libxkbcommon
    wayland
    # Rust
    # rustToolchain
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    targets = [
      "wasm32-unknown-unknown"
    ];
    channel = "stable";
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts = {
    web-run.exec = ''
      trunk serve
    '';
    web-build.exec = ''
      #!/bin/bash

      set -e  # Exit on any error

      echo "Running trunk build..."
      trunk build

      echo "Looking for .wasm files in dist folder..."
      wasm_files=$(find dist -name "*.wasm" -type f)

      if [ -z "$wasm_files" ]; then
          echo "No .wasm files found in dist folder"
          exit 1
      fi

      echo "Found .wasm files:"
      echo "$wasm_files"

      echo "Compressing with gzip -9..."
      for wasm_file in $wasm_files; do
          echo "Compressing: $wasm_file"
          gzip -9 "$wasm_file"
      done

      echo "Done!"
    '';

    enterShell = ''
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
        pkgs.lib.makeLibraryPath [
          pkgs.alsa-lib
          pkgs.udev
          pkgs.vulkan-loader
          pkgs.wayland
          pkgs.libxkbcommon
        ]
      }"
    '';
  };

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
