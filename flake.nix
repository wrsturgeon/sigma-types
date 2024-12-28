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
  };
  outputs =
    {
      fenix,
      flake-utils,
      nix-filter,
      nixpkgs,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pname = "sigma-types";
        version = "0.2.0";
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
          RUST_BACKTRACE = "full";
          RUST_LOG = "debug";
        };

        dependencies = { };
        dev-dependencies = {
          quickcheck = [ ];
          serde_json = [ "std" ];
        };
        features = {
          malachite = {
            dependencies = {
              malachite-base = [ ];
            };
            other-features = [ ];
          };
          serde = {
            dependencies = {
              serde = [ ];
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
                pkg: features:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then "=${dependency-versions."${pkg}"}" else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") features)
                } ] }"
              ) dependencies
            )
          )}
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                pkg: features:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then "=${dependency-versions."${pkg}"}" else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") features)
                } ], optional = true }"
              ) feature-dependencies
            )
          )}
          [dev-dependencies]
          ${pkgs.lib.strings.concatLines (
            builtins.attrValues (
              builtins.mapAttrs (
                pkg: features:
                "${pkg} = { version = \"${
                  if builtins.hasAttr pkg dependency-versions then "=${dependency-versions."${pkg}"}" else "*"
                }\", default-features = false, features = [ ${
                  pkgs.lib.strings.concatStringsSep ", " (builtins.map (feature: "\"${feature}\"") features)
                } ] }"
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
          [lints.rust]
          ${tomlize (
            builtins.mapAttrs (
              k: v: if builtins.hasAttr k override-lints then override-lints.${k} else "deny"
            ) (import "${self.packages.${system}.all-lints}/rustc.nix")
          )}
          [lints.clippy]
          ${tomlize (
            builtins.mapAttrs (
              k: v: if builtins.hasAttr k override-lints then override-lints.${k} else "deny"
            ) (import "${self.packages.${system}.all-lints}/clippy.nix")
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

      in
      {
        apps =
          builtins.mapAttrs
            (name: script: {
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
                  exit 1
                fi
                set -e

                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --release
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --all-features
                ${full-toolchain}/bin/cargo-clippy -- --all-targets --color=always --all-features --release
              '';
            };
        packages = {
          all-lints = pkgs.stdenvNoCC.mkDerivation {
            name = "all-lints";
            src = nix-filter {
              root = ./.;
              include = [ ];
            };
            buildPhase = ":";
            installPhase =
              let
                binaries = builtins.mapAttrs (name: from: "${from}/bin/${name}") {
                  cut = pkgs.coreutils;
                  grep = pkgs.gnugrep;
                  head = pkgs.coreutils;
                  sort = pkgs.coreutils;
                  tail = pkgs.coreutils;
                  tr = pkgs.coreutils;
                };
                clippy-help-cmd = "cargo clippy --no-deps --offline -- -Zunstable-options -W help > \${out}/clippy-help.txt 2>&1";
                check-empty = file: ''
                  if [ ! -s "${file}" ]
                  then
                    echo '***** ERROR: empty `${file}` *****'
                    for f in $(find . -name '*.txt')
                    do
                      echo
                      echo 'contents of `'"''${f}"'`:'
                      cat "''${f}"
                    done
                    exit 1
                  fi
                '';
              in
              with binaries;
              ''
                set -eu
                set -o pipefail

                mkdir -p src
                echo ${pkgs.lib.strings.escapeShellArg ''
                  cargo-features = [ "edition2024" ]

                  [package]
                  name = "all-lints"
                  version = "0.1.0"
                  edition = "2024"
                ''} > Cargo.toml
                touch src/lib.rs

                set +e
                mkdir -p ''${out}
                ${clippy-help-cmd}
                if [ "$?" -ne '0' ]
                then
                  echo '***** ERROR (${clippy-help-cmd}) *****'
                  echo 'dump:'
                  cat ''${out}/clippy-help.txt
                  exit 1
                fi

                mkdir clippy
                cd clippy
                cat ''${out}/clippy-help.txt | ${grep} -A9999 -m1 -e '^Lint checks loaded by this crate:$' > after-lint-checks.txt
                ${check-empty "after-lint-checks.txt"}
                cat after-lint-checks.txt | ${grep} -A9999 -m1 -e '-------' > after-line.txt
                ${check-empty "after-line.txt"}
                rm after-lint-checks.txt
                cat after-line.txt | ${tail} -n+2 > after-two-more.txt
                ${check-empty "after-two-more.txt"}
                rm after-line.txt
                cat after-two-more.txt | ${grep} -B9999 -m1 -e '^$' > before-blank-line.txt
                ${check-empty "before-blank-line.txt"}
                rm after-two-more.txt
                cat before-blank-line.txt | ${head} -n-1 > before-one-more.txt
                ${check-empty "before-one-more.txt"}
                rm before-blank-line.txt
                cat before-one-more.txt | ${cut} -d ':' -f 3- > after-colon.txt
                ${check-empty "after-colon.txt"}
                rm before-one-more.txt
                cat after-colon.txt | ${tr} -s ' ' > single-space.txt
                ${check-empty "single-space.txt"}
                rm after-colon.txt
                cat single-space.txt | ${cut} -d ' ' -f -2 > first-two-columns.txt
                ${check-empty "first-two-columns.txt"}
                rm single-space.txt
                cat first-two-columns.txt | ${sort} > ''${out}/sorted-clippy.txt
                ${check-empty "\${out}/sorted-clippy.txt"}
                rm first-two-columns.txt
                if [ ! -s ''${out}/sorted-clippy.txt ]
                then
                  echo
                  echo '*******************************************'
                  echo '***** ERROR: no `clippy` lints found! *****'
                  echo '*******************************************'
                  echo
                  for f in $(find . -name '*.txt')
                  do
                    echo
                    echo 'contents of `'"''${f}"'`:'
                    cat "''${f}"
                  done
                  exit 1
                fi
                cd ..

                mkdir rustc
                cd rustc
                cat ''${out}/clippy-help.txt | ${grep} -A9999 -m1 -e '^Lint checks provided by rustc:$' > after-lint-checks.txt
                ${check-empty "after-lint-checks.txt"}
                cat after-lint-checks.txt | ${grep} -A9999 -m1 -e '-------' > after-line.txt
                ${check-empty "after-line.txt"}
                rm after-lint-checks.txt
                cat after-line.txt | ${tail} -n+2 > after-two-more.txt
                ${check-empty "after-two-more.txt"}
                rm after-line.txt
                cat after-two-more.txt | ${grep} -B9999 -m1 -e '^$' > before-blank-line.txt
                ${check-empty "before-blank-line.txt"}
                rm after-two-more.txt
                cat before-blank-line.txt | ${head} -n-1 > before-one-more.txt
                ${check-empty "before-one-more.txt"}
                rm before-blank-line.txt
                cat before-one-more.txt | ${tr} -s ' ' > single-space.txt
                ${check-empty "single-space.txt"}
                rm before-one-more.txt
                cat single-space.txt | sed -e 's/^[[:space:]]*//' > without-initial-space.txt
                ${check-empty "without-initial-space.txt"}
                rm single-space.txt
                cat without-initial-space.txt | ${cut} -d ' ' -f -2 > first-two-columns.txt
                ${check-empty "first-two-columns.txt"}
                rm without-initial-space.txt
                cat first-two-columns.txt | ${sort} > ''${out}/sorted-rustc.txt
                ${check-empty "\${out}/sorted-rustc.txt"}
                rm first-two-columns.txt
                if [ ! -s ''${out}/sorted-rustc.txt ]
                then
                  echo
                  echo '*******************************************'
                  echo '***** ERROR: no `rustc` lints found! *****'
                  echo '*******************************************'
                  echo
                  for f in $(find . -name '*.txt')
                  do
                    echo
                    echo 'contents of `'"''${f}"'`:'
                    cat "''${f}"
                  done
                  exit 1
                fi
                cd ..

                set -e

                echo '{' > ''${out}/clippy.nix
                while read line
                do
                  echo '  '$(echo "''${line}" | cut -d ' ' -f 1)' = "'$(echo "''${line}" | cut -d ' ' -f 2)'";' >> ''${out}/clippy.nix
                done < ''${out}/sorted-clippy.txt
                echo '}' >> ''${out}/clippy.nix

                echo '{' > ''${out}/rustc.nix
                while read line
                do
                  echo '  '$(echo "''${line}" | cut -d ' ' -f 1)' = "'$(echo "''${line}" | cut -d ' ' -f 2)'";' >> ''${out}/rustc.nix
                done < ''${out}/sorted-rustc.txt
                echo '}' >> ''${out}/rustc.nix
              '';
            nativeBuildInputs = [
              (fenix.packages.${system}.complete.withComponents [
                "cargo"
                "clippy"
              ])
            ];
          };
          default = rust-platform.buildRustPackage (
            ENV
            // (
              let
                src = full-src;
              in
              {
                inherit pname src;
                name = pname;
                cargoLock.lockFile = "${src}/Cargo.lock";
                buildInputs = with pkgs; [ openssl ];
                nativeBuildInputs = with pkgs; [
                  google-chrome
                  openssl
                  pkg-config
                  undetected-chromedriver
                  zlib
                ];
              }
            )
          );
        };
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
      }
    );
}
