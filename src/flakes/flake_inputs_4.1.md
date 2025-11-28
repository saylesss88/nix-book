---
title: Flake Inputs
date: 2025-11-28
author: saylesss88
collection: blog
tags: ["nixos", "flakes"]
draft: false
---

# Nix Flake Inputs

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

The attribute `inputs` specifies the dependencies of a flake, as an attrset
mapping input names to flake references.

If a repository provides a `flake.nix` you can include it as an input in your
`flake.nix`.

For example, I like yazi as my file explorer and have been using helix as my
editor. To be able to get yazi to work with helix I needed the latest versions
of both yazi and helix. One way to get the latest versions was to add their
flakes as inputs to my flake:

```nix
{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
		home-manager = {
			url = "github:nix-community/home-manager/release-24.11";
			inputs.nixpkgs.follows = "nixpkgs";
		};
    helix = {
      url = "github:helix-editor/helix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
		yazi.url = "github:sxyazi/yazi";
	};
	outputs = { nixpkgs, home-manager, ... } @ inputs: {
	# ... snip ... #
```

- Now to use this input, I would reference these inputs in both my yazi and
  helix modules:

```nix
# yazi.nix
{ pkgs, config, inputs, ... }: {
	programs.yazi = {
		enable = true;
		package = inputs.yazi.packages.${pkgs.system}.default;
	};
}
```

```nix
# helix.nix
{ pkgs, config, inputs, ... }: {
	programs.helix = {
		enable = true;
		package = inputs.helix.packages.${pkgs.system}.helix;
	};
}
```

Understanding `.default` vs. Named Outputs (e.g., `.helix`) from the Source

The difference between `inputs.yazi.packages.${pkgs.system}.default` and
`inputs.helix.packages.${pkgs.system}.helix` comes down to how the respective
upstream flakes define their outputs. You can always inspect a flake's
`flake.nix` or use `nix flake show <flake-reference>` to understand its
structure.

## Helix `flake.nix`

Let's look at the relevant section of Helix's `flake.nix` click the eye to see
the full flake:

```nix
~ {
~   description = "A post-modern text editor.";
~
~   inputs = {
~     nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
~     rust-overlay = {
~       url = "github:oxalica/rust-overlay";
~       inputs.nixpkgs.follows = "nixpkgs";
~     };
~   };
~
~   outputs = {
~     self,
~     nixpkgs,
~     rust-overlay,
~     ...
~   }: let
~     inherit (nixpkgs) lib;
~     systems = [
~       "x86_64-linux"
~       "aarch64-linux"
~       "x86_64-darwin"
~       "aarch64-darwin"
~     ];
~     eachSystem = lib.genAttrs systems;
~     pkgsFor = eachSystem (system:
~       import nixpkgs {
~         localSystem.system = system;
~         overlays = [(import rust-overlay) self.overlays.helix];
~       });
~     gitRev = self.rev or self.dirtyRev or null;
   in {
     packages = eachSystem (system: {
       inherit (pkgsFor.${system}) helix;
       /*
       The default Helix build. Uses the latest stable Rust toolchain, and unstable
       nixpkgs.

       The build inputs can be overridden with the following:

       packages.${system}.default.override { rustPlatform = newPlatform; };

       Overriding a derivation attribute can be done as well:

       packages.${system}.default.overrideAttrs { buildType = "debug"; };
       */
      default = self.packages.${system}.helix;
    });
~    checks =
~      lib.mapAttrs (system: pkgs: let
~        # Get Helix's MSRV toolchain to build with by default.
~        msrvToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
~        msrvPlatform = pkgs.makeRustPlatform {
~          cargo = msrvToolchain;
~          rustc = msrvToolchain;
~        };
~      in {
~        helix = self.packages.${system}.helix.override {
~          rustPlatform = msrvPlatform;
~        };
~      })
~      pkgsFor;
~
~    # Devshell behavior is preserved.
~    devShells =
~      lib.mapAttrs (system: pkgs: {
~        default = let
~          commonRustFlagsEnv = "-C link-arg=-fuse-ld=lld -C target-cpu=native --cfg tokio_unstable";
~          platformRustFlagsEnv = lib.optionalString pkgs.stdenv.isLinux "-Clink-arg=-Wl,--no-rosegment";
~        in
~          pkgs.mkShell {
~            inputsFrom = [self.checks.${system}.helix];
~            nativeBuildInputs = with pkgs;
~              [
~                lld
~                cargo-flamegraph
~                rust-bin.nightly.latest.rust-analyzer
~              ]
~              ++ (lib.optional (stdenv.isx86_64 && stdenv.isLinux) cargo-tarpaulin)
~              ++ (lib.optional stdenv.isLinux lldb)
~              ++ (lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.CoreFoundation);
~            shellHook = ''
~              export RUST_BACKTRACE="1"
~              export RUSTFLAGS="''${RUSTFLAGS:-""} ${commonRustFlagsEnv} ${platformRustFlagsEnv}"
~            '';
~          };
~      })
~      pkgsFor;
~
~    overlays = {
~      helix = final: prev: {
~        helix = final.callPackage ./default.nix {inherit gitRev;};
~      };
~
~      default = self.overlays.helix;
~    };
~  };
~  nixConfig = {
~    extra-substituters = ["https://helix.cachix.org"];
~    extra-trusted-public-keys = ["helix.cachix.org-1:ejp9KQpR1FBI2onstMQ34yogDm4OgU2ru6lIwPvuCVs="];
~  };
~}
```

