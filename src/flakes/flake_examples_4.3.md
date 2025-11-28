---
title: Flake outputs
date: 2025-11-28
author: saylesss88
collection: blog
tags: ["nixos", "flakes"]
draft: false
---

# Nix Flake Examples

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

This chapter provides practical examples to illustrate the concepts discussed in
"Nix Flakes Explained."

## Example showing the extensibility of Flakes

NixOS modules and configurations offer us a powerful and composable way to
define and share system configurations. Imagine we have several independent
"players," each with their own unique set of configurations or modules. How do
we combine these individual contributions into a single, cohesive system without
directly altering each player's original flake?

This example demonstrates how flakes can extend and compose each other, allowing
you to layer configurations on top of existing ones. This is particularly useful
when you want to:

- Build upon a base configuration without modifying its source.

- Combine features from multiple independent flakes into a single system.

- Create specialized versions of an existing configuration.

Let's simulate this by creating a players directory with three sub-directories:
`first`, `second`, and `third`. Each of these will contain its own `flake.nix`.

```bash
mkdir players
cd players
mkdir first
mkdir second
mkdir third
cd first
```

Now create a `flake.nix` with the following contents:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: {
    nixosModules.default = {
      config,
      pkgs,
      lib,
      ...
    }: {
      # Create a file `/etc/first-file`
      environment.etc.first-file.text = "Hello player # 1!";
      boot.initrd.includeDefaultModules = false;
      documentation.man.enable = false;
      boot.loader.grub.enable = false;
      fileSystems."/".device = "/dev/null";
      system.stateVersion = "24.11";
    };
    nixosConfigurations.testing = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        self.nixosModules.default
      ];
    };
  };
}
```

- This demonstrates using `self` to reference this flake from within its own
  outputs. This is the main use for `self` with flakes. Without `self`, I
  wouldn't have a direct way to refer to the `nixosModules.default` that's
  defined within the same flake.

Now in the `players/second` directory create this `flake.nix`:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: {
    nixosModules.default = {
      config,
      pkgs,
      lib,
      ...
    }: {
      # Create a file `/etc/second-file`
      environment.etc.second-file.text = "Hello player # 2!";
    };
  };
}
```

- `nixosModules.default` is a module which is a function that, when called by
  the NixOS module system, returns an attribute set representing a piece of
  system configuration.
  - Within that attribute set, it specifies that the file `/etc/second-file`
    should exist with "Hello player # 2!" as its content.

And finally in `players/third` create another `flake.nix`:

```nix
# flake.nix
{
  inputs = {
    first.url = "/home/jr/players/first";
    nixpkgs.follows = "first/nixpkgs";
    second = {
      url = "/home/jr/players/second";
      inputs.nixpkgs.follows = "first/nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    first,
    second,
  }:
    first.outputs
    // {
      nixosConfigurations.testing = first.nixosConfigurations.testing.extendModules {
        modules = [
          second.nixosModules.default
        ];
      };
    };
}
```

- You'll have to change the locations to where you placed your `players`
  directory in the `inputs` above.

In your `third` directory inspect it with:

```bash
  ~/players/third
❯ nix flake show
path:/home/jr/players/third?lastModified=1748271697&narHash=sha256-oNzkC6X9hA0MpOBmJSZ89w4znXxv4Q5EkFhp0ewehY0%3D
├───nixosConfigurations
│   └───testing: NixOS configuration
└───nixosModules
    └───default: NixOS module
```

and build it with:

```bash
nix build .#nixosConfigurations.testing.config.system.build.toplevel
```

```bash
cat result/etc/first-file
Hello player # 1!
cat result/etc/second-file
Hello player # 2!
```

**Understanding the Extension**

As you saw in the `flake.nix` for the third player, we leveraged two key flake
features to combine and extend the previous configurations:

1. **Attribute Set Union** (`//` operator):

```nix
outputs = { ..., first, second, ... }:
first.outputs // { # ... your extensions here ...
};
```

The `//` (attribute set union) operator allows us to take all the outputs from
`first.outputs` (which includes its `nixosConfigurations` and `nixosModules`)
and then overlay or add to them on the right-hand side. This means our third
flake will inherit all the outputs from first, but we can then modify or add new
ones without changing the first flake itself.

2. `config.extendModules`:

```nix
    nixosConfigurations.testing = first.nixosConfigurations.testing.extendModules {
      modules = [
        second.nixosModules.default
      ];
    };
```

This is the core of the extension. We're taking the testing NixOS configuration
defined in the first flake (`first.nixosConfigurations.testing`) and then
calling its `extendModules` function. This function allows us to inject
additional NixOS modules into an already defined system configuration. In this
case, we're adding the default module from the second flake
(`second.nixosModules.default`).

By combining these techniques, the third flake successfully creates a NixOS
configuration that includes both the settings from first (like `/etc/first-file`
and the base system options) and the settings from second (like
`/etc/second-file`), all without directly altering the first or second flakes.
This demonstrates the incredible power of flake extensibility for building
complex, modular, and composable systems.
