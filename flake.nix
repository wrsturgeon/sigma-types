{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-analyzer-src = {
          flake = false;
          url = "github:rust-lang/rust-analyzer/nightly";
        };
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    treefmt-nix = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:numtide/treefmt-nix";
    };
  };
  outputs =
    {
      fenix,
      flake-utils,
      nix-filter,
      nixpkgs,
      self,
      treefmt-nix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pname = "sigma-types";
        version = "0.3.6";
        synopsis = "Types checked for an invariant.";
        description = synopsis;
        src = nix-filter {
          root = ./.;
          include = [
            ./Cargo.lock
            ./src
          ];
        };

        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        fenix-toolchain = "default";
        toolchain = fenix.packages.${system}.${fenix-toolchain}.withComponents [
          "cargo"
          "rustc"
        ];
        fenix-full-toolchain = "complete";
        full-toolchain = fenix.packages.${system}.${fenix-full-toolchain}.withComponents [
          "cargo"
          "clippy"
          "miri"
          "rustc"
          "rustfmt"
        ];
        rust-platform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };

        ENV = {
          # environment variables:
          MIRIFLAGS = "-Zmiri-disable-isolation";
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };

        dependencies = { };
        dev-dependencies = {
          paste = {
            features = [ ];
          };
          quickcheck = {
            features = [ ];
          };
          quickcheck_macros = {
            features = [ ];
          };
          serde_json = {
            features = [ "std" ];
          };
        };
        features = {
          malachite = {
            dependencies = {
              malachite-base = {
                features = [ ];
              };
            };
            other-features = [ ];
          };
          quickcheck = {
            dependencies = {
              quickcheck = {
                features = [ ];
              };
            };
            other-features = [ ];
          };
          serde = {
            dependencies = {
              serde = {
                features = [ ];
              };
            };
            other-features = [ ];
          };
          std = {
            dependencies = { };
            other-features = [ ];
          };
        };
        feature-dependencies = builtins.foldl' (
          acc: { dependencies, other-features }: acc // dependencies
        ) { } (builtins.attrValues features);

        tomlize =
          set:
          pkgs.lib.strings.concatLines (
            builtins.filter (s: !builtins.isNull s) (
              builtins.attrValues (
                builtins.mapAttrs (k: v: if builtins.isNull v then null else "${k} = \"${v}\"") set
              )
            )
          );

        cargo-lock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
        dependency-versions = builtins.listToAttrs (
          builtins.map (dependency: {
            inherit (dependency) name;
            value = dependency.version;
          }) cargo-lock.package
        );

        cargo-toml = "Cargo.toml";
        registry-keywords = [
          "no_std"
          "no-std"
          "testing"
          "math"
          "mathematics"
        ];
        registry-categories = [
          "data-structures"
          "development-tools"
          "mathematics"
          "no-std"
          "no-std::no-alloc"
        ];
        override-lints = {
          empty-enum = "allow";
          field-scoped-visibility-modifiers = "allow";
          float-arithmetic = "allow";
          implicit-return = "allow";
          inline-always = "allow";
          map-err-ignore = "allow";
          min-ident-chars = "allow";
          multiple-crate-versions = "allow";
          needless-borrowed-reference = "allow";
          pub-use = null;
          pub-with-shorthand = "allow";
          question-mark-used = "allow";
          redundant-pub-crate = "allow";
          ref-patterns = "allow";
          semicolon-outside-block = "allow";
          separated-literal-suffix = "allow";
          single-char-lifetime-names = "allow";
          tail-expr-drop-order = "warn";
          unknown-lints = "allow";
          unneeded-field-pattern = "allow";
          unqualified-local-imports = null;
          unsafe-code = "allow";
          unstable-features = "allow";
          warnings = "warn";
          wildcard-dependencies = "allow";
        };
        cargo-toml-contents = ''
          [package]
          name = "${pname}"
          version = "${version}"
          edition = "2024"
          publish = true
          authors = [ "Will Sturgeon" ]
          description = "${description}"
          readme = "README.md"
          homepage = "https://github.com/wrsturgeon/${pname}"
          repository = "https://github.com/wrsturgeon/${pname}"
          license = "MPL-2.0"
          keywords = [ "${pkgs.lib.strings.concatStringsSep "\", \"" registry-keywords}" ]
          categories = [ "${pkgs.lib.strings.concatStringsSep "\", \"" registry-categories}" ]

          [dependencies]
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                pkg: attrs:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then dependency-versions.${pkg} else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") attrs.features)
                } ]${if attrs ? git then ", git = \"${attrs.git}\"" else ""} }"
              ) dependencies
            )
          )}
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                pkg: attrs:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then dependency-versions.${pkg} else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") attrs.features)
                } ]${if attrs ? git then ", git = \"${attrs.git}\"" else ""}, optional = true }"
              ) feature-dependencies
            )
          )}
          [dev-dependencies]
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                pkg: attrs:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then dependency-versions.${pkg} else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") attrs.features)
                } ]${if attrs ? git then ", git = \"${attrs.git}\"" else ""} }"
              ) dev-dependencies
            )
          )}
          [features]
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                k:
                { dependencies, other-features }:
                "${k} = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (
                    builtins.map (s: "\"${s}\"") (
                      other-features ++ builtins.map (s: "dep:${s}") (builtins.attrNames dependencies)
                    )
                  )
                } ]"
              ) features
            )
          )}
        '';
        update-cargo-toml = "echo ${pkgs.lib.strings.escapeShellArg cargo-toml-contents} > ${cargo-toml}";

        full-src = pkgs.stdenvNoCC.mkDerivation {
          pname = "full-src";
          inherit version;
          inherit src;
          buildPhase = update-cargo-toml;
          installPhase = "cp -r . $out";
        };

        treefmt = treefmt-nix.lib.evalModule pkgs ./.treefmt.nix;

      in
      {
        apps =
          builtins.mapAttrs
            (name: script: {
              meta = { };
              type = "app";
              program =
                let
                  full-script = ''
                    #!${pkgs.bash}/bin/bash
                    set -eu
                    set -o pipefail
                    ${script}
                  '';
                  written = pkgs.writeScriptBin name full-script;
                in
                "${written}/bin/${name}";
            })
            {
              inherit update-cargo-toml;

              miri = ''
                export QUICKCHECK_TESTS=10
                cargo miri test
                cargo miri test --release
                cargo miri test --all-features
                cargo miri test --all-features --release
              '';

              test = ''
                cargo test
                cargo test --release
                cargo test --all-features
                cargo test --all-features --release
              '';

              update-other-cargo-files =
                let
                  rust-toolchain-toml = ''
                    [toolchain]
                    channel = "nightly"
                  '';
                in
                ''
                  echo ${pkgs.lib.strings.escapeShellArg rust-toolchain-toml} > rust-toolchain.toml
                '';
              clippy = ''
                set +e
                ${pkgs.ripgrep}/bin/rg 'let &' --iglob='!flake\.nix'
                if [ "$?" -eq 0 ]
                then
                  echo 'Found `let &`. Exiting as an error.'
                  exit 1
                fi
                set -ex

                # No features:
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features --release
                ${
                  if features ? std then
                    ''
                      # No features except the standard library:
                      ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features --features=std
                      ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features --features=std --release
                    ''
                  else
                    ""
                }
                # All features that don't use the standard library:
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features --features=${
                  builtins.concatStringsSep "," (
                    builtins.filter (f: f != "std" && !(builtins.any (f: f == "std") features.${f}.other-features)) (
                      builtins.attrNames features
                    )
                  )
                }
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --no-default-features --features=${
                  builtins.concatStringsSep "," (
                    builtins.filter (f: f != "std" && !(builtins.any (f: f == "std") features.${f}.other-features)) (
                      builtins.attrNames features
                    )
                  )
                } --release
                # All features, including those that might use the standard library:
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --all-features
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --all-features --release
              '';
            };
        packages.default = rust-platform.buildRustPackage (
          ENV
          // (
            let
              src = full-src;
            in
            {
              inherit pname src;
              name = pname;
              cargoLock.lockFile = "${src}/Cargo.lock";
            }
          )
        );
        devShells.default = pkgs.mkShell (
          ENV
          // {
            inputsFrom = builtins.attrValues self.packages.${system};
            packages = with pkgs; [
              full-toolchain
              lldb
              rust-analyzer
            ];
          }
        );
        formatter = treefmt.config.build.wrapper;
      }
    );
}
