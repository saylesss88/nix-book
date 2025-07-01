# Cachix and the devour-flake

Using devour-flake to Cache All Your Flake Outputs to Cachix

When working with Nix flakes, it’s common to have many outputs—packages, apps,
dev shells, NixOS or Darwin configurations, and more. Efficiently building and
caching all these outputs can be challenging, especially in CI or when
collaborating. This is where devour-flake and Cachix shine. Why Use
devour-flake?

By default, building all outputs of a flake with `nix build .#a .#b ... .#z` can
be slow and inefficient, as Nix will evaluate the flake multiple times—once for
each output. devour-flake solves this by generating a "consumer" flake that
depends on all outputs, allowing you to build everything in one go with a single
evaluation

## Installation

There quite a few ways to do this, choose a method of installation from the
[devour-flake](https://github.com/srid/devour-flake) repository and then
continue with step 1.

You can even build it without installing with the following command:

```bash
nix build github:srid/devour-flake \
  -L --no-link --print-out-paths \
  --override-input flake github:nammayatri/nammayatri | cachix push <name>
```

This will push all flake outputs to cachix if you have a valid authentication
token and have created a cache already.

How to Use devour-flake with Cachix

1. Prerequisites

- A Cachix cache: Create one on [Cachix](https://www.cachix.org/) and generate a
  "Write + Read" auth token. You'll click the cache you just created and select
  Settings, in the settings you'll find Auth Tokens. When in the Auth Tokens
  section give your token a Description, Expiration date, and finally click
  Generate.

(Optional) Configure your token locally, copy your auth token for the following
command:

```bash
cachix authtoken <YOUR_TOKEN>
# Use cachix cli for the following
cachix use <your-cache-name>
```

- `cachix use` adds your substitutors and trusted-public-keys to your
  `~/.config/nix/nix.conf` and creates one if it doesn't exist.

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

- The sayls88 entries are my custome cache. To find your trusted key go to the
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

2. Building and Caching All Outputs

You can build and push all outputs of your flake to Cachix using the following
command when in your flake directory:

```bash
nix build github:srid/devour-flake \
 -L --no-link --print-out-paths \
 --override-input flake . \
 | cachix push <your-cache-name>
```

- Replace <your-cache-name> with your actual Cachix cache name.

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
{ inputs = { devour-flake.url = "github:srid/devour-flake";
devour-flake.flake = false; }; }
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

devour-flake detects and builds all standard outputs of a flake, including:

- packages

- apps

- checks

- devShells

- nixosConfigurations.\*

- darwinConfigurations.\*

- home-manager configurations

This ensures that everything your flake produces is available in your Cachix
cache for fast, reproducible builds.

References:

[devour-flake documentation](https://github.com/srid/devour-flake)

[Discourse Cachix for Flakes](https://discourse.nixos.org/t/how-to-set-up-cachix-in-flake-based-nixos-config/31781)

[Cachix docs: Flakes](https://docs.cachix.org/installation#flakes)
