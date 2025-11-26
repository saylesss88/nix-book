---
title: Nix Flakes Explained
date: 2025-11-21
author: saylesss88
---

# Chapter 4

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![trees3](images/trees3.cleaned.png)

<!-- <img src="images/gruv15.png" width="800" height="600"> -->

## Nix Flakes Explained

If you're completely new, take a look at
[this](https://nixos.wiki/wiki/flakes#Installing_flakes) to get flakes on your
system.

For the Nix Flake man page type `man nix3 flake` and for a specific feature,
type something like `man nix3 flake-lock`.

Flakes replace stateful channels (which cause much confusion among novices) and
introduce a more intuitive and consistent CLI, making them a perfect opportunity
to start using Nix. -- Alexander Bantyev
[Practical Nix Flakes](https://serokell.io/blog/practical-nix-flakes)

The "state" being remembered and updated by channels is the specific revision of
the Nixpkgs repository that your local Nix installation considers "current" for
a given channel. When this state changes on your machine, your builds diverge
from others whose machines have a different, independently updated channel
state.

Channels are also constantly updated on the remote servers. So, "nixos-unstable"
today refers to a different set of packages and versions than "nixos-unstable"
did yesterday or will tomorrow.

Flakes solve this by making the exact revision of `nixpkgs` (and other
dependencies) an explicit input within your `flake.nix` file, pinned in the
`flake.lock`. This means the state is explicitly defined in the configuration
itself, not implicitly managed by a global system setting.

Evaluation time is notoriously slow on NixOS, the problem was that in the past
Nix evaluation wasn't hermetic preventing effective evaluation caching. A `.nix`
file can import other Nix files or by looking them up in the Nix search path
(`$NIX_PATH`). This causes a cached result to be inconsistent unless every file
is perfectly kept track of. Flakes solve this problem by ensuring fully hermetic
evaluation.

"Hermetic" means that the output of an evaluation (the derivation itself)
depends _only_ on the explicit inputs provided, not on anything external like
environment variables or pulling in files only on your system. This is the
problem that Nix solves and the problem that flakes are built around.

## What is a Nix Flake?

**Nix flakes** are independent components in the Nix ecosystem. They define
their own **dependencies** (inputs) and what they produce (outputs), which can
include **packages**, **deployment configurations**, or **Nix functions** for
other flakes to use.

Flakes provide a standardized framework for building and managing software,
making all project inputs explicit for greater reproducibility and
self-containment.

At its core, a flake is a source tree (like a Git repository) that contains a
`flake.nix` file in its root directory. This file provides a standardized way to
access Nix artifacts such as packages and modules.

Flakes provide a standard way to write Nix expressions (and therefore packages)
whose dependencies are version-pinned in a lock file, improving reproducibility
of Nix installations. -- NixOS Wiki

Think of `flake.nix` as the central entry point of a flake. It not only defines
what the flake produces but also declares its dependencies.

### Key Concepts

`flake.nix`: **The Heart of a Flake**

The `flake.nix` file is mandatory for any flake. It must contain an attribute
set with at least one required attribute: `outputs`. It can also optionally
include `description` and `inputs`.

**Basic Structure:**

```nix
{
  description = "Package description";
  inputs = { /* Dependencies go here */ };
  outputs = { /* What the flake produces */ };
  nixConfig = { /* Advanced configuration options */ };
}
```

I typically see `nixConfig` used for extra-substituters for cachix. This is a
general-purpose way to define Nix configuration options that apply when this
flake is evaluated or built. It ties into your `/etc/nix/nix.conf` or
`~/.config/nix/nix.conf`.

For example, create a directory and add a `flake.nix` with the following
contents, yes this is a complete `flake.nix` demonstrating _outputs_ being the
only required attribute:

```nix
# flake.nix
{
  outputs = _: { multiply = 2 * 2; };
}
```

Now evaluate it with:

```bash
nix eval .#multiply
4
```

In the `outputs = _: { ... };` line, the `_` (underscore) is a placeholder
argument. It represents the inputs that the outputs function could receive (like
`inputs`, `self`, `pkgs`, etc.), but in this specific case, we're not using any
of them to define the multiply attribute. It's a common convention in Nix to use
`_` when an argument is required by a function but intentionally ignored.

In the command `nix eval .#multiply`:

- the `.` signifies the current directory, indicating that Nix should look for a
  `flake.nix` file in the directory where you're running the command.

- The `#` is used to select a specific attribute from the `outputs` of the
  flake. In this case, it's telling Nix to evaluate the `multiply` attribute.

In the next example we will create a `devShells` output as well as a `packages`
output.

**`flake.lock` auto-generated lock file**

All flake inputs are pinned to specific revisions in a lockfile called
`flake.lock` This file stores the revision info as JSON.

The `flake.lock` file ensures that Nix flakes have purely deterministic outputs.
A `flake.nix` file without an accompanying `flake.lock` should be considered
incomplete and a kind of proto-flake. Any Nix CLI command that is run against
the flake—like `nix build`, `nix develop`, or even `nix flake show`—generates a
`flake.lock` for you.

Here’s an example section of a `flake.lock` file that pins Nixpkgs to a specific
revision:

```bash
$ cat flake.lock
{
  "nodes": {
    "nixpkgs": {
      "info": {
        "lastModified": 1587398327,
        "narHash": "sha256-mEKkeLgUrzAsdEaJ/1wdvYn0YZBAKEG3AN21koD2AgU="
      },
      "locked": {
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "5272327b81ed355bbed5659b8d303cf2979b6953",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-20.03",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 5
}
```

Any future build of this flake will use the version of `nixpkgs` recorded in the
lock file. If you add new inputs, they will be automatically added when you run
a nix flake command like `nix flake show`. But it won't replace existing locks.

If you need to update a locked input to the latest version:

```bash
nix flake lock --update-input nixpkgs
nix build
```

The above command allows you to update individual inputs, and `nix flake update`
will update the whole lock file.

### Helper functions that are good to know for working with Flakes

`lib.genAttrs`: A function, given the name of the attribute, returns the
attribute's value

Example:

```nix
nix repl
nix-repl> :l <nixpkgs>
nix-repl> lib.genAttrs [ "boom" "bash" ] (name: "sonic" + name)
```

**Output**:

```nix
{
  bash = "sonicbash";
  boom = "sonicboom";
}
```

You will often see the following:

A common use for this with flakes is to have a list of different systems:

```nix
     systems = [
       "x86_64-linux"
       "aarch64-linux"
       "x86_64-darwin"
       "aarch64-darwin"
     ];
```

And use it to generate an attribute set for each listed system:

```nix
eachSystem = lib.genAttrs systems;
```

The above command creates an attribute set by mapping over a list of system
strings. If you notice, you provide it a list (i.e. [ 1 2 3 ]) and the function
returns a set (i.e. `{ ... }`)

Why `genAttrs` is useful:

- It lets you define attributes (like `packages`, `checks`, `devShells`) per
  supported system in a DRY(don't repeat yourself), structured way.

- `lib.mapAttrs`: A function, given an attribute's name and value, returns a new
  `nameValuePair`.

Example:

```nix
nix-repl> builtins.mapAttrs (name: value: name + "-" + value) { x = "foo"; y = "bar"; }
```

**Output**:

```nix
{
  x = "x-foo";
  y = "y-bar";
}
```

`pkgs.mkShell`: is a specialized `stdenv.mkDerivation` that removes some
repetition when using it with `nix-shell` (or `nix develop`)

Example:

```nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = [ pkgs.gnumake ];

  inputsFrom = [ pkgs.hello pkgs.gnutar ];

  shellHook = ''
    export DEBUG=1
  '';
}
```

#### A Simple flake that outputs a devshell and a package

In a new directory create a `flake.nix`

```nix
# flake.nix
{
  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {

    packages.x86_64-linux.default = pkgs.kakoune; # You could define a meta-package here

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
        pkgs.kakoune
        pkgs.git
        pkgs.ripgrep
        pkgs.fzf
      ];
    };
  };
}
```

`mkShell` is a wrapper around `mkDerivation`

This flake offers two main outputs for `x86_64-linux` systems:

1. A **standard package** (`packages.x86_64-linux.default`): This simple example
   just re-exports `kakoune` from `nixpkgs`. You could build your own apps here.

2. A **development shell** (`devShells.x86_64-linux.default`): This provides a
   convenient environment where you have specific tools available without
   installing them globally on your system.

To use this flake you have a few options:

- `nix run` will launch kakoune

- `nix develop` will activate the development environment providing all of the
  pkgs listed under `mkShell`.

- Or more explicitly `nix develop .#devShells.x86_64-linux.default`, does the
  same thing as the command above.

#### Flake References

<details>
<summary> ✔️ Flake References (Click to Expand) </summary>

**Flake references** (flakerefs) are a way to specify the location of a flake.
They have two different formats:

> **Attribute set representation**:
>
> ```nix
> {
>   type = "github";
>   owner = "NixOS";
>   repo = "nixpkgs";
> }
> ```
>
> or **URL-like syntax**:
>
> ```nix
> github:NixOS/nixpkgs
> ```
>
> These are used on the command line as a more convenient alternative to the
> attribute set representation. For instance, in the command
>
> ```nix
> nix build github:NixOS/nixpkgs#hello
> ```
>
> `github:NixOS/nixpkgs` is a flake reference (while `hello` is an output
> attribute). They are also allowed in the `inputs` attribute of a flake, e.g.
>
> ```nix
> inputs.nixpkgs.url = "github:NixOS/nixpkgs";
> ```
>
> is equivalent to
>
> ```nix
> inputs.nixpkgs = {
>   type = "github";
>   owner = "NixOS";
>   repo = "nixpkgs";
> };
> ```
>
> --
> [nix.dev flake-references](https://nix.dev/manual/nix/2.24/command-ref/new-cli/nix3-flake#flake-references)

</details>

#### Nix Flake Commands

<details>
<summary> ✔️ Flake Commands (Click to Expand) </summary>

> `nix flake` provides subcommands for creating, modifying and querying _Nix
> Flakes_. Flakes are the unit for packaging Nix code in a reproducible and
> discoverable way. They can have dependencies on other flakes, making it
> possible to have multi-repository Nix projects.

— From
[nix.dev Reference Manual](https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake)

- The main thing to note here is that `nix flake` is used to manage Nix flakes
  and that Flake commands are whitespace separated rather than hyphen `-`
  separated.

- Flakes do provide some advantages when it comes to discoverability of outputs.

- For Example, two helpful commands to inspect a flake are:
  - [nix flake show](https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake-show)
    command: Show the outputs provided by a flake.

  - [nix flake check](https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake-check)
    command: check whether the flake evaluates and run its tests.

  - Any Nix CLI command that is run against a flake -- like `nix build`,
    `nix develop`, `nix flake show` -- generate a `flake.lock` file for you.
    - The `flake.lock` file ensures that all flake inputs are pinned to specific
      revisions and that Flakes have purely deterministic outputs.

  Example:

```bash
nix shell nixpkgs#ponysay --command ponysay "Flakes Rock!"
```

This works because of the [flake registry] that maps symbolic identifiers like
`nixpkgs` to actual locations such as `https://github.com/NixOS/nixpkgs`. So the
following are equivalent:

```bash
nix shell nixpkgs#ponysay --command ponysay Flakes Rock!
nix shell github:NixOS/nixpkgs#ponysay --command ponysay Flakes Rock!
```

To override the `nixpkgs` registry with your own local copy you could:

```bash
nix registry add nixpkgs ~/src/local-nixpkgs
```

</details>

### Attribute Sets: The Building Blocks

<details>
<summary> ✔️ Attribute set Refresher (Click to Expand) </summary>

**Attribute sets** are fundamental in Nix. They are simply collections of
name-value pairs wrapped in curly braces `{}`.

- Example, (click to see Output):

```nix
let
  my_attrset = { foo = "bar"; };
in
my_attrset.foo
~ "bar"
```

**Top-Level Attributes of a Flake**:

Flakes have specific **top-level attributes** that can be accessed directly
(without dot notation). The most common ones are `inputs`, `outputs`, and
`nixConfig`.

  </details>

### Deeper Dive into the Structure of `flake.nix`

<!-- ![Flakes](images/Flakes.png) -->

`inputs`: **Declaring Dependencies**

The `inputs` attribute set specifies the other flakes that your current flake
depends on.

Each key in the `inputs` set is a name you choose for the dependency, and the
value is a reference to that flake (usually a URL or a Git Repo).

To access something from a dependency, you generally go through the `inputs`
attribute (e.g., `inputs.helix.packages`).

See
[Nix Flake inputs](https://saylesss88.github.io/flakes/flake_inputs_4.1.html)
for a flake inputs deep dive.

**Example:** This declares dependencies on the `nixpkgs` and `import-cargo`
flakes:

```nix
inputs = {
  import-cargo.url = "github:edolstra/import-cargo";
  nixpkgs.url = "nixpkgs";
};
```

When Nix evaluates your flake, it fetches and evaluates each input. These
evaluated inputs are then passed as an attribute set to the outputs function,
with the keys matching the names you gave them in the inputs set.

The special input `self` is a reference to the `outputs` and the source tree of
the current flake itself.

**`outputs`: Defining What Your Flake Provides**

The **`outputs`** attribute defines what your flake makes available. This can
include packages, NixOS modules, development environments (`devShells`) and
other Nix derivations.

Flakes can output arbitrary Nix values. However, certain outputs have specific
meanings for Nix commands and must adhere to particular types (often
derivations, as described in the
[output schema](https://nixos.wiki/wiki/Flakes)).

You can inspect the outputs of a flake using the command:

```nix
nix flake show
```

> This command takes a flake URI and displays its outputs in a tree structure,
> showing the attribute paths and their corresponding types.

**Understanding the `outputs` Function**

Beginners often mistakenly think that self and nixpkgs within
`outputs = { self, nixpkgs, ... }: { ... }` are the outputs themselves. Instead,
they are the _input arguments_ (often called _output arguments_) to the outputs
function.

The outputs function in `flake.nix` always takes a single argument, which is an
attribute set. The syntax `{ self, nixpkgs, ... }` is Nix's way of destructuring
this single input attribute set to extract the values associated with the keys
`self` and `nixpkgs`.

Flakes output your whole system configuration, packages, as well as Nix
functions for use elsewhere.

- For example, the `nixpkgs` repository has its own `flake.nix` file that
  outputs many helper functions via the `lib` attribute.

- For a deep dive into flake outputs, see
  [Nix Flake Outputs](https://saylesss88.github.io/flakes/flake_outputs_4.2.html)

> The `lib` convention The convention of using `lib` to output functions is
> observed not just by Nixpkgs but by many other Nix projects. You’re free,
> however, to output functions via whichever attribute you prefer. --
> [Zero to Nix Flakes](https://zero-to-nix.com/concepts/flakes/#inputs)

Some flake outputs are required to be system specific (i.e. "x86_64-linux" for
(64-bit AMD/Intel Linux) including packages, development environments, and NixOS
configurations)

**Variadic Attributes (...) and @-patterns**

The `...` syntax in the input arguments of the outputs function indicates
variadic attributes, meaning the input attribute set can contain more attributes
than just those explicitly listed (like `lib` and `nixpkgs`).

**Example:**

```nix
mul = { a, b, ... }: a * b;
mul { a = 3; b = 4; c = 2; } # 'c' is an extra attribute
```

However, you cannot directly access these extra attributes within the function
body unless you use the @-pattern:

- (Click for Output)

```nix
mul = s@{ a, b, ... }: a  b  s.c; # 's' now refers to the entire input set
mul { a = 3; b = 4; c = 2; } # Output: 24
~ 24
```

When used in the outputs function argument list (e.g.,
`outputs = { pkgs, ... } @ inputs)`, the @-pattern binds the entire input
attribute set to a name (in this case, `inputs`) while also allowing you to
destructure specific attributes like pkgs.

**What `outputs = { pkgs, ... } @ inputs: { ... };` does:**

1. **Destructuring:** It tries to extract the value associated with the key
   `pkgs` from the input attribute set and binds it to the variable `pkgs`. The
   `...` allows for other keys in the input attribute set to be ignored during
   this direct destructuring.

2. **Binding the Entire Set:** It binds the entire input attribute set to the
   variable inputs.
   - Example `flake.nix`:

```nix
{
inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
inputs.home-manager.url = "github:nix-community/home-manager";

outputs = { self, nixpkgs, ... } @ attrs: { # A `packages` output for the x86_64-linux platform
packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;

    # A `nixosConfigurations` output (for a NixOS system named "fnord")
    nixosConfigurations.fnord = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = attrs;
      modules = [ ./configuration.nix ];
    };

};
}
```

**Platform Specificity in Outputs**

Flakes ensure that their outputs are consistent across different evaluation
environments. Therefore, any package-related output must explicitly specify the
target platform (a combination of architecture and OS, `x86_64-linux`).

**legacyPackages Explained**

`legacyPackages` is a way for flakes to interact with the traditional, less
structured package organization of nixpkgs. Instead of packages being directly
at the top level (e.g., `pkgs.hello`), `legacyPackages` provides a
platform-aware way to access them within the flake's structured output format
(e.g., `nixpkgs.legacyPackages.x86_64-linux.hello`). It acts as a bridge between
the flake's expected output structure and nixpkgs's historical organization.

**The Sole Argument of outputs**

It's crucial to remember that the outputs function accepts only one argument,
which is an attribute set. The `{ self, nixpkgs, ... }` syntax is simply
destructuring that single input attribute set.

**Outputs of the Flake (Return Value)**

The outputs of the flake refer to the attribute set that is returned by the
`outputs` function. This attribute set can contain various named outputs like
`packages`, `nixosConfigurations`, `devShells`, etc.

**Imports: Including Other Nix Expressions**

The `import` function in Nix is used to evaluate the Nix expression found at a
specified path (usually a file or directory) and return its value.

Basic Usage: import `./path/to/file.nix`

**Passing Arguments During Import**

`import <nixpkgs> {}` is calling two functions, not one.

1. `import <nixpkgs>`: The first function call

- `import` is a built-in Nix function. Its job is to load and evaluate a Nix
  expression from a specified path.

- `<nixpkgs>` is a flake reference. When you use `import <nixpkgs>`, Nix
  evaluates the `default.nix` file (or sometimes `lib/default.nix`) found at
  that location.

- The `default.nix` in `nixpkgs` evaluates to a function. This function is
  designed to be configurable, allowing you to pass arguments like `system`,
  `config`, etc. to customize how `nixpkgs` behaves and what packages it
  provides.

- So, `import <nixpkgs>` doesn't give you the `nixpkgs` package set directly; it
  gives you the function that generates the `nixpkgs` package set derivation.

2. `{}`: The second function call (and its argument)

- `{}` denotes an empty attribute set

- When an attribute set immediately follows a function, it means you are calling
  that function and passing the attribute set as its single argument.

So, the `{}` after `import <nixpkgs>` is not part of the `import` function
iteself. It's the argument being passed to the function that `import <nixpkgs>`
just returned.

You can also pass an attribute set as an argument to the Nix expression being
imported:

```nix
let
myHelpers = import ./lib/my-helpers.nix { pkgs = nixpkgs; };
in
# ... use myHelpers
```

In this case, the Nix expression in `./lib/my-helpers.nix` is likely a function
that expects an argument (often named `pkgs` by convention):

```nix
# ./lib/my-helpers.nix

{ pkgs }:
let
myPackage = pkgs.stdenv.mkDerivation {
name = "my-package"; # ...
};
in
myPackage
```

By passing `{ pkgs = nixpkgs; }` during the import, you are providing the
nixpkgs value from your current `flake.nix` scope to the pkgs parameter expected
by the code in `./lib/my-helpers.nix`.

**Importing Directories (`default.nix`)**

When you use import with a path that points to a directory, Nix automatically
looks for a file named `default.nix` within that directory. If found, Nix
evaluates the expressions within `default.nix` as if you had specified its path
directly in the import statement.

- For more advanced examples see
  [Nix Flake Examples](https://saylesss88.github.io/flakes/flake_examples_4.3.html)

##### Conclusion: Unifying Your Nix Experience with Flakes

For some examples of more advanced outputs like `devShells` and `checks`, check
out this blog post that I wrote:
[Nix Flakes Tips and Tricks](https://tsawyer87.github.io/posts/nix_flakes_tips/)

In this chapter, we've explored Nix Flakes as a powerful and modern approach to
managing Nix projects, from development environments to entire system
configurations. We've seen how they provide structure, dependency management,
and reproducibility through well-defined inputs and outputs. Flakes offer a
cohesive way to organize your Nix code and share it with others.

As we've worked with the flake.nix file, you've likely noticed its structure – a
top-level attribute set defining various outputs like devShells, packages,
nixosConfigurations, and more. These top-level attributes are not arbitrary;
they follow certain conventions and play specific roles within the Flake
ecosystem.

In the next chapter,
[Understanding Top-Level Attributes](https://saylesss88.github.io/Understanding_Top-Level_Attributes_5.html)
we will delve deeper into the meaning and purpose of these common top-level
attributes. We'll explore how they are structured, what kind of expressions they
typically contain, and how they contribute to the overall functionality and
organization of your Nix Flakes. Understanding these attributes is key to
effectively leveraging the full potential of Nix Flakes.

##### Further Resources

<details>
<summary> ✔️ Resources (Click to Expand)</summary>

- [practical-nix-flakes](https://serokell.io/blog/practical-nix-flakes)

- [Nix Flakes an Introduction](https://xeiaso.net/blog/nix-flakes-1-2022-02-21/)

- [tweag nix-flakes](https://www.tweag.io/blog/2020-07-31-nixos-flakes/)

- [NixOS-wiki Flakes](https://nixos.wiki/wiki/Flakes)

- [nix.dev flakes](https://nix.dev/concepts/flakes.html)

- [anatomy-of-a-flake](https://vtimofeenko.com/posts/practical-nix-flake-anatomy-a-guided-tour-of-flake.nix/)

- [flakes-arent-real](https://jade.fyi/blog/flakes-arent-real/)

- [wombats-book-of-nix](https://mhwombat.codeberg.page/nix-book/#_attribute_set_operations)

- [zero-to-nix flakes](https://zero-to-nix.com/concepts/flakes/)

- [nixos-and-flakes-book](https://nixos-and-flakes.thiscute.world/)

- [FlakeHub](https://flakehub.com/)

![FlakeHub](images/nixosnix.png)

</details>
