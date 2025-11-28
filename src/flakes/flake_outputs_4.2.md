---
title: Flake outputs
date: 2025-11-28
author: saylesss88
collection: blog
tags: ["nixos", "flakes", "outputs"]
draft: false
---

# Nix Flake Outputs

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

Flake outputs are what the flake produces when built. Flakes can have multiple
outputs simultaneously such as:

- **Packages**: Self-contained bundles that are built using derivations and
  provide either some kind of software or dependencies of software.

- [NixOS modules](https://saylesss88.github.io/NixOS_Modules_Explained_3.html)

- Nix development environments

- [Nix templates](https://github.com/NixOS/templates)

- The `outputs` top-level attribute is actually a function that takes an
  attribute set of inputs and returns an attribute set that is essentially a
  recipe for building the flake.

## Output Schema

Once the inputs are resolved, they're passed to the `outputs` attribute. This
`outputs` attribute is, in fact, a function, as indicated by the `:` colon (or
the `lambda` syntax) that follows its definition. This function takes the
resolved inputs (and `self`, the flake's directory in the store) as arguments,
and its return value dictates the outputs of the flake, following this schema:

```nix
{ self, nixpkgs, ... }@inputs:
{
  # Executed by `nix flake check`
  checks."<system>"."<name>" = derivation;
  # Executed by `nix build .#<name>`
  packages."<system>"."<name>" = derivation;
  # Executed by `nix build .`
  packages."<system>".default = derivation;
  # Executed by `nix run .#<name>`
  apps."<system>"."<name>" = {
    type = "app";
    program = "<store-path>";
  };
  # Executed by `nix run . -- <args?>`
  apps."<system>".default = { type = "app"; program = "..."; };

  # Formatter (alejandra, nixfmt or nixpkgs-fmt)
  formatter."<system>" = derivation;
  # Used for nixpkgs packages, also accessible via `nix build .#<name>`
  legacyPackages."<system>"."<name>" = derivation;
  # Overlay, consumed by other flakes
  overlays."<name>" = final: prev: { };
  # Default overlay
  overlays.default = final: prev: { };
  # Nixos module, consumed by other flakes
  nixosModules."<name>" = { config, ... }: { options = {}; config = {}; };
  # Default module
  nixosModules.default = { config, ... }: { options = {}; config = {}; };
  # Used with `nixos-rebuild switch --flake .#<hostname>`
  # nixosConfigurations."<hostname>".config.system.build.toplevel must be a derivation
  nixosConfigurations."<hostname>" = {};
  # Used by `nix develop .#<name>`
  devShells."<system>"."<name>" = derivation;
  # Used by `nix develop`
  devShells."<system>".default = derivation;
  # Hydra build jobs
  hydraJobs."<attr>"."<system>" = derivation;
  # Used by `nix flake init -t <flake>#<name>`
  templates."<name>" = {
    path = "<store-path>";
    description = "template description goes here?";
  };
  # Used by `nix flake init -t <flake>`
  templates.default = { path = "<store-path>"; description = ""; };
}
```

The first line `{ self, nixpkgs, ... }@ inputs:` defines the functions
parameters: It's important to understand that within the scope of the `outputs`
function `nixpkgs` is available at the top-level because we explicitly passed it
as an argument but for individual modules outside this flake the scope is lost,
and you need to use `inputs.nixpkgs` (or equivalent)

1. It explicitly names the `self` attribute, making it directly accessible. The
   variadic `...` ellipses part of the function signature is what allows all
   your flake inputs to be brought into the function's scope without having to
   list each one explicitly.

2. It destructures all other attributes (your defined `inputs`) into the
   functions scope.

3. It gives you a convenient single variable, `inputs`, that refers to the
   entire attribute set passed to the `outputs` function. This allows you to
   access inputs either individually (e.g. `nixpkgs`) or through the `inputs`
   variable (e.g. `inputs.nixpkgs`).

You can also define additional arbitrary attributes, but these are the outputs
that Nix knows about.

As you can see, the majority of the outputs within the outputs schema expect a
derivation. This means that for packages, applications, formatters, checks, and
development shells, you'll be defining a Nix derivation—a set of instructions
that tells Nix how to build a particular software component. This is central to
Nix's declarative nature.

- The command `nix flake show`, takes a flake URI and prints all the outputs of
  the flake as a nice tree structure, mapping attribute paths to the types of
  values.

```bash
  ~/players/third  3s
❯ nix flake show
path:/home/jr/players/third?lastModified=1748272555&narHash=sha256-oNzkC6X9hA0MpOBmJSZ89w4znXxv4Q5EkFhp0ewehY0%3D
├───nixosConfigurations
│   └───testing: NixOS configuration
└───nixosModules
    └───default: NixOS module
```

To show you the structure of this little flake project:

```bash
  ~/players
❯ tree
 .
├──  first
│   ├──  flake.lock
│   ├──  flake.nix
│   └──  result -> /nix/store/701vyaanmqchd2nnaq71y65v8ws11zx0-nixos-system-nixos-24.11.20250523.f09dede
├──  second
│   ├──  flake.lock
│   └──  flake.nix
└──  third
    ├──  flake.lock
    ├──  flake.nix
    └──  result -> /nix/store/mlszr5ws3xaly8m4q9jslgs31w6w76y2-nixos-system-nixos-24.11.20250523.f09dede
```

## Simple Example providing an output

```nix
# flake.nix
{
  outputs = { self }: {
    bada = "bing";
  };
}
```

You can then evaluate this specific output using `nix eval`:

```bash
nix eval .#bada
"bing"
```

## Outputs understood by Nix

While the attribute set that `outputs` returns may contain arbitrary attributes,
meaning any valid Nix value. Some of the standard outputs are understood by
various `nix` utilities. `packages` is one of these:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { self, nixpkgs }: {
    # this is the re-exporting part!
    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
  };
}
```

- Re-exporting happens when you take the value of `hello` in its standard
  derivation format, exactly as `nixpkgs` produces it and assign it to an
  attribute in your own flake's outputs.
  - `packages.x86_64-linux.hello`(your flake's output path) `=`
    ` nixpkgs.legacyPackages.x86_64-linux.hello`(the source from the `nixpkgs`
    flake's output)

  - We're saying, My flakes `hello` package is exactly the same as the `hello`
    package found inside the `nixpkgs` input flake.

  - It's important to understand that within the scope of the `outputs` function
    (i.e. within your flake), `nixpkgs` is available at the top-level (i.e. the
    `= nixpkgs` part) because we explicitly passed it as an argument but for
    individual modules outside of this flake the scope is lost, and
    `inputs.nixpkgs` is needed.

The following command builds the reexported package:

```bash
nix build .#hello
```

or run it with:

```bash
nix run .#hello
```

You might notice `x86_64-linux` appearing in the package path, and there's a
good reason for it. Flakes are designed to provide _hermetic evaluation_,
meaning their outputs should be identical regardless of the environment where
they're built. A key factor in any build system is the platform (which combines
the architecture and operating system, like `x86_64-linux` or `aarch64-darwin`).

Because of Nix's commitment to reproducibility across different systems, any
flake output that involves building software packages must explicitly specify
the platform. The standard approach is to structure these outputs as an
attribute set where the names are platforms, and the values are the outputs
specific to that platform. For the packages output, each platform-specific value
is itself an attribute set containing the various packages built for that
particular system.

## Exporting Functions

This example outputs a `sayGoodbye` function, via the `lib` attribute, that
takes a name for its input and outputs a string saying Goodbye very nicely to
the person with that name:

```nix
{
  outputs = { self }: {
    lib = {
      sayGoodbye = name: "Goodbye F*** Off, ${name}!";
    };
  };
}
```

You could then specify this flake as an input to another flake and use
`sayGoodbye` however you'd like.

Or load it into the `nix repl` like so:

```bash
nix repl
nix-repl> :lf .
nix-repl> lib.sayGoodbye
«lambda sayGoodbye @ /nix/store/665rwfvkwdx6kwvk9ldijp2a6jvcgv1n-source/flake.nix:4:20»
nix-repl> lib.sayGoodbye "Jr"
"Goodbye F*** Off, Jr!"
```

- As you can see, specifying `lib.sayGoodbye` without any arguments returns a
  function. (a lambda function)

## Simplifying Multi-Platform Outputs with flake-utils

Manually repeating these platform definitions for every output (`packages`,
`devShells`, `checks`, etc.) can quickly become verbose. This is where the
flake-utils helper flake comes in handy. It provides utilities to reduce
boilerplate when defining outputs for multiple systems.

A commonly used function is `flake-utils.lib.eachDefaultSystem`, which
automatically generates outputs for common platforms (like `x86_64-linux`,
`aarch64-linux`, `x86_64-darwin`, `aarch64-darwin`). This transforms your
outputs definition from manually listing each system to a more concise
structure:

# Example using flake-utils

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils"; # Don't forget to add flake-utils to inputs!
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        packages.hello = pkgs.hello; # Now directly defines 'hello' for the current 'system' # packages.default = self.packages.${system}.hello; # Optional default alias
        devShells.default = pkgs.mkShell {
          packages = [pkgs.hello];
        };
      }
    );
}
```

- This flake-utils pattern is particularly useful for defining consistent
  development environments across platforms, which can then be activated simply
  by running `nix develop` in the flake's directory.

### Adding Formatter, Checks, and Devshell Outputs

This is a minimal flake for demonstration with a hardcoded `system`, for more
portability:

```nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
   };

  outputs = inputs@{ nixpkgs, home-manager, treefmt-nix, ... }: let

    system = "x86_64-linux";
    host = "your-hostname-goes-here";
      # Define pkgs with allowUnfree
    pkgs = import inputs.nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };

        # Formatter configuration
    treefmtEval = treefmt-nix.lib.evalModule pkgs ./lib/treefmt.nix;

