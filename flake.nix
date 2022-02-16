# SPDX-FileCopyrightText: 2022 localthomas
# SPDX-License-Identifier: MIT OR Apache-2.0
{
  description = "A pandoc filter that converts draw.io files into common image/vector formats automatically.";

  inputs = {
    # for eachSystem function
    flake-utils.url = "github:numtide/flake-utils";
    # use flake-compat as side-effect for flake.lock file that is read by shell.nix
    # fill the flake.lock file with `nix flake lock --update-input flake-compat`
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    # get the rust toolchain
    rust-overlay.url = "github:oxalica/rust-overlay";
    # use the rust toolchain for building the binary
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        cargo-metadata = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
        crateName = cargo-metadata.package.name;

        # apply the rust-overlay to nixpkgs
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        # setup the rust toolchain based on the rust-toolchain file
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Override the version used in naersk and using the toolchain from above
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };

        # dependencies for building
        buildingDependencies = with pkgs; [
          nixpkgs-fmt
          rust
          cargo-about
        ];
      in
      with pkgs;
      {
        devShell = mkShell {
          # tools and dependencies for building and developing
          nativeBuildInputs = buildingDependencies ++ [
            # dependency for checking REUSE compliance
            reuse
            # dependencies for the test.sh script
            pandoc
            drawio
            xvfb-run
            librsvg
            texlive.combined.scheme-medium
          ];
        };

        checks = {
          format = runCommand "check-format"
            {
              nativeBuildInputs = buildingDependencies;
            }
            ''
              cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
              nixpkgs-fmt --check ${./.}
              touch $out # touch output file to give the information that check was successful
            '';
          reuse = runCommand "check-reuse"
            {
              nativeBuildInputs = [ reuse ];
            }
            ''
              reuse --root ${./.} lint
              touch $out # touch output file to give the information that check was successful
            '';
        };

        packages."${crateName}-unwrapped" = naersk-lib.buildPackage {
          pname = crateName;
          root = ./.;
          # The packages of the devShell are re-used for building
          nativeBuildInputs = buildingDependencies;
          # Configures the target which will be built.
          # ref: https://doc.rust-lang.org/cargo/reference/config.html#buildtarget
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS = "-C target-feature=+crt-static";

          # only build if clippy does not generate warnings
          cargoTestCommands = default: [ ''cargo clippy --locked --offline -- --deny "warnings"'' ] ++ default;
          doCheck = true;
        };

        packages.${crateName} = stdenv.mkDerivation {
          name = crateName;
          src = self;
          installPhase = ''
            mkdir -p $out/bin;

            echo "#!${bash}/bin/bash" > $out/bin/${crateName};
            echo 'export ELECTRON_DISABLE_SECURITY_WARNINGS="true"' >> $out/bin/${crateName};
            echo '${self.packages.${system}."${crateName}-unwrapped"}/bin/${crateName} --drawio-cmd ${drawio}/bin/drawio --xvfb-run-cmd ${xvfb-run}/bin/xvfb-run "''$@"' >> $out/bin/${crateName};
            chmod +x $out/bin/${crateName}
          '';
        };

        defaultPackage = self.packages.${system}.${crateName};

        defaultApp = { type = "app"; program = "${self.defaultPackage.${system}}/bin/${crateName}"; };
      }
    );
}
