---
title: NixOS Containers
date: 2026-01-30
author: saylesss88
collection: "blog"
tags: ["nix lang", "idiomatic"]
draft: true
---

# Idiomatic Nix

There are quite a few resources out there that share best practices, but no
single unified place to find them all. I'm going to try to build on
[nix.dev's Best practices](https://nix.dev/guides/best-practices), by doing some
research as well as examining the code of some of the leaders in the NixOS
world. (Tweag, numtide, etc.)

<details>
<summary> ✔️ mdbook-nix-repl for interactive code blocks </summary>

I've added a `flake.nix` to the `mdbook-nix-repl` repo, you can add it as a
flake input:

1. `flake.nix`:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    mdbook-nix-repl.url = "github:saylesss88/mdbook-nix-repl?dir=server";
  };

  outputs = { self, nixpkgs, mdbook-nix-repl, ... }: {
    nixosConfigurations.magic = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix

        mdbook-nix-repl.nixosModules.default
      ];
    };
  };
}
```

2. `configuration.nix`:

```nix
{ pkgs, ... }:
{
  imports = [
  ];

  # This option is now provided by the module you imported from the flake
  custom.nix-repl-server = {
    enable = true;
    port = 8080;
    tokenFile = "/etc/nix-repl-server.env";
  };
}
```

3. Copy the value of `NIX_REPL_TOKEN` in `theme/index.hbs`, and add create file
   `/etc/nix-repl-server.env`:

```bash
# Create the file with strict permissions (root read-only)
sudo touch /etc/nix-repl-server.env
sudo chmod 600 /etc/nix-repl-server.env

# Edit it to add: NIX_REPL_TOKEN=your_token_from_index_hbs
sudo vim /etc/nix-repl-server.env
```

Expected format:

```text
NIX_REPL_TOKEN=9deb7efadb74b9e962e7911bb5caf3b3fef275a1b915b526
```

4. Rebuild, and the server will now be running at boot.

</details>

All the following examples are interactive, press play to see the result.(The
following examples come directly from `nix.dev`)

```nix repl
rec {
    a = 1;
    b = a + 2;
}
```

Use this instead:

```nix repl
let
  a = 1;
in {
    a = a;
    b = a + 2;
}
```

> 💡 TIP Self-reference can be achieved by explicitly naming the attribute set:

```nix repl
 let
   argset = {
     a = 1;
     b = argset.a + 2;
  };
in
  argset
```

## Updating nested attribute sets

```nix repl
{ a = 1; b = 2; } // { b = 3; c = 4; }
```

Updates are shallow, names on the right take precidence:

```nix repl
{ a = { b = 1; }; } // { a = { c = 3; }; }
```

```nix repl
let pkgs = import <nixpkgs> {}; in
pkgs.lib.recursiveUpdate { a = { b = 1; }; } { a = { c = 3;}; }
```
