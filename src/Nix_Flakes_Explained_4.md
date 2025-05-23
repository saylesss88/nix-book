# Chapter 4

<!--toc:start-->

- [Nix Flakes Explained](#nix-flakes-explained)
- [What is a Nix Flake?](#what-is-a-nix-flake)
- [Key Concepts](#key-concepts)
- [Nix Flake Commands](#nix-flake-commands)
- [Attribute Sets: The Building Blocks](#attribute-sets-the-building-blocks)
  - [Anatomy of `flake.nix`](#anatomy-of-flakenix)
- [Conclusion: Unifying Your Nix Experience with Flakes](#conclusion-unifying-your-nix-experience-with-flakes)
- [Further Resources](#further-resources)

<!--toc:end-->

## Nix Flakes Explained

<img src="images/gruv15.png" width="800" height="600">

If you're completely new, take a look at
[this](https://nixos.wiki/wiki/flakes#Installing_flakes) to get flakes on your
system.

Flakes replace stateful channels (which cause much confusion among novices) and
introduce a more intuitive and consistent CLI, making them a perfect opportunity
to start using Nix. -- Alexander Bantyev
[Practical Nix Flakes](https://serokell.io/blog/practical-nix-flakes)

- The "state" being remembered and updated by channels is the specific revision
  of the Nixpkgs repository that your local Nix installation considers "current"
  for a given channel. When this state changes on your machine, your builds
  diverge from others whose machines have a different, independently updated
  channel state.

  - Channels are also constantly updated on the remote servers. So,
    "nixos-unstable" today refers to a different set of packages and versions
    than "nixos-unstable" did yesterday or will tomorrow.

- Flakes solve this by making the exact revision of `nixpkgs` (and other
  dependencies) an explicit input within your `flake.nix` file, pinned in the
  `flake.lock`. This means the state is explicitly defined in the configuration
  itself, not implicitly managed by a global system setting.

## What is a Nix Flake?

- **Nix flakes** are independent components in the Nix ecosystem. They define
  their own **dependencies** (inputs) and what they produce (outputs), which can
  include **packages**, **deployment configurations**, or **Nix functions** for
  other flakes to use.

- Flakes provide a standardized framework for building and managing software,
  making all project inputs explicit for greater reproducibility and
  self-containment.

- At its core, a flake is a source tree (like a Git repository) that contains a
  `flake.nix` file in its root directory. This file provides a standardized way
  to access Nix artifacts such as packages and modules.

- Flakes provide a standard way to write Nix expressions (and therefore
  packages) whose dependencies are version-pinned in a lock file, improving
  reproducibility of Nix installations. -- NixOS Wiki

- Think of `flake.nix` as the central entry point of a flake. It not only
  defines what the flake produces but also declares its dependencies.

## Key Concepts

`flake.nix`: **The Heart of a Flake**

- The `flake.nix` file is mandatory for any flake. It must contain an attribute
  set with at least one required attribute: `outputs`. It can also optionally
  include `description` and `inputs`.
- **Basic Structure:**

```nix
{
  description = "Package description";
  inputs = { /* Dependencies go here */ };
  outputs = { /* What the flake produces */ };
  nixConfig = { /* Advanced configuration options */ };
}
```

## Flake References

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

## Nix Flake Commands

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

  </details>

## Attribute Sets: The Building Blocks

<details>
<summary> ✔️ Attribute set Refresher (Click to Expand) </summary>

- **Attribute sets** are fundamental in Nix. They are simply collections of
  name-value pairs wrapped in curly braces `{}`.

  - Example, (click to see Output):

  ```nix
  let
    my_attrset = { foo = "bar"; };
  in
  my_attrset.foo
  ~ "bar"
  ```

- **Top-Level Attributes of a Flake**:

  - Flakes have specific **top-level attributes** that can be accessed directly
    (without dot notation). The most common ones are `inputs`, `outputs`, and
    `nixConfig`.

  </details>

### Anatomy of `flake.nix`

![Flakes](images/Flakes.png)

`inputs`: **Declaring Dependencies**

- The `inputs` attribute set specifies the other flakes that your current flake
  depends on.

- Each key in the `inputs` set is a name you choose for the dependency, and the
  value is a reference to that flake (usually a URL or a Git Repo).

- To access something from a dependency, you generally go through the `inputs`
  attribute (e.g., `inputs.helix.packages`).

  - **Example:** This declares dependencies on the `nixpkgs` and `import-cargo`
    flakes:

  ```nix
  inputs = {
    import-cargo.url = "github:edolstra/import-cargo";
    nixpkgs.url = "nixpkgs";
  };
  ```

  - When Nix evaluates your flake, it fetches and evaluates each input. These
    evaluated inputs are then passed as an attribute set to the outputs
    function, with the keys matching the names you gave them in the inputs set.

  - The special input `self` is a reference to the `outputs` and the source tree
    of the current flake itself.

**`outputs`: Defining What Your Flake Provides**

- The **`outputs`** attribute defines what your flake makes available. This can
  include packages, NixOS modules, development environments (`devShells`) and
  other Nix derivations.

- Flakes can output arbitrary Nix values. However, certain outputs have specific
  meanings for Nix commands and must adhere to particular types (often
  derivations, as described in the
  [output schema](https://nixos.wiki/wiki/Flakes)).

- You can inspect the outputs of a flake using the command:

```nix
nix flake show
```

> This command takes a flake URI and displays its outputs in a tree structure,
> showing the attribute paths and their corresponding types.

**Understanding the `outputs` Function**

- Beginners often mistakenly think that self and nixpkgs within
  `outputs = { self, nixpkgs, ... }: { ... }` are the outputs themselves.
  Instead, they are the _input arguments_ (often called _output arguments_) to
  the outputs function.

- The outputs function in `flake.nix` always takes a single argument, which is
  an attribute set. The syntax `{ self, nixpkgs, ... }` is Nix's way of
  destructuring this single input attribute set to extract the values associated
  with the keys self and nixpkgs.

- Flakes output your whole system configuration, packages, and also Nix
  functions for use elsewhere.

  - For example, the `nixpkgs` repository has its own `flake.nix` file that
    outputs many helper functions via the `lib` attribute.

> The `lib` convention The convention of using `lib` to output functions is
> observed not just by Nixpkgs but by many other Nix projects. You’re free,
> however, to output functions via whichever attribute you prefer. --
> [Zero to Nix Flakes](https://zero-to-nix.com/concepts/flakes/#inputs)

- Some flake outputs are required to be system specific (i.e. "x86_64-linux" for
  (64-bit AMD/Intel Linux) including packages, development environments, and
  NixOS configurations)

**Referencing the Current Flake** (`self`)

- `self` provides a way to refer back to the current flake from within the
  outputs function. You can use it to access other top-level attributes like
  inputs (e.g., `self.inputs`).

- The outputs function always receives an argument conventionally named self,
  which represents the entire flake, including all its top-level attributes.
  You'll typically use self to reference things defined within your own flake
  (e.g., `self.packages.my-package`).

**Variadic Attributes (...) and @-patterns**

- The `...` syntax in the input arguments of the outputs function indicates
  variadic attributes, meaning the input attribute set can contain more
  attributes than just those explicitly listed (like `self` and `nixpkgs`).

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

  - When used in the outputs function argument list (e.g.,
    `outputs = { pkgs, ... } @ inputs)`, the @-pattern binds the entire input
    attribute set to a name (in this case, `inputs`) while also allowing you to
    destructure specific attributes like pkgs.

  - **What `outputs = { pkgs, ... } @ inputs: { ... };` does:**

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

- Flakes ensure that their outputs are consistent across different evaluation
  environments. Therefore, any package-related output must explicitly specify
  the target platform (a combination of architecture and OS, `x86_64-linux`).

**legacyPackages Explained**

- `legacyPackages` is a way for flakes to interact with the traditional, less
  structured package organization of nixpkgs. Instead of packages being directly
  at the top level (e.g., `pkgs.hello`), `legacyPackages` provides a
  platform-aware way to access them within the flake's structured output format
  (e.g., `nixpkgs.legacyPackages.x86_64-linux.hello`). It acts as a bridge
  between the flake's expected output structure and nixpkgs's historical
  organization.

**The Sole Argument of outputs**

- It's crucial to remember that the outputs function accepts only one argument,
  which is an attribute set. The `{ self, nixpkgs, ... }` syntax is simply
  destructuring that single input attribute set.

**Outputs of the Flake (Return Value)**

- The outputs of the flake refer to the attribute set that is returned by the
  `outputs` function. This attribute set can contain various named outputs like
  `packages`, `nixosConfigurations`, `devShells`, etc.

**Imports: Including Other Nix Expressions**

- The `import` function in Nix is used to evaluate the Nix expression found at a
  specified path (usually a file or directory) and return its value.

- Basic Usage: import `./path/to/file.nix`

**Passing Arguments During Import**

- You can also pass an attribute set as an argument to the Nix expression being
  imported:

```nix
let
myHelpers = import ./lib/my-helpers.nix { pkgs = nixpkgs; };
in
# ... use myHelpers
```

- In this case, the Nix expression in `./lib/my-helpers.nix` is likely a
  function that expects an argument (often named `pkgs` by convention):

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

- By passing `{ pkgs = nixpkgs; }` during the import, you are providing the
  nixpkgs value from your current `flake.nix` scope to the pkgs parameter
  expected by the code in `./lib/my-helpers.nix`.

**Importing Directories (`default.nix`)**

- When you use import with a path that points to a directory, Nix automatically
  looks for a file named `default.nix` within that directory. If found, Nix
  evaluates the expressions within `default.nix` as if you had specified its
  path directly in the import statement.

## Conclusion: Unifying Your Nix Experience with Flakes

For some examples of more advanced outputs like `devShells` and `checks`, check
out this blog post:
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

## Further Resources

<details>
<summary> ✔️ Resources (Click to Expand)</summary>

- [practical-nix-flakes](https://serokell.io/blog/practical-nix-flakes)

- [Nix Flakes an Introduction](https://xeiaso.net/blog/nix-flakes-1-2022-02-21/)

- [tweag nix-flakes](https://www.tweag.io/blog/2020-07-31-nixos-flakes/)

- [NixOS-wiki Flakes](https://nixos.wiki/wiki/Flakes)

- [nix.dev flakes](https://nix.dev/concepts/flakes.html)

- [flakes-arent-real](https://jade.fyi/blog/flakes-arent-real/)

- [wombats-book-of-nix](https://mhwombat.codeberg.page/nix-book/#_attribute_set_operations)

- [zero-to-nix flakes](https://zero-to-nix.com/concepts/flakes/)

- [nixos-and-flakes-book](https://nixos-and-flakes.thiscute.world/)

- [FlakeHub](https://flakehub.com/)

![FlakeHub](images/nixosnix.png)

</details>