Dissecting `inherit (pkgsFor.${system}) helix;`

Imagine the Nix evaluation process for Helix `flake.nix` in the `outputs`
section:

1. `packages = eachSystem (system: { ... });` Part iterates through each
   `system` (like `x86_64-linux`). For each `system`, it's creating an attribute
   set that will become `self.packages.${system}`.

2. Inside the `eachSystem` function, for a specific system (e.g.
   `x86_64-linux`): The code is building an attribute set that will ultimately
   be assigned to `self.packages.x86_64-linux`.

3. When you write `inherit (sourceAttrset) attributeName;`, it's equivalent to
   writing `attributeName = sourceAttrset.attributeName;`.

So, `inherit (pkgsFor.${system}) helix;` is equivalent to:

```nix
helix = pkgsFor.${system}.helix;
```

Therefore, because of `inherit (pkgsFor.${system}) helix;`, the helix attribute
is explicitly defined under
`packages.${system}``. This is why you access it as `inputs.helix.packages.${pkgs.system}.helix;`.

## Yazi `flake.nix`

Now this is yazi's `flake.nix`, yazi's documentation tells you to use `.default`
but lets examine the flake and see why:

```nix
~{
~  inputs = {
~    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
~    flake-utils.url = "github:numtide/flake-utils";
~    rust-overlay = {
~      url = "github:oxalica/rust-overlay";
~      inputs.nixpkgs.follows = "nixpkgs";
~    };
~  };
~
~  outputs =
~    {
~      self,
~      nixpkgs,
~      rust-overlay,
~      flake-utils,
~      ...
~    }:
~    flake-utils.lib.eachDefaultSystem (
~      system:
~      let
~        pkgs = import nixpkgs {
~          inherit system;
~          overlays = [ rust-overlay.overlays.default ];
~        };
~        toolchain = pkgs.rust-bin.stable.latest.default;
~        rustPlatform = pkgs.makeRustPlatform {
~          cargo = toolchain;
~          rustc = toolchain;
~        };
~
~        rev = self.shortRev or self.dirtyShortRev or "dirty";
~        date = self.lastModifiedDate or self.lastModified or "19700101";
~        version =
~          (builtins.fromTOML (builtins.readFile ./yazi-fm/Cargo.toml)).package.version
~          + "pre${builtins.substring 0 8 date}_${rev}";
~      in
      {
        packages = {
          yazi-unwrapped = pkgs.callPackage ./nix/yazi-unwrapped.nix {
            inherit
              version
              rev
              date
              rustPlatform
              ;
          };
          yazi = pkgs.callPackage ./nix/yazi.nix { inherit (self.packages.${system}) yazi-unwrapped; };
          default = self.packages.${system}.yazi;
        };

~        devShells = {
~          default = pkgs.callPackage ./nix/shell.nix { };
~        };
~
~        formatter = pkgs.nixfmt-rfc-style;
~      }
~    )
~    // {
~      overlays = {
~        default = self.overlays.yazi;
~        yazi = _: prev: { inherit (self.packages.${prev.stdenv.system}) yazi yazi-unwrapped; };
~      };
~    };
~}
```

In this case using `inputs.yazi.packages.${pkgs.system}.yazi` would also work

- `yazi = pkgs.callPackage ./nix/yazi.nix { inherit (self.packages.${system}) yazi-unwrapped; };`
  This line defines the yazi variable (or, more precisely, creates an attribute
  named yazi within the `packages.${system}` set). It assigns to this yazi
  attribute the result of calling the Nix expression in `./nix/yazi.nix` with
  yazi-unwrapped as an argument. This yazi attribute represents the actual,
  runnable Yazi package.

- `default = self.packages.${system}.yazi;` This line then aliases the yazi
  package. It creates another attribute named `default` within the same
  `packages.${system}` set and points it directly to the yazi attribute that was
  just defined.

- So, when you access `inputs.yazi.packages.${pkgs.system}.default`, you're
  effectively following the alias to the yazi package.

- The choice to use `.default` is primarily for convenience and adherence to a
  common flake convention, making the flake easier for users to consume without
  needing to dive into its internal structure.
