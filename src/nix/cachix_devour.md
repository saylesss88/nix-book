---
title: Cachix devour-flake
date: 2025-12-05
author: saylesss88
collection: "notes"
tags: ["cache", "flakes"]
draft: false
---

# Cachix and the devour-flake

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

Using devour-flake to Cache All Your Flake Outputs to Cachix

When working with Nix flakes, it’s common to have many outputs: packages, apps,
dev shells, NixOS or Darwin configurations, and more. Efficiently building and
caching all these outputs can be challenging, especially in CI or when
collaborating. This is where devour-flake and Cachix shine.

**Why Use the devour-flake?**

By default, building all outputs of a flake with `nix build .#a .#b ... .#z` can
be slow and inefficient, as Nix will evaluate the flake multiple times, once for
each output. devour-flake solves this by generating a "consumer" flake that
depends on all outputs, allowing you to build everything in one go with a single
evaluation

## Installation

There quite a few ways to do this, choose a method of installation from the
[devour-flake](https://github.com/srid/devour-flake) repository and then
continue with step 1.

Nix will only download binaries from binary caches if they are cryptographically
signed with any of the keys listed in `nix.settings.trusted-public-keys`.

```nix
# This is the default
nix.settings.require-sigs = true;
```

Example:

```nix
nix.settings = {
  builders-use-substitutes = true;
  substituters = [
    "https://cache.nixos.org"
  ];
  trusted-public-keys = [
    "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
  ];
};
```

You can even build it without installing with the following command:

```bash
nix build github:srid/devour-flake \
  -L --no-link --print-out-paths \
  --override-input flake path/to/flake | cachix push <name>
```

```bash
nix-shell -p cachix
```

This will push all flake outputs to cachix if you have a valid authentication
token and have created a cache already.

How to Use devour-flake with Cachix

1. Prerequisites

- **A Cachix cache**: Create one on [Cachix](https://www.cachix.org/) and
  generate a "Write + Read" auth token. You'll click the cache you just created
  and select Settings, in the settings you'll find Auth Tokens. When in the Auth
  Tokens section give your token a Description, Expiration date, and finally
  click Generate.

(Optional) Configure your token locally, copy your auth token for the following
command:

```bash
cachix authtoken <YOUR_TOKEN>
# Use cachix cli for the following
cachix use your-cache-name
```

- `cachix use` adds your substitutors and trusted-public-keys to your
  `~/.config/nix/nix.conf` and creates one if it doesn't exist.

**Push All Flake Inputs to Cachix**

Replace `<mycache>` with the name of the cache you just created.

```bash
nix flake archive --json \
  | jq -r '.path,(.inputs|to_entries[].value.path)' \
  | cachix push <mycache>
```

You should see output similar to the following:

```bash
Pushing 637 paths (2702 are already present) using zstd to cache sayls8 ⏳

✓ /nix/store/0aqvmjvhkar3j2f7zag2wjl4073apnvk-vimplugin-crates.nvim-2025-05-30 (734.65 KiB)
✓ /nix/store/02wm10zck7rb836kr0h3afjxl80866dp-X-Restart-Triggers-keyd (184.00 B)
✓ /nix/store/0asdaaax0lf1wa6m6lqqdvc8kp6qn3f6-dconf-cleanup (1008.00 B)
✓ /nix/store/09ki2jlh6sqbn01yw6n15h8d55ihxygf-helix-tree-sitter-mojo-3d7c53b8038f9ebbb57cd2e61296180aa5c1cf64 (601.37 KiB)
✓ /nix/store/0i2c29nldqvb9pnypvp3ika4i7fhc0ck-devour-output (312.00 B)
✓ /nix/store/0c0mwfb78xm862a7g4h9fhgzn55zppj6-helix-term (29.88 MiB)
✓ /nix/store/0fhdpb2qck1kbngq1dlc8lyqqadj2pb1-hyprcursor-0.1.12+date=2025-06-05_45fcc10-lib (487.30 KiB)
✓ /nix/store/0mfpi51bswgd91l8clqcz6mxy5k5zcd4-vimplugin-auto-pairs-2019-02-27 (40.60 KiB)
✓ /nix/store/0k2zq8y78vrhhkf658j6i45vz3y89v11-helix-tree-sitter-tcl-56ad1fa6a34ba800e5495d1025a9b0fda338d5b8 (110.25 KiB)
✓ /nix/store/0qxmahrw935136dbxkmvrg14fgnzi6bb-vimplugin-obsidian.nvim-2025-07-01 (493.02 KiB)
✓ /nix/store/0wjppqzcbnlf9srhr6k27pz403j3mg2j-hm-session-vars.sh (1.86 KiB)
✓ /nix/store/0z41071z33zg1zqyasccc3cfhxj389k0-helix-tree-sitter-swift-57c1c6d6ffa1c44b330182d41717e6fe37430704 (2.77 MiB)
✓ /nix/store/0n5f1x8lpc93zm81bxrfh6yccyngvrdl-unit-plymouth-read-write.service (1.19 KiB)
✓ /nix/store/0z8ac35n89lv2knzaj6kkp0cfxr6pmgc-hm_face.png (300.60 KiB)
✓ /nix/store/0zp5846pry5rknnvzz81zlvj4ghnkxp5-hyprutils-0.8.1+date=2025-07-07_a822973 (421.64 KiB)
✓ /nix/store/118ihgwjw6kp0528igns3pnvzbszljmg-unit-dbus.service (1.34 KiB)
✓ /nix/store/0pajdq9mfgkcdwbqp38j7d4clc9h9iik-hm_.mozillafirefoxdefault.keep (112.00 B)
✓ /nix/store/0nlvffvpx6s8mpd2rpnqb1bl5idd16yk-hm-dconf.ini (224.00 B)
✓ /nix/store/1fiqgqvi574rdckav0ikdh8brwdhvh69-vimplugin-alpha-nvim-2025-05-26 (69.38 KiB)
✓ /nix/store/1fqxw31p1llag0g7wg7izq22x5msz47r-vimplugin-persistence.nvim-2025-02-25 (37.74
```

> ❗ NOTE: The effectiveness of pushing the rest to cachix depend on your
> network speed. I actually noticed a slow down after pushing the `nix/store`.
> Pushing the `nix/store` is rarely necessary and can be very slow and
> bandwidth-intensive. Most users will only need to push relevant outputs.

**Push the Entire /nix/store**

```bash
nix path-info --all | cachix push <mycache>
```

**Pushing shell environment**

```bash
nix develop --profile dev-profile -c true
# then run
cachix push <mycache> dev-profile
```

- For the Flake way of doing things you would create something like the
  following:

```nix
{
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.custom.cachix;
in {
  options = {
    custom.cachix.enable = lib.mkEnableOption "Enable custom cachix configuration";
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = with pkgs; [cachix];

    # to prevent garbage collection of outputs immediately after building
    nix.extraOptions = "gc-keep-outputs = true";
    nix.settings = {
      substituters = [
        "https://nix-community.cachix.org"
        "https://hyprland.cachix.org"
        "https://ghostty.cachix.org"
        "https://neovim-nightly.cachix.org"
        "https://yazi.cachix.org"
        "https://helix.cachix.org"
        "https://nushell-nightly.cachix.org"
        "https://wezterm.cachix.org"
        "https://sayls88.cachix.org"
        # "https://nixpkgs-wayland.cachix.org"
      ];
      trusted-public-keys = [
        "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
        "hyprland.cachix.org-1:a7pgxzMz7+chwVL3/pzj6jIBMioiJM7ypFP8PwtkuGc="
        "ghostty.cachix.org-1:QB389yTa6gTyneehvqG58y0WnHjQOqgnA+wBnpWWxns="
        "neovim-nightly.cachix.org-1:feIoInHRevVEplgdZvQDjhp11kYASYCE2NGY9hNrwxY="
        "yazi.cachix.org-1:Dcdz63NZKfvUCbDGngQDAZq6kOroIrFoyO064uvLh8k="
        "helix.cachix.org-1:ejp9KQpR1FBI2onstMQ34yogDm4OgU2ru6lIwPvuCVs="
        "nushell-nightly.cachix.org-1:nLwXJzwwVmQ+fLKD6aH6rWDoTC73ry1ahMX9lU87nrc="
        "wezterm.cachix.org-1:kAbhjYUC9qvblTE+s7S+kl5XM1zVa4skO+E/1IDWdH0="
        "sayls88.cachix.org-1:LT8JnboX8mKhabC3Mj/ONHb5tyrjlnsdauQkD8Lu0us="
        # "nixpkgs-wayland.cachix.org-1:3lwxaILxMRkVhehr5StQprHdEo4IrE8sRho9R9HOLYA="
      ];
    };
  };
}
```

- The sayls8 entries are my custom cache. To find your trusted key go to the
  cachix website, click on your cache and it is listed near the top.

- I enable this with `custom.cachix.enable = true;` in my `configuration.nix` or
  equivalent.

- Another option is to use the top-level `nixConfig` attribute for adding your
  substitutors and trusted-public-keys. You only need to choose 1 method FYI:

```nix
{
  description = "NixOS & Flake Config";

# the nixConfig here only affects the flake itself, not the system configuration!
  nixConfig = {
    experimental-features = [ "nix-command" "flakes" ];
    trusted-users = [ "ryan" ];

    substituters = [
      # replace official cache with a mirror located in China
      "https://mirrors.ustc.edu.cn/nix-channels/store"
      "https://cache.nixos.org"
    ];

    # nix community's cache server
    extra-substituters = [
      "https://nix-community.cachix.org"
      "https://nixpkgs-wayland.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "nixpkgs-wayland.cachix.org-1:3lwxaILxMRkVhehr5StQprHdEo4IrE8sRho9R9HOLYA="
    ];
  };
# ... snip
```

> ❗️ WARNING: `trusted-users` is a list of users who are allowed to bypass many
> of Nix's normal safety restrictions and change daemon-level settings. Keep
> `trusted-users` as small as possible (often just `root` and maybe your own
> user on a single-user box)

2. Building and Caching All Outputs

You can build and push all outputs of your flake to Cachix using the following
command when in your flake directory:

```bash
nix build github:srid/devour-flake \
 -L --no-link --print-out-paths \
 --override-input flake . \
 | cachix push <your-cache-name>
```

- Replace `your-cache-name` with your actual Cachix cache name.

  This command will:

- Use devour-flake to enumerate and build all outputs of your flake (including
  packages, devShells, NixOS configs, etc.)

- Pipe the resulting store paths to cachix push, uploading them to your binary
  cache.

3. Example

Suppose your cache is named my-flake-cache:

```bash
nix build github:srid/devour-flake \
 -L --no-link --print-out-paths \
 --override-input flake . \
 | cachix push my-flake-cache
```

4. Integration in CI

This approach is particularly useful in CI pipelines, where you want to ensure
all outputs are built and cached for collaborators and future builds. You can
add the above command to your CI workflow, ensuring the Cachix auth token is
provided as a secret

5. Advanced: Using as a Nix App

You can add devour-flake as an input to your flake for local development:

```nix
{
  inputs = {
    devour-flake.url = "github:srid/devour-flake";
    devour-flake.flake = false;
  };
}
```

And in your flake's `outputs`, add an overlay that makes `devour-flake`
available in your package set:

```nix
outputs = { self, nixpkgs, devour-flake, ... }@inputs: {
  overlays.default = final: prev: {
    devour-flake = import devour-flake { inherit (prev) pkgs; };
  };

  # Example: Add devour-flake to your devShell
  devShells.x86_64-linux.default = let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [ self.overlays.default ];
    };
  in pkgs.mkShell {
    buildInputs = [ pkgs.devour-flake ];
  };
};
```

Use devour-flake in your devShell:

```bash
nix develop
```

You'll have the `devour-flake` command available for local use, so you can
quickly build and push all outputs as needed.

> TIP: Alternatively, use `devour-flake` as an app:
>
> ```nix
> apps.x86_64-linux.devour-flake = {
>  type = "app";
>  program = "${self.packages.x86_64-linux.devour-flake}/bin/devour-flake";
> };
>
> ```

What Gets Built and Cached?

`devour-flake` detects and builds all standard outputs of a flake, including:

- packages

- apps

- checks

- devShells

- nixosConfigurations.\*

- darwinConfigurations.\*

- home-manager configurations

This ensures that everything your flake produces is available in your Cachix
cache for fast, reproducible builds.

---

## References:

[devour-flake documentation](https://github.com/srid/devour-flake)

[Discourse Cachix for Flakes](https://discourse.nixos.org/t/how-to-set-up-cachix-in-flake-based-nixos-config/31781)

[Cachix docs: Flakes](https://docs.cachix.org/installation#flakes)

[Tweag Evaluation Caching](https://www.tweag.io/blog/2020-06-25-eval-cache/#:~:text=The%20overhead%20for%20creating%20the,nixpkgs%20blender%20takes%204.9%20seconds.)

[Scrive Caching](https://scrive.github.io/nix-workshop/06-infrastructure/01-caching-nix.html)
