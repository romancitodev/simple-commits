let
  inherit
    (builtins)
    currentSystem
    fromJSON
    readFile
    ;
  getFlake = name:
    with (fromJSON (readFile ../flake.lock)).nodes.${name}.locked; {
      inherit rev;
      outPath = fetchTarball {
        url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
        sha256 = narHash;
      };
    };
in
{ system ? currentSystem
, pkgs ? import (getFlake "nixpkgs") { localSystem = { inherit system; }; }
, lib ? pkgs.lib
, crane
, cranix
, fenix
, ...
}:
let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.${system}.overrideToolchain toolchain;
  # cranix: extends crane building system with workspace bin building and Mold + Cranelift integrations
  cranixLib = craneLib.overrideScope (cranix.${system}.craneOverride);

  deps = {
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
  };

  # Lambda for build packages with cached artifacts
  commonArgs = targetName:
    deps //
    {
      src = lib.cleanSourceWith {
        src = craneLib.path ./.;
        filter = craneLib.filterCargoSources;
      };
      doCheck = false;
    };

  # Build packages and `nix run` apps
  simpleCommitsPkg = cranixLib.buildCranixBundle (commonArgs "simple-commits");
in
{
  # `nix run`
  apps = rec {
    simpleCommits = simpleCommitsPkg.app;
    default = simpleCommits;
  };
  # `nix build`
  packages = rec {
    simpleCommits = simpleCommitsPkg.pkg;
    default = simpleCommits;
  };
  # `nix develop`
  devShells.default = cranixLib.devShell {
    packages = with pkgs; [
      toolchain
      cargo-release
      cargo-dist
    ] ++ deps.buildInputs;
  };
}
