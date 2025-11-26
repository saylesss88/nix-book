---
title: Top-Level Attributes
date: 2025-11-21
author: saylesss88
---

# Chapter 5

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![coding1](images/coding1.png) -->

<img src="images/gruv9.png" width="800" height="600">

## Understanding Top-Level Attributes in NixOS Modules

This explanation is based on insights from Infinisil, a prominent figure in the
Nix community, to help clarify the concept of top-level attributes within NixOS
modules.

### The Core of a NixOS System: `system.build.toplevel`

<details>
<summary> ✔️ `system.build.toplevel` Explained (Click to Expand) </summary>

In a NixOS system, everything is built from a single "system derivation." The
command `nix-build '<nixpkgs/nixos>' -A system` initiates this build process.

The `-A system` part tells Nix to focus on the `system` attribute defined in the
`'<nixpkgs/nixos>'` file (which is essentially `./default.nix` within the
Nixpkgs repository).

This `system` attribute is specifically the NixOS option `system.build.toplevel`
. Think of `system.build.toplevel` as the **very top of the configuration
hierarchy** for your entire NixOS system. Almost every setting you configure
eventually influences this top-level derivation, often through a series of
intermediate steps.

**Key Takeaway:** `system.build.toplevel` is the ultimate output that defines
your entire NixOS system.

</details>

### How Options Relate: A Chain of Influence

Options in NixOS are not isolated; they often build upon each other.

<details>
<summary>Example: Nginx Option Chain (Click to Expand)</summary>

Here's an example of how a high-level option can lead down to a low-level system
configuration:

- You enable Nginx with `services.nginx.enable = true;`.
- This setting influences the lower-level option `systemd.services.nginx`.
- Which, in turn, affects the even lower-level option
  `systemd.units."nginx.service"`.
- Ultimately, this leads to the creation of a systemd unit file within
  `environment.etc."systemd/system"`.
- Finally, this unit file ends up as `result/etc/systemd/system/nginx.service`
  within the final `system.build.toplevel` derivation.

</details>

**Key Takeaway:** Higher-level, user-friendly options are translated into
lower-level system configurations that are part of the final system build.

### The NixOS Module System: Evaluating Options

So, how do these options get processed and turned into the final system
configuration? That's the job of the **NixOS module system**, located in the
`./lib` directory of Nixpkgs (specifically in `modules.nix`, `options.nix`, and
`types.nix`).

Interestingly, the module system isn't exclusive to NixOS; you can use it to
manage option sets in your own Nix projects.

Here's a simplified example of using the module system outside of NixOS:

```nix
let
  systemModule = { lib, config, ... }: {
    options.toplevel = lib.mkOption {
      type = lib.types.str;
    };

    options.enableFoo = lib.mkOption {
      type = lib.types.bool;
      default = false;
    };

    config.toplevel = ''
      Is foo enabled? ${lib.boolToString config.enableFoo}
    '';
  };

  userModule = {
    enableFoo = true;
  };

in (import <nixpkgs/lib>).evalModules {
  modules = [ systemModule userModule ];
}
```

**You can evaluate the `config.toplevel` option from this example using:**

```bash
nix-instantiate --eval file.nix -A config.toplevel
```

**Key Takeaway**: The NixOS module system is responsible for evaluating and
merging option configurations from different modules.

### How the Module System Works: A Simplified Overview

The module system processes a set of "modules" through these general steps:

<details>
<summary> ✔️ Detailed Steps (Click to Expand)</summary>

1. **Importing Modules**: It recursively finds and includes all modules
   specified in `imports = [ ... ];` statements.

2. **Declaring Options**: It collects all option declarations defined using
   `options = { ... };` from all the modules and merges them. If the same option
   is declared in multiple modules, the module system handles this (details
   omitted for simplicity).

3. **Defining Option Values**: For each declared option, it gathers all the
   value assignments (defined using `config = { ... };` or directly at the top
   level if no `options` or `config` are present) from all modules and merges
   them according to the option's defined type.

> **Important Note**: Option evaluation is lazy, meaning an option's value is
> only computed when it's actually needed. It can also depend on the values of
> other options.

</details>

**Key Takeaway**: The module system imports, declares, and then evaluates option
values from various modules to build the final configuration.

**Top-Level Attributes in a Module: `imports`, `options`, and `config`**

Within a NixOS module (the files that define parts of your system configuration)
, the attributes defined directly at the top level of the module's function have
specific meanings:

