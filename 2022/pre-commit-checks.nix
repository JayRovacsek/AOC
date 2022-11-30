{ self, pkgs, system }: {
  src = self;
  hooks = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    clippy.enable = true;
    cargo-check.enable = true;
  };
}
