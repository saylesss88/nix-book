---
title: Nix Module System Explained
date: 2025-11-21
author: saylesss88
---

# Chapter 3

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

## Nix Module System Explained

<!-- ![gruv3](images/gruv3.png) -->

![buildings](images/buildings1.png)

**TL;DR**: In this chapter, we will break down the Nix module system used by
both NixOS and Home-Manager. We will discuss using home-manager as a module and
the flexibility that modules give us. We will touch on options and break down
the `vim` module from the Nixpkgs collection. Finally we will display how to
test modules with the repl.

Your `configuration.nix` is a module. For the Nixpkgs collection most modules
are in `nixos/modules`.

The suggested way of using `home-manager` according to their manual is as a
[NixOS module](https://nix-community.github.io/home-manager/index.xhtml#sec-install-nixos-module).
Both home-manager and NixOS use the same module system.

## Module Structure

```nix
{
  imports = [
    # Paths to other modules.
    # Compose this module out of smaller ones.
  ];

  options = {
    # Option declarations.
    # Declare what settings a user of this module can set.
    # Usually this includes a global "enable" option which defaults to false.
  };

  config = {
    # Option definitions.
    # Define what other settings, services and resources should be active.
    # Usually these depend on whether a user of this module chose to "enable" it
    # using the "option" above.
    # Options for modules imported in "imports" can be set here.
  };
}
```

`imports`, `options`, and `config` are the top-level attributes of a Nix module.
They are the primary, reserved keys that the Nix module system recognizes and
processes to combine different configurations into a single, cohesive system or
user environment. `config` is the same `config` you receive as a module argument
(e.g. `{ pkgs, config, ... }:` at the top of your module function)

Understanding `config`:

`config` is the big constantly updated blueprint of your entire system.

Every time you bring in a new module, it adds its own settings and options to
this blueprint. So, when a module receives the `config` argument, it's getting
the complete picture of everything you've asked NixOS to set up so far.

This allows the module to:

- See what other parts of your system are doing.

- Make smart decisions based on those settings.

- Add its own pieces to the overall plan, building on what's already there.

- Most modules are functions that take an attribute set and return an attribute
  set.

To turn the above module into a function accepting an attribute set just add the
function arguments to the top, click the eye to see the whole module:

```nix
{ config, pkgs, ... }:
~ {
~   imports = [
~     # Paths to other modules.
~     # Compose this module out of smaller ones.
~   ];
~
~   options = {
~     # Option declarations.
~     # Declare what settings a user of this module can set.
~     # Usually this includes a global "enable" option which defaults to false.
~   };
~
~   config = {
~     # Option definitions.
~     # Define what other settings, services and resources should be active.
~     # Usually these depend on whether a user of this module chose to "enable" it
~     # using the "option" above.
~     # Options for modules imported in "imports" can be set here.
~   };
~ }
```

It may require the attribute set to contain:

- `config`: The configuration of the entire system.

- `options`: All option declarations refined with all definition and declaration
  references.

- `pkgs`: The attribute set extracted from the Nix package collection and
  enhanced with the `nixpkgs.config` option.

- `modulesPath`: The location of the module directory of NixOS.

## Modularize your configuration.nix

Many people start of using a single `configuration.nix` and eventually their
single file configuration gets too large to search through and maintain
conveniently.

This is where **modules** come in allowing you to break up your configuration
into logical parts. Your `boot.nix` will contain settings and options related to
the actual boot process. You're `services.nix` will only have services and so
on...

- These modules are placed in a logical path relative to either your
  `configuration.nix` or equivalent or if you're using flakes relative to your
  `flake.nix` or equivalent.
  - The `imports` mechanism takes paths to other modules as its argument and
    combines them to be included in the evaluation of the system configuration.

> ```nix
> { ... }:
> {
>   imports = [
>      # Paths to other modules
>
>      # They can be relative paths
>      ./otherModule.nix
>
>      # Or absolute
>      /path/to/otherModule.nix
>
>      # Or to a directory
>      ../modules/home/shells/nushell
>   ];
> }
> ```

> ❗: The **imports** mechanism includes and evaluates the Nix expression found
> at the given path _as a module_. If that path is a directory, it will
> automatically look for and evaluate a `default.nix` file within that directory
> _as a module_. It is common to have that `default.nix` be a function that only
> imports and combines all the modules in said directory. Like the above
> example, in the nushell directory would be a `default.nix` that is
> automatically imported and evaluated.

**Crucial Distinction: `imports` vs. `import`**:

Beginners often confuse the modules attribute `imports = [./module.nix]` here
with the Nix builtins function `import module.nix`. The first expects a path to
a file containing a NixOS module (having the same specific structure we're
describing here), while the second loads whatever Nix expression is in that file
(no expected structure). --NixOS Wiki.

Considering `configuration.nix` is a module, it can be imported like any other
module and this is exactly what you do when getting started with flakes.

```nix
# flake.nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ nixpkgs, home-manager, ... }: {
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

`modules = [...]` in `flake.nix`: This is effectively the initial `imports` list
for your entire NixOS system or Home Manager user configuration. It tells the
Nix module system: "Start by collecting and merging the configurations defined
in these specific modules."

The above example is what you get from running:
`nix flake new /etc/nixos -t github:nix-community/home-manager#nixos`

If you notice the `home-manager.nixosModules.home-manager`, that is what imports
home-manager as a module.

You could also make the actual home-manager module and import it like this:

```nix
# home-manager.nix
{ inputs, outputs, ... }: {
  imports = [
    # Import home-manager's NixOS module
    inputs.home-manager.nixosModules.home-manager
  ];

  home-manager = {
    extraSpecialArgs = { inherit inputs outputs; };
    users = {
      # Import your home-manager configuration
      your-username = import ../home-manager/home.nix;
    };
  };
}
```

This "module" isn't much different from the one included in the `flake.nix`
above, it is just shown here to show the flexibility of modules. They can be as
big and complex or as small and simple as you want. You can break up every
single program or component of your configuration into individual modules or
have modules that bundle similar programs the choice is yours.

Then in your `configuration.nix` or equivalent you would add `home-manager.nix`
to your imports list and you would have home-manager as a NixOS module.

<details>
<summary>
✔️ Refresher (Click to Expand):
</summary>

An **attribute set** is a collection of name-value pairs called _attributes_:

Attribute sets are written enclosed in curly braces `{}`. Attribute names and
attribute values are separated by an equal sign `=`. Each value can be an
arbitrary expression, terminated by a semicolon `;`.

> **Example**:[nix.dev reference](https://nix.dev/manual/nix/2.24/language/syntax#attrs-literal)
> This defines an attribute set with attributes named:
>
> - `x` with the value `123`, an integer
> - `text` with the value `"Hello"`, a string
> - `y` where the value is the result of applying the function `f` to the
>   attribute set `{bla = 456; }`
>
> ```nix
> {
>  x = 123;
>  text = "Hello";
>  y = f { bla = 456; };
> }
> ```
>
> ```nix
> { a = "Foo"; b = "Bar"}.a
> ~ "Foo"
> ```

Attributes can appear in any order. An attribute name may only occur once in
each attribute set.

> ❗ Remember `{}` is a valid attribute set in Nix.

The following is a **function** with an attribute set argument, remember that
anytime you see a `:` in Nix code it means this is a function. To the left is
the **function arguments** and to the right is the **function body**:

```nix
{ a, b }: a + b
```

The simplest possible **NixOS Module**:

```nix
{ ... }:
{
}
```

</details>

NixOS produces a full system configuration by combining smaller, more isolated
and reusable components: **Modules**. If you want to understand Nix and NixOS
make sure you grasp modules!

A NixOS module defines configuration options and behaviors for system
components, allowing users to extend, customize, and compose configurations
declaratively.

A **module** is a file containing a Nix expression with a specific structure. It
_declares_ options for other modules to define (give a value). Modules were
introduced to allow extending NixOS without modifying its source code.

To define any values, the module system first has to know which ones are
allowed. This is done by declaring options that specify which attributes can be
set and used elsewhere.

If you want to write your own modules, I recommend setting up
[nixd](https://github.com/nix-community/nixd?tab=readme-ov-file) or
[nil](https://github.com/oxalica/nil) with your editor of choice. This will
allow your editor to warn you about missing arguments and dependencies as well
as syntax errors.

### Declaring Options

Options are declared under the top-level `options` attribute with
`lib.mkOption`.

[mkOption](https://nixos.org/manual/nixpkgs/stable/#function-library-lib.options.mkOption)
Creates an Option attribute set. It accepts an attribute set with certain keys
such as, `default`, `package`, and `example`.

```nix
# options.nix
{ lib, ... }:
{
  options = {
    name = lib.mkOption { type = lib.types.str; };
  };
}
```

> `lib` provides helper functions from `nixpkgs.lib` and the ellipsis (`...`) is
> for arbitrary arguments which means that this function is prepared to accept
> **any additional arguments** that the caller might provide, even if those
> arguments are not explicitly named or used within the module's body. They make
> the modules more flexible, without the `...` each module would have to
> explicitly list every possible argument it might receive, which would be
> cumbersome and error-prone. So `{lib, ... }:` means that "I need the `lib`
> argument" **and** I acknowledge that the module system might pass other
> arguments automatically (like `config`, `pkgs`, etc.) and I'm fine with them
> being there, even if I don't use them directly in this specific module file.

### Defining Values

Options are **set** or **defined** under the top-level `config` attribute:

```nix
# config.nix
{ ... }:
{
  config = {
    name = "Slick Jones";
  };
}
```

In this **option declaration**, we created an option `name` of type _string_ and
set that same option to a string.

**Option Definitions** can be in a separate file than **Option Declarations**

### Evaluating Modules

Modules are **evaluated** with
[lib.evalModules](https://nixos.org/manual/nixpkgs/stable/#module-system-lib-evalModules)
`lib.evalModules` evaluates a set of modules, typically once per application
(e.g. once for NixOS and once for Home-Manager).

## Checking out the Vim module provided by Nixpkgs

The following is `nixpkgs/nixos/modules/programs/vim.nix`, a module that is
included in the Nixpkgs collection:

```nix
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.vim;
in
{
  options.programs.vim = {
    enable = lib.mkEnableOption "Vi IMproved, an advanced text";

    defaultEditor = lib.mkEnableOption "vim as the default editor";

    package = lib.mkPackageOption pkgs "vim" { example = "vim-full"; };
  };

  # TODO: convert it into assert after 24.11 release
  config = lib.mkIf (cfg.enable || cfg.defaultEditor) {
    warnings = lib.mkIf (cfg.defaultEditor && !cfg.enable) [
      "programs.vim.defaultEditor will only work if programs.vim.enable is
       enabled, which will be enforced after the 24.11 release"
    ];
    environment = {
      systemPackages = [ cfg.package ];
      variables.EDITOR = lib.mkIf cfg.defaultEditor (lib.mkOverride 900 "vim");
      pathsToLink = [ "/share/vim-plugins" ];
    };
  };
}
```

It provides options to enable Vim, set it as the default editor, and specify the
Vim package to use.

<details>
<summary> ✔️ Breakdown of the vim module.(Click to Expand)</summary>
1. Module Inputs and Structure:

```nix
{
  config,
  lib,
  pkgs,
  ...
}
```

**Inputs**: The module takes the above inputs and `...` (catch-all for other
args)

- `config`: Allows the module to read option values (e.g.
  `config.programs.vim.enable`). It provides access to the evaluated
  configuration.

- `lib`: The Nixpkgs library, giving us helper functions like `mkEnableOption` ,
  `mkIf`, and `mkOverride`.

- `pkgs`: The Nixpkgs package set, used to access packages like `pkgs.vim`

- `...`: Allows the module to accept additional arguments, making it flexible
  for extension in the future.

> Key Takeaways: A NixOS module is typically a function that can include
> `config`, `lib`, and `pkgs`, but it doesn’t require them. The `...` argument
> ensures flexibility, allowing a module to accept extra inputs without breaking
> future compatibility. Using `lib` simplifies handling options (mkEnableOption,
> mkIf, mkOverride) and helps follow best practices. Modules define options,
> which users can set in their configuration, and `config`, which applies
> changes based on those options.

2. Local Configuration Reference:

```nix
let
  cfg = config.programs.vim;
in
```

This is a local alias. Instead of typing `config.programs.vim` over and over,
the module uses `cfg`.

3. Option Declaration

```nix
options.programs.vim = {
  enable = lib.mkEnableOption "Vi IMproved, an advanced text";
  defaultEditor = lib.mkEnableOption "vim as the default editor";
  package = lib.mkPackageOption pkgs "vim" { example = "vim-full"; };
};
```

This defines three user-configurable options:

- `enable`: Turns on Vim support system-wide.

- `defaultEditor`: Sets Vim as the system's default `$EDITOR`.

- `package`: lets the user override which Vim package is used.

> `mkPackageOption` is a helper that defines a package-typed option with a
> default (`pkgs.vim`) and provides docs + example. Using `lib.mkEnableOption`
> makes it clear exactly where this function is coming from. Same with
> `lib.mkIf` and as you can see they can be further down the configuration,
> further from where you defined `with lib;` making it less clear where they
> come from. Explicitness is your friend when it comes to reproducability and
> clarity.

4. Conditional Configuration

```nix
config = lib.mkIf (cfg.enable || cfg.defaultEditor) {
```

- This block is only activated if _either_ `programs.vim.enable` or
  `defaultEditor` is set.

5. Warnings

```nix
warnings = lib.mkIf (cfg.defaultEditor && !cfg.enable) [
  "programs.vim.defaultEditor will only work if programs.vim.enable is enabled,
   which will be enforced after the 24.11 release"
];
```

Gives you a soft warning if you try to set `defaultEditor = true` without also
enabling Vim.

6. Actual System Config Changes

```nix
environment = {
  systemPackages = [ cfg.package ];
  variables.EDITOR = lib.mkIf cfg.defaultEditor (lib.mkOverride 900 "vim");
  pathsToLink = [ "/share/vim-plugins" ];
};
```

It adds Vim to your `systemPackages`, sets `$EDITOR` if `defaultEditor` is true,
and makes `/share/vim-plugins` available in the environment.

</details>

The following is a bat home-manager module that I wrote:

```nix
# bat.nix
{
  pkgs,
  config,
  lib,
  ...
}: let
  cfg = config.custom.batModule;
in {
  options.custom.batModule.enable = lib.mkOption {
    type = lib.types.bool;
    default = false;
    description = "Enable bat module";
  };

  config = lib.mkIf cfg.enable {
    programs.bat = {
      enable = true;
      themes = {
        dracula = {
          src = pkgs.fetchFromGitHub {
            owner = "dracula";
            repo = "sublime"; # Bat uses sublime syntax for its themes
            rev = "26c57ec282abcaa76e57e055f38432bd827ac34e";
            sha256 = "019hfl4zbn4vm4154hh3bwk6hm7bdxbr1hdww83nabxwjn99ndhv";
          };
          file = "Dracula.tmTheme";
        };
      };
      extraPackages = with pkgs.bat-extras; [
        batdiff
        batman
        prettybat
        batgrep
      ];
    };
  };
}
```

Now I could add this to my `home.nix` to enable it:

```nix
# home.nix
custom = {
  batModule.enable = true;
}
```

If I set this option to true the bat configuration is dropped in place. If it's
not set to true, it won't put the bat configuration in the system. Same as with
options defined in modules within the Nixpkgs repository.

If I had set the default to `true`, it would automatically enable the module
without requiring an explicit `custom.batModule.enable = true;` call in my
`home.nix`.

### Module Composition

NixOS achieves its full system configuration by combining the configurations
defined in various modules. This composition is primarily handled through the
`imports` mechanism.

`imports`: This is a standard option within a NixOS or Home Manager
configuration (often found in your configuration.nix or home.nix). It takes a
list of paths to other Nix modules. When you include a module in the imports
list, the options and configurations defined in that module become part of your
overall system configuration.

You declaratively state the desired state of your system by setting options
across various modules. The NixOS build system then evaluates and merges these
option settings. The culmination of this process, which includes building the
entire system closure, is represented by the derivation built by
`config.system.build.toplevel`.

### NixOS Modules and Dependency Locking with npins

<details>
<summary> ✔️ npins example (Click to Expand)</summary>
As our NixOS configurations grow in complexity, so too does the challenge of
managing the dependencies they rely on. Ensuring consistency and reproducibility
not only applies to individual packages but also to the versions of Nixpkgs and
other external resources our configurations depend upon.

Traditionally, NixOS configurations often implicitly rely on the version of
Nixpkgs available when `nixos-rebuild` is run. However, for more robust and
reproducible setups, especially in collaborative environments or when rolling
back to specific configurations, explicitly locking these dependencies to
specific versions becomes crucial.

In the following example, we'll explore how to use a tool called `npins` to
manage and lock the dependencies of a NixOS configuration, ensuring a more
predictable and reproducible system. This will involve setting up a project
structure and using npins to pin the specific version of Nixpkgs our
configuration relies on.

This is the file structure:

```bash
❯ tree
.
├── configuration.nix
├── default.nix
├── desktop.nix
└── npins
    ├── default.nix
    └── sources.json
```

This uses `npins` for dependency locking. Install it and run this in the project

directory:

```bash
npins init
```

Create a `default.nix` with the following:

```nix
# default.nix
{ system ? builtins.currentSystem, sources ? import ./npins, }:
let
  pkgs = import sources.nixpkgs {
    config = { };
    overlays = [ ];
  };
  inherit (pkgs) lib;
in lib.makeScope pkgs.newScope (self: {

  shell = pkgs.mkShell { packages = [ pkgs.npins self.myPackage ]; };

    # inherit lib;

  nixosSystem = import (sources.nixpkgs + "/nixos") {
    configuration = ./configuration.nix;
  };

  moduleEvale = lib.evalModules {
    modules = [
      # ...
    ];
  };
})
```

A `configuration.nix` with the following:

```nix
# configuration.nix
{
  boot.loader.grub.device = "nodev";
  fileSystems."/".device = "/devst";
  system.stateVersion = "25.05";

  # declaring options means to declare a new option
  # defining options means to define a value of an option
  imports = [
    # ./main.nix
     ./desktop.nix # Files
    # ./minimal.nix
  ];

  # mine.desktop.enable = true;
}
```

And a `desktop.nix` with the following:

```nix
# desktop.nix
{ pkgs, lib, config, ... }:

{
  imports = [];

  # Define an option to enable or disable desktop configuration
  options.mine.desktop.enable = lib.mkEnableOption "desktop settings";

  # Configuration that applies when the option is enabled
  config = lib.mkIf config.mine.desktop.enable {
    environment.systemPackages = [ pkgs.git ];
  };
}
```

`mkEnableOption` defaults to false. Now in your `configuration.nix` you can
uncomment `mine.desktop.enable = true;` to enable the desktop config and
vice-versa.

You can test that this works by running:

```bash
nix-instantiate -A nixosSystem.system
```

`nix-instantiate` performs only the evaluation phase of Nix expressions. During
this phase, Nix interprets the Nix code, resolves all dependencies, and
constructs derivations but does not execute any build actions. Useful for
testing.

To check if this worked and `git` is installed in systemPackages you can load it
into `nix repl` but first you'll want `lib` to be available so uncomment this in
your `default.nix`:

```nix
# default.nix
inherit lib;
```

Rerun `nix-instantiate -A nixosSystem.system`

Then load the repl and check that `git` is in `systemPackages`:

```bash
nix repl -f .
nix-repl> builtins.filter (pkg: lib.hasPrefix "git" pkg.name) nixosSystem.config.environment.systemPackages
```

This shows the path to the derivation

Check that mine.desktop.enable is true

```nix
nix-repl> nixosSystem.config.mine.desktop.enable
true
```

As demonstrated with npins, explicitly managing the dependencies of your NixOS
modules is a powerful technique for ensuring the long-term stability and
reproducibility of your system configurations. By pinning specific versions of
Nixpkgs and other resources, you gain greater control over your environment and
reduce the risk of unexpected changes due to upstream updates.

</details>

### Best Practices

You'll see the following all throughout Nix code and is convenient although it
doesn't follow best practices. One reason is static analysis can't reason about
the code (e.g. Because it implicitly brings all attributes into scope, tools
can't verify which ones are actually being used), because it would have to
actually evaluate the files to see which names are in scope:

```nix
# utils.nix
{ pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    rustup
    evcxr
    nix-prefetch-git
  ];
}
```

Another reason the above expression is considered an "anti-pattern" is when more
then one `with` is used, it's no longer clear where the names are coming from.

Scoping rules for `with` are not intuitive, see
[issue](https://github.com/NixOS/nix/issues/490) --nix.dev This can make
debugging harder, as searching for variable origins becomes ambiguous (i.e. open
to more than one interpretation).

The following follows best practices:

```nix
{pkgs, ... }: {
  environment.systemPackages = builtins.attrValues {
    inherit (pkgs)
      rustup
      evcxr
      nix-prefetch-git;
  };
}
```

- [Noogle builtins.attrValues](https://noogle.dev/f/builtins/attrValues)

<details>
<summary> ✔️ Above Command Summary (Click to Expand) </summary>

```nix
{
  inherit (pkgs) rustup evcxr nix-prefetch-git;
}
```

is equivalent to:

```nix
{
  rustup = pkgs.rustup;
  evcxr = pkgs.evcxr;
  nix-prefetch-git = pkgs.nix-prefetch-git;
}
```

Applying `builtins.attrValues` produces:

```nix
[ pkgs.evcxr pkgs.nix-prefetch-git pkgs.rustup ]
```

As you can see only the values are included in the list, not the keys. This is
more explicit and declarative but can be more complicated, especially for a
beginner.

`builtins.attrValues` returns the values of all attributes in the given set,
sorted by attribute name. The above expression turns into something like the
following avoiding bringing every attribute name from `nixpkgs` into scope.

A more straightforward example:

```nix
attrValues {c = 3; a = 1; b = 2;}
=> [1 2 3]
```

</details>

This approach avoids unintended name clashes or confusion when debugging.

Upon looking into this a bit further, most people use the following format to
avoid the "anti-pattern" from using `with pkgs;`:

```nix
# utils.nix
{ pkgs, ... }: {
  environment.systemPackages = [
    pkgs.rustup
    pkgs.evcxr
    pkgs.nix-prefetch-git
  ];
}
```

While the performance differences might be negligible on modern computers,
adopting this best practice from the start is highly recommended. The above
approach is more explicit, it's clear exactly where each package is coming from.

If maintaining strict scope control matters, use `builtins.attrValues`.

If readability and simplicity are more your priority, explicitly referencing
`pkgs.<packageName>` might be better. Now you can choose for yourself.

#### Conclusion

As we have seen throughout this chapter, modules are the building blocks of your
NixOS system and are themselves often functions. There are a few different ways
to use these modules to build your system. In the next chapter,
[Nix Flakes Explained](https://saylesss88.github.io/Nix_Flakes_Explained_4.html)
we will learn about Nix Flakes as a more modern and comprehensive entrypoint for
managing your entire system and its dependencies.

To further deepen your understanding of NixOS Modules and the broader ecosystem
of tools and best practices surrounding them, the following resources offer
valuable insights and information.

#### Resources on Modules

<details>
<summary> ✔️ Resources (Click to Expand) </summary>

- [WritingNixOsModules](https://nixos.org/manual/nixos/stable/#sec-writing-modules)

- [NixWikiNixOSModules](https://nixos.wiki/wiki/NixOS_modules)

- [nix.dev A basic module](https://nix.dev/tutorials/module-system/a-basic-module/index.html)

- [ModuleSystemDeepDive](https://nix.dev/tutorials/module-system/deep-dive#module-system-deep-dive)

- [xeiaso Nixos Modules for fun & profit](https://xeiaso.net/talks/asg-2023-nixos/)

- [NixOS Flakes Book Module System](https://nixos-and-flakes.thiscute.world/other-usage-of-flakes/module-system)

# Videos

[NixHour Writing NixOS modules](https://www.youtube.com/watch?v=N7hFP_40DJo&t=17s)
-- This example is from this video
[infinisilModules](https://infinisil.com/modules.mp4)

[tweagModuleSystemRecursion](https://www.youtube.com/watch?v=cZjOzOHb2ow)

</details>
