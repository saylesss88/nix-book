---
title: Overlays
date: 2025-11-22
author: saylesss88
collection: blog
tags: ["nixos", "overlays", "outputs"]
draft: false
---

# Extending Flakes with Custom Packages using Overlays

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![Pokego Logo](../images/pokego.png)--[pokego repo](https://github.com/rubiin/pokego)

Overlays are Nix functions that accept two arguments, `final` and `prev` and
return a set of packages. Overlays are similar to `packageOverrides` as a way to
customize Nixpkgs, `packageOverrides` acts as an overlay with only the `prev`
argument. Therefore, `packageOverrides` is appropriate for basic use, but
overlays are more powerful and easier to distribute.

Example:

```nix
final: prev: {
  firefox = prev.firefox.overrideAttrs (old: {
    buildInputs = (old.buildInputs or []) ++ [ prev.vlc ];
    env.FIREFOX_DISABLE_GMP_UPDATER = "1";
  });
}
```

To see the original derivation, run `nix edit -f "<nixpkgs>" firefox`.

This modifies Firefox by:

- Adding `vlc` to `buildInputs`, useful if a package requires additional
  dependencies.

- Setting an environment variable (`FIREFOX_DISABLE_GMP_UPDATER=1`) to disable
  automatic updates of the Gecko Media Plugin.

It is very common to use overlays in Nix to install packages that aren't
available in the standard Nixpkgs repository.

**Overlays** are one of the primary and recommended ways to extend and customize
your Nix environment. It's important to remember that Nix overlays are made to
allow you to modify or extend the package set provided by Nixpkgs (or other Nix
sources) without directly altering the original package definitions. This is
crucial for maintaining reproducibility and avoiding conflicts. Overlays are
essentially functions that take the previous package set and allow you to add,
modify, or remove packages.

- To better understand the structure of my `flake.nix` it may be helpful to
  first read [This](https://tsawyer87.github.io/posts/nix_flakes_tips/) blog
  post first.

## Adding the overlays output to your Flake

I'll show the process of adding the `pokego` package that is not in Nixpkgs:

1. In my `flake.nix` I have a custom inputs variable within my let block of my
   flake like so just showing the necessary parts for brevity:

```nix
# flake.nix
  outputs = my-inputs @ {
    self,
    nixpkgs,
    treefmt-nix,
    ...
  }: let
    system = "x86_64-linux";
    host = "magic";
    userVars = {
      username = "jr";
      gitUsername = "saylesss88";
      editor = "hx";
      term = "ghostty";
      keys = "us";
      browser = "firefox";
      flake = builtins.getEnv "HOME" + "/flake";
    };

    inputs =
      my-inputs
      // {
        pkgs = import inputs.nixpkgs {
          inherit system;
        };
        lib = {
          overlays = import ./lib/overlay.nix;
          nixOsModules = import ./nixos;
          homeModules = import ./home;
          inherit system;
        };
      };
      # ... snip ...
```

- Why I Created `inputs.lib` in My `flake.nix`. In the above example, you'll
  notice a `lib` attribute defined within the main `let` block.
  - This might seem a bit unusual at first, as inputs are typically defined at
    the top level of a flake. However, this structure provides a powerful way to
    organize and reuse common Nix functions and configurations across my flake.

  - By bundling my custom logic and modules into `inputs.lib`, I can pass
    `inputs` (which now includes my custom `lib`) as a `specialArgs` to other
    modules. This provides a clean way for all modules to access these shared
    resources. For example, in `configuration.nix`, `inputs.lib.overlays`
    directly references my custom overlay set.

  - My `inputs.lib` is my own project-specific library, designed to hold
    functions and attribute sets relevant to my flake's custom configurations.
    While `nixpkgs.lib` is globally available, my custom `lib` contains my
    unique additions.

While defining `inputs` within the `let` block to achieve this structure is a
personal preference and works well for my setup, the core benefit is the
creation of a dedicated, centralized `lib` attribute that encapsulates my
flake's reusable Nix code, leading to a more organized and maintainable
configuration.

## The Actual Overlay

2. In the `overlay.nix` I have this helper function and the defined package:

```nix
# overlay.nix
_final: prev: let
  # Helper function to import a package
  callPackage = prev.lib.callPackageWith (prev // packages);

  # Define all packages
  packages = {
    # Additional packages
    pokego = callPackage ./pac_defs/pokego.nix {};
  };
in
  packages
```

1. `_final: prev:`: This is the function definition of the overlay.

- `_final`: This argument represents the final, merged package set after all
  overlays have been applied. It's often unused within a single overlay, hence
  the `_` prefix (a Nix convention for unused variables).

- `prev`: This is the crucial argument. It represents the package set before
  this overlay is applied. This allows you to refer to existing packages and
  functions from Nixpkgs.

2. `let ... in packages`: This introduces a `let` expression, which defines
   local variables within the scope of this overlay function. The `in packages`
   part means that the overlay function will ultimately return the `packages`
   attribute set defined within the `let` block.

3. `callPackage = prev.lib.callPackageWith (prev // packages)`: This line
   defines a helper function called `callPackage`.

- `prev.lib.callPackageWith` Is a function provided by Nixpkgs' `lib`.
  `callPackageWith` is like `prev.lib.callPackage`, but allows the passing of
  additional arguments that will then be passed to the package definition.

- `(prev // packages)`: This is an attribute set merge operation. It takes the
  `prev` package set (Nixpkgs before this overlay) and merges it with the
  `packages` attribute set defined later in this overlay.

- By using `callPackageWith` with this merged attribute set, the `callPackage`
  function defined here is set up to correctly import package definitions,
  ensuring they have access to both the original Nixpkgs and any other packages
  defined within this overlay.

4. `packages = { ... };`: This defines an attribute set named `packages`. This
   set will contain all the new or modified packages introduced by this overlay.

5. `pokego = callPackages ./pac_defs/pokego.nix { };`: This is the core of how
   the `pokego` package is added.

- `pokego =`: This defines a new attribute named `pokego` within the packages
  attribute set. This name will be used to refer to the pokego package later.

- `callPackage ./pac_defs/pokego.nix {}`: This calls the callPackage helper
  function defined earlier.

- `./pac_defs/pokego.nix`: This is the path to another Nix file(`pokego.nix`)
  that contains the actual package definition for pokego. This file would define
  how to fetch, build, and install the pokego software

- `{}`: This is an empty attribute set passed as additional arguments to the
  `pokego.nix` package definition. If `pokego.nix` expected any specific
  parameters (like versions or dependencies), you would provide them here. Since
  it's empty, it implies pokego.nix either has no required arguments or uses
  default values.

6. `in packages`: As mentioned earlier, the overlay function returns the
   packages attribute set. When this overlay is applied, the packages defined
   within this packages set (including pokego) will be added to the overall Nix
   package set.

## The pokego Package definition

The following is the `./pac_defs/pokego.nix`:

```nix
# pokego.nix
{
  lib,
  buildGoModule,
  fetchFromGitHub,
}:
buildGoModule rec {
  pname = "pokego";
  version = "0.3.0";

  src = fetchFromGitHub {
    owner = "rubiin";
    repo = "pokego";
    rev = "v${version}";
    hash = "sha256-cFpEi8wBdCzAl9dputoCwy8LeGyK3UF2vyylft7/1wY=";
  };

  vendorHash = "sha256-7SoKHH+tDJKhUQDoVwAzVZXoPuKNJEHDEyQ77BPEDQ0=";

  # Install shell completions
  postInstall = ''
    install -Dm644 completions/pokego.bash "$out/share/bash-completion/completions/pokego"
    install -Dm644 completions/pokego.fish "$out/share/fish/vendor_completions.d/pokego.fish"
    install -Dm644 completions/pokego.zsh "$out/share/zsh/site-functions/_pokego"
  '';

  meta = with lib; {
    description = "Command-line tool that lets you display Pokémon sprites in color directly in your terminal";
    homepage = "https://github.com/rubiin/pokego";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [
      rubiin
      jameskim0987
      vinibispo
    ];
    mainProgram = "pokego";
    platforms = platforms.all;
  };
}
```

## Adding the overlay to your configuration

There are a few places you could choose to put the following, I choose to use my
`configuration.nix` because of my setup:

```nix
# configuration.nix
nixpkgs.overlays = [inputs.lib.overlays]
```

## Installing Pokego

- If you are managing your entire system configuration with NixOS, you would
  typically add `pokego` to your `environment.systemPackages`.

```nix
# configuration.nix
environment.systemPackages = with pkgs; [
  pokego
]
```

- If you prefer home-manager you can install `pokego` with home-manager also:

```nix
# home.nix
home.packages = [
  pkgs.pokego
]
```

### Another Overlay Example

```nix
{
  inputs = {
    nixpkgs.url = "https://flakehub.com/NixOS/nixpkgs/*.tar.gz";

    nix.url = "https://flakehub.com/f/NixOS/nix/2.17.0.tar.gz";
  };

  outputs = { self, nixpkgs, nix }:

    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          nix.overlays.default
        ];
      };
    in
    {
     # `pkgs` is nixpkgs for the system, with nix's overlay applied
    };
}
```

- Normally,
  `pkgs = import nixpkgs { }`` imports Nixpkgs with default settings.  However, the example above customizes this import by passing arguments:  `pkgs
  = import nixpkgs { inherit system; overlays = [
  nix.overlays.default];}`.  This makes the pkgs variable represent nixpkgs specifically for the `aarch64-darwin`
  system, with the overlay from the nix flake applied.

- Consequently, any packages built using this customized `pkgs` will now depend
  on or use the specific nix version (`2.17.0`) provided by the nix flake,
  instead of the version that comes with the fetched `nixpkgs`. This technique
  can be useful for ensuring a consistent environment or testing specific
  package versions.

## Customizing Nixpkgs Imports and Overlays

While overlays are typically used to add or modify packages within a single
`nixpkgs` instance, Nix's lazy evaluation and flake inputs allow for even more
powerful scenarios. You can have multiple versions of nixpkgs in a single flake,
and they will only be evaluated when a package from that specific version is
actually referenced. This complements overlays by giving you fine-grained
control over which nixpkgs instance an overlay applies to, or which `nixpkgs`
version a specific part of your project depends on.

Consider this example where we import nixpkgs with a specific overlay applied
directly at the import site:

```nix
{
  inputs = {
    nixpkgs.url = "[https://flakehub.com/NixOS/nixpkgs/*.tar.gz](https://flakehub.com/NixOS/nixpkgs/*.tar.gz)"; # This will be the base nixpkgs

    nix.url = "[https://flakehub.com/f/NixOS/nix/2.17.0.tar.gz](https://flakehub.com/f/NixOS/nix/2.17.0.tar.gz)"; # This flake provides an overlay for a specific Nix version
  };

  outputs = { self, nixpkgs, nix }:

    let
      system = "aarch64-darwin";
      # Here, we import nixpkgs and apply the 'nix' flake's overlay.
      # This 'pkgs' variable now holds a customized Nix package set.
      # In this 'pkgs' set, the 'nix' package (and anything that depends on it)
      # will be version 2.17.0 as defined by the 'nix' flake's overlay.
      pkgs_with_custom_nix = import nixpkgs {
        inherit system;
        overlays = [
          nix.overlays.default # Apply the overlay from the 'nix' flake here
        ];
      };
    in
    {
      # We can then expose packages or devShells that use this customized `pkgs` set.
      devShells.${system}.default = pkgs_with_custom_nix.mkShell {
        packages = [
          pkgs_with_custom_nix.nix # This 'nix' package is now version 2.17.0 due to the overlay!
        ];
        shellHook = ''
          echo "Using Nix version: <span class="math-inline">\(nix \-\-version\)"
'';
};
# You can also make this customized package set available as a top-level overlay
# if other parts of your flake or configuration want to use it.
# overlays.custom-nix-version = final: prev: {
#   inherit (pkgs_with_custom_nix) nix; # Expose the specific nix package from our overlayed pkgs
# };
# You can also import multiple versions of nixpkgs and select packages from them:
# pkgs-2505 = import (inputs.nixpkgs-2505 or nixpkgs) { inherit system; }; # Example, assuming 2505 is an input
# packages.</span>{system}.my-tool-2505 = pkgs-2505.myTool; # Using a package from a specific stable version
    };
}
```

Normally, `pkgs = import nixpkgs { }` imports Nixpkgs with default settings.
However, the example above customizes this import by passing arguments:
`pkgs = import nixpkgs { inherit system; overlays = [ nix.overlays.default];}`.
This makes the `pkgs_with_custom_nix` variable represent Nixpkgs specifically
for the `aarch64-darwin` system, with the overlay from the nix flake applied at
the time of import.

Consequently, any packages built using this customized `pkgs_with_custom_nix`
will now depend on or use the specific Nix version (`2.17.0`) provided by the
nix flake's overlay, instead of the version that comes with the base `nixpkgs`
input. This technique is highly useful for ensuring a consistent environment or
testing specific package versions without affecting the entire system's
`nixpkgs` set.
