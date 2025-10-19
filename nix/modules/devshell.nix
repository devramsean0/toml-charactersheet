{ inputs, ... }: {
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "toml-charactersheet-shell";
      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];
      packages = with pkgs; [
        just
        nixd # Nix language server
        bacon
        wkhtmltopdf
        pandoc
        #        config.process-compose.cargo-doc-live.outputs.package
      ];
    };
  };
}