- `imports`: This attribute is a list of other module files to include. Their
  options and configurations will also be part of the evaluation.

- `options`: This attribute is where you declare new configuration options. You
  define their type, default value, description, etc., using functions like
  `lib.mkOption` or `lib.mkEnableOption`.

- `config`: This attribute is where you assign values to the options that have
  been declared (either in the current module or in imported modules).

**Key Takeaway**: The top-level attributes `imports`, `options`, and `config`
are the primary ways to structure a NixOS module.

**The Rule: Move Non-Option Attributes Under `config`**

If you define either an `options` or a `config` attribute at the top level of
your module, any other attributes that are not option declarations must be moved
inside the config attribute.

<details>
<summary> ✔️ Examples of Correct and Incorrect Usage (Click to Expand)</summary>

Let's look at an example of what not to do:

```nix
{ pkgs, lib, config, ... }:
{
imports = [];

# Defining an option at the top level

options.mine.desktop.enable = lib.mkEnableOption "desktop settings";

# This will cause an error because 'environment' and 'appstream'

# are not 'options' and 'config' is also present at the top level.jjjj

environment.systemPackages =
lib.mkIf config.appstream.enable [ pkgs.git ];

appstream.enable = true;
}
```

This will result in the error:
`error: Module has an unsupported attribute 'appstream' This is caused by introducing a top-level 'config' or 'options' attribute. Add configuration attributes immediately on the top level instead, or move all of them into the explicit 'config' attribute`.

**Key Takeaway**: When you have `options` or `config` at the top level, all
value assignments need to go inside the config block.

**The Correct Way**): Using the `config` Attribute

To fix the previous example, you need to move the value assignments for
`environment.systemPackages` and `appstream.enable` inside the config attribute:

```nix
{ pkgs, lib, config, ... }:
{
imports = [];

# Defining an option at the top level

options.mine.desktop.enable = lib.mkEnableOption "desktop settings";

config = {
environment.systemPackages =
lib.mkIf config.appstream.enable [ pkgs.git ];

    appstream.enable = true;

};
}
```

Now, Nix knows that you are declaring an option (`options.mine.desktop.enable`)
and then setting values for other options (`environment.systemPackages`,
`appstream.enable`) within the `config` block.

**Key Takeaway**: The `config` attribute is used to define the values of
options.

**Implicit `config`: When `options` is Absent**

If your module does not define either `options` or `config` at the top level,
then any attributes you define directly at the top level are implicitly treated
as being part of the config.

For example, this is valid:

```nix
{ pkgs, lib, config, ... }:
{
environment.systemPackages =
lib.mkIf config.appstream.enable [ pkgs.git ];

appstream.enable = true;
}
```

Nix will implicitly understand that `environment.systemPackages` and
`appstream.enable` are configuration settings.

**Key Takeaway**: If no explicit options or config are present, top-level
attributes are automatically considered part of the configuration.

**Removing an Option: What Happens to `config`**

Even if you remove the `options` declaration from a module that has a `config`
section, the `config = { environment.systemPackages = ... };` part will still
function correctly, assuming the option it's referencing (`appstream.enable` in
this case) is defined elsewhere (e.g., in an imported module).

</details>

**Key Takeaway**: The `config` section defines values for options, regardless of
whether those options are declared in the same module.

#### Conclusion

Understanding the nuances of top-level attributes within NixOS modules,
particularly `imports`, `options`, and `config`, is fundamental to structuring
and managing your system's configuration effectively. As we've seen, the module
system provides a powerful and declarative way to define and evaluate system
settings, ultimately contributing to the construction of the
`system.build.toplevel` derivation that represents your entire NixOS
environment.

The concepts of option declaration and value assignment, along with the crucial
rule of organizing non-option attributes under the `config` attribute when
`options` is present, provide a clear framework for building modular and
maintainable configurations.

Now that we have a solid grasp of how NixOS modules are structured and how they
contribute to the final system derivation, it's a natural next step to explore
the tangible results of these configurations: the software and system components
themselves. These are built and managed by a core concept in Nix, known as
**derivations**.

In the next chapter,
[Package Definitions Explained](https://saylesss88.github.io/Package_Definitions_Explained_6.html)
we will shift our focus from the abstract configuration to the concrete software
packages. We will learn how Nix uses _package definitions_ to create
_derivations_, which are the actual build plans that produce the software we use
on our NixOS systems. This will bridge the gap between configuring your system
and understanding how the software within it is managed.
