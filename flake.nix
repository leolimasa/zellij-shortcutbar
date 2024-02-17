{
  description = "A sandbox rust project";

  # Declare inputs to fetch from
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
      flake = true;
    };
  };

  # Declare your Nix environment
  outputs = { self, nixpkgs, flake-utils }: 
    let
     forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ] (system: function nixpkgs.legacyPackages.${system});
    in {
      devShell = forAllSystems (pkgs: 
          with pkgs; mkShell {
            buildInputs = [
              # Add your dependencies here
              rustc
              rustup
              watchexec
              zlib
            ]; 
            hardeningDisable = [ "fortify" ];
            shellHook = ''
              export ENV_NAME=zellij-shortcutbar
              rustup target add wasm32-wasi
              exec $SHELL
            '';
          });
    };
}

