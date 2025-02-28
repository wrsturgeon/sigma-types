{ pkgs, ... }:
{
  programs =
    builtins.mapAttrs
      (_: package: {
        inherit package;
        enable = true;
      })
      {
        inherit (pkgs) mdformat rustfmt taplo;
        nixfmt = pkgs.nixfmt-rfc-style;
      };
  projectRootFile = "flake.nix";
}
