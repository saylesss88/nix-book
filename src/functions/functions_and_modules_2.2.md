---
title: Functions and NixOS Modules
date: "2025-11-30"
author: "saylesss88"
collection: "blog"
tags: ["nixos", "functions"]
draft: false
---

# Functions and NixOS Modules

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

When you start exploring NixOS configurations or tools like Home Manager, you'll
encounter a concept called Nix Modules. Modules are also functions, but they
behave differently regarding their arguments, which can be a source of
confusion.

**What are NixOS Modules**?

Nix Modules are a powerful system built on top of basic Nix functions, primarily
used for declarative system configurations (like NixOS, Home Manager, NixOps,
etc.). They allow you to define parts of your system configuration in separate
files that are then composed together.

Each module is typically a Nix function that returns an attribute set with
specific keys like `options`, `config`, and `imports`.

**Automatic Arguments in Modules**

Unlike the functions we've been writing, Nix's module system automatically
passes a standard set of arguments to every module function it evaluates. You
don't explicitly pass these arguments when you `import` a module file; the
module system handles it for you.

The most common automatic arguments you'll see are:

- `config`: The aggregated configuration options of all modules combined. This
  is what you use to read other configuration values.

- `options`: The definitions of all available configuration options across all
  modules.

- `pkgs`: The standard Nixpkgs set, equivalent to `import <nixpkgs> {}`. This is
  incredibly convenient as you don't need to import it in every module.

- `lib`: The Nixpkgs utility library (`pkgs.lib`), providing helper functions
  for common tasks.

- `specialArgs`: An attribute set of extra arguments to be passed to the module
  functions.

A typical module might start like this:

```nix
# Example NixOS module
{ config, pkgs, lib, ... }: # These arguments are passed automatically by the module system
{
  # ... module options and configuration
  environment.systemPackages = [ pkgs.firefox pkgs.git ];
  services.nginx.enable = true;
  # ...
}
```

In the above module, the only required argument is `pkgs` because we explicitly
use it in the module (i.e. `pkgs.firefox`). Editors have pretty good support for
letting you know if you're missing arguments or have unnecessary ones. `config`,
and `lib` and would be required if we were setting any options in this module.

This automatic passing of arguments is a core feature of the module system that
simplifies writing configurations, as you always have access to `pkgs`, `lib`,
and the evolving `config` and `options` without boilerplate.

#### `specialArgs`: Passing Custom Arguments to Modules

While the module system passes a standard set of arguments automatically, what
if you need to pass additional, custom data to your modules that isn't part of
the standard `config`, `pkgs`, `lib`, or `options`? This is where `specialArgs`
comes in.

`specialArgs` is an attribute you can pass to the `import` function when you
load a module (or a set of modules). It's typically used to provide data that
your modules need but isn't something Nixpkgs would normally manage.

For example, in a `configuration.nix`:

```nix
# From your configuration.nix
{ config, pkgs, lib, ... }: # Standard module arguments

let
  myCustomValue = "helloWorld";
in
{
  # ... imports all modules, including your custom ones
  imports = [
    ./hardware-configuration.nix
    ./my-webserver-module.nix
  ];

  # This is where specialArgs would be used (often in import statements)
  # Example: passing a custom value to ALL modules:
  # (in module context, this is more complex, but conceptually)
  # let
  #   allModules = [ ./my-module.nix ];
  # in
  # lib.nixosSystem {
  #   modules = allModules;
  #   specialArgs = {
  #     username = "johndoe";
  #     mySecretKey = "/run/keys/ssh_key";
  #   };
  #   # ...
  # };
}
```

And then, inside `my-webserver-module.nix`:

```nix
# my-webserver-module.nix
{ config, pkgs, lib, username, mySecretKey, ... }: # username and mySecretKey come from specialArgs
{
  # ... use username and mySecretKey in your module
  users.users.${username} = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ];
    # ...
  };
  # ...
}
```

Any argument listed in a module's function signature that is not one of the
standard `config`, `pkgs`, `lib`, `options` (or `pkgs.callPackage`, etc., which
are often implicit through `pkgs`) must be provided via `specialArgs` at the
point where the modules are composed.

Any values listed in a module that aren’t automatically passed via Nixpkgs must
be explicitly provided through `specialArgs`.

### `specialArgs` and `extraSpecialArgs` with Flakes

NixOS modules use `specialArgs` and Home-Manager uses `extraSpecialArgs` to
allow you to pass extra arguments.

Or with Flakes it would look like this:

```nix
{
  description = "My Flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
   };

  outputs = { self, nixpkgs, home-manager, ... }:
    let
      lib = nixpkgs.lib;
      pkgs = nixpkgs.legacyPackages.${"x86_64-linux"};
      system = "x86_64-linux";
  host = "magic";
  username = "jr";
  userVars = {
    timezone = "America/New_York";
    locale = "en_US.UTF-8";
    gitUsername = "TSawyer87";
    dotfilesDir = "~/.dotfiles";
    wm = "hyprland";
    browser = "firefox";
    term = "ghostty";
    editor = "hx";
    keyboardLayout = "us";
  };
    in {
      nixosConfigurations = {
        YOURHOSTNAME = lib.nixosSystem {
          system = "x86_64-linux";
          modules = [ ./configuration.nix ];
          specialArgs = {
            inherit userVars; # == userVars = userVars;
            inherit host;
            inherit username;
          };
        };
      };
      homeConfigurations = {
        USERNAME = home-manager.lib.homeManagerConfiguration {
          inherit pkgs;
          modules = [ ./home.nix ];
          extraSpecialArgs = {
            inherit userVars;
            inherit host;
            inherit username;
            # or it can be written like this:
            # inherit userVars host username;
          };
        };
      };
    };
}
```

Now if I want to use any of these arguments in modules I can by any module file
referenced by my configuration.

For example, the following is a `git.nix` module that uses the variables from
the flake passed from `extraSpecialArgs` in this case because it's a
home-manager module:

```nix
# git.nix
{ userVars, ... }: {
  programs = {
    git = {
      enable = true;
      userName = userVars.gitUsername;
    };
  };
}
```

| Feature         | Regular Nix Function (e.g., `hello.nix`)                            | Nix Module (e.g., `my-config-module.nix`)                           |
| :-------------- | :------------------------------------------------------------------ | :------------------------------------------------------------------ |
| **Arguments**   | **You must explicitly pass every single argument.**                 | **Automatically receives `config`, `pkgs`, `lib`, `options`, etc.** |
| **Custom Args** | Passed directly in the function call.                               | Passed via `specialArgs` when the modules are composed.             |
| **Boilerplate** | Often needs `pkgs = import <nixpkgs> {};` if not explicitly passed. | `pkgs` and `lib` are always available automatically.                |
| **Purpose**     | Defines a package, a utility, or a single value.                    | Defines a reusable part of a declarative system configuration.      |