in {

    formatter.${system} = treefmtEval.config.build.wrapper;

    # Style check for CI
    checks.${system}.style = treefmtEval.config.build.check self;

    # Development shell
    devShells.${system}.default = import ./lib/dev-shell.nix {
      inherit inputs;
    };


    nixosConfigurations = {
      hostname = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.users.jdoe = ./home.nix;

            # Optionally, use home-manager.extraSpecialArgs to pass
            # arguments to home.nix
          }
        ];
      };
    };
  };
}
```

And in `lib/treefmt.nix`:

```nix
# treefmt.nix
{
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true;
    deadnix.enable = true;
    # rustfmt.enable = true;
    # shellcheck.enable = true;
    # prettier.enable = true;
    statix.enable = true;
    keep-sorted.enable = true;
    # nixfmt = {
    #   enable = true;
    #   # strict = true;
    # };
  };
  settings = {
    global.excludes = [
      "LICENSE"
      "README.md"
      ".adr-dir"
      "nu_scripts"
      # unsupported extensions
      "*.{gif,png,svg,tape,mts,lock,mod,sum,toml,env,envrc,gitignore,sql,conf,pem,*.so.2,key,pub,py,narHash}"
      "data-mesher/test/networks/*"
      "nss-datamesher/test/dns.json"
      "*.age"
      "*.jpg"
      "*.nu"
      "*.png"
      ".jj/*"
      "Cargo.lock"
      "flake.lock"
      "hive/moonrise/borg-key-backup"
      "justfile"
    ];
    formatter = {
      deadnix = {
        priority = 1;
      };
      statix = {
        priority = 2;
      };
      alejandra = {
        priority = 3;
      };
    };
  };
}
```

Now we have a few commands available to us in our flake directory:

- `nix fmt`: Will format your whole configuration consistently

- `nix flake check`: While this command was already available, it is now tied to
  treefmt's check which will check the style of your syntax and provide
  suggestions.

And this is `lib/dev-shell.nix`:

```nix
{
  inputs,
  system ? "x86_64-linux",
}: let
  # Instantiate nixpkgs with the given system and allow unfree packages
  pkgs = import inputs.nixpkgs {
    inherit system;
    config.allowUnfree = true;
    overlays = [
      # Add overlays if needed, e.g., inputs.neovim-nightly-overlay.overlays.default
    ];
  };
in
  pkgs.mkShell {
    name = "nixos-dev";
    packages = with pkgs; [
      # Nix tools
      nixfmt-rfc-style # Formatter
      deadnix # Dead code detection
      nixd # Nix language server
      nil # Alternative Nix language server
      nh # Nix helper
      nix-diff # Compare Nix derivations
      nix-tree # Visualize Nix dependencies

      # Code editing
      helix # Your editor

      # General utilities
      git
      ripgrep
      jq
      tree
    ];

    shellHook = ''
      echo "Welcome to the NixOS development shell!"
      echo "System: ${system}"
      echo "Tools available: nixfmt, deadnix, nixd, nil, nh, nix-diff, nix-tree, helix, git, ripgrep, jq, tree"
    '';
  }
```

Now you can run `nix develop` in the flake directory and if successfull, you'll
see the `echo` commands above and you will have all the tools available in your
environment without having to explicitly install them.
