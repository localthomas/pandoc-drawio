# SPDX-FileCopyrightText: 2022 localthomas
# SPDX-License-Identifier: MIT OR Apache-2.0

# Back-compat for using nix-shell with this file instead of nix develop.
# References the flakes.lock file, and the flake-compat hash in there for easier updates.
# See https://nixos.wiki/wiki/Flakes#Using_flakes_project_from_a_legacy_Nix for more information.
(import
  (
    let
      lock = builtins.fromJSON (builtins.readFile ./flake.lock);
    in
    fetchTarball {
      url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
      sha256 = lock.nodes.flake-compat.locked.narHash;
    }
  )
  {
    src = ./.;
  }).shellNix
