---
title: Intro to Derivations
date: 2025-11-29
author: saylesss88
collection: "notes"
tags: ["notes", "derivations"]
draft: false
---

# Chapter 7

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

## Introduction to Nix Derivations

![gruv10](images/gruv10.png)

Nix's build instructions, known as **derivations**, are defined using the Nix
Language. These derivations can describe anything from individual software
packages to complete system configurations. The Nix package manager then
deterministically "realizes" (builds) these derivations, ensuring consistency
because they rely solely on a predefined set of inputs.

Most things in NixOS are built around derivations. Your NixOS system is
described by such a single system derivation. When you want to apply a new
configuration, `nixos-rebuild` handles the process:

It first builds this derivation:

```bash
nix-build '<nixpkgs/nixos>' -A system
```

Then, once the build is complete, it switches to that new system:

```bash
result/bin/switch-to-configuration
```

After the build, `nixos-rebuild` updates a crucial symbolic link:
`/run/current-system` This symlink always points to the active, running version
of your system in the Nix store. In essence, the `/run/current-system` path is
the currently active system derivation. This design choice gives NixOS its
powerful atomic upgrade and rollback capabilities: changing your system involves
building a new system derivation and updating this symlink to point to the
latest version.

> ```nix
>  ls -lsah /run/current-system
>  0 lrwxrwxrwx 1 root root 85 May 23 12:11 /run/current-system -> /nix/store/
>  cy2c0kxpjrl7ajlg9v3zh898mhj4dyjv-nixos-system-magic-25.11.20250520.2795c50
> ```

- The `->` indicates a symlink and it's pointing to a **store path** which is
  the result of a derivation being built (the system closure)

- For beginners, the analogy of a cooking recipe is helpful:
  - **Ingredients (Dependencies):** What other software or libraries are needed.

  - **Steps (Build Instructions):** The commands to compile, configure, and
    install.

  - **Final Dish (Output):** The resulting package or resource.

A Nix derivation encapsulates all this information, telling Nix what inputs to
use, how to build it, and what the final output should be.

Nix derivations run in **pure**, **isolated environments**, meaning they
**cannot** access the internet during the build phase. This ensures that builds
are reproducible -- they don't depend on external sources that might change over
time.

There are `Fixed-output-derivations` that allow fetching resources during the
build process by explicitly specifying the expected hash upfront. Just keep this
in mind that normal derivations don't have network access.

## Creating Derivations in Nix

The primary way to define packages in Nix is through the `mkDerivation`
function, which is part of the standard environment (`stdenv`). While a
lower-level `derivation` function exists for advanced use cases, `mkDerivation`
simplifies the process by automatically managing dependencies and the build
environment.

`mkDerivation` (and `derivation`) takes a set of attributes as its argument. At
a minimum, you'll often encounter these essential attributes:

1.  **name:** A human-readable identifier for the derivation (e.g., "foo",
    "hello.txt"). This helps you and Nix refer to the package.

2.  **system:** Specifies the target architecture for the build (e.g.,
    `builtins.currentSystem` for your current machine).

3.  **builder:** Defines the program that will execute the build instructions
    (e.g., `bash`).

**How do we pass these required attributes to the `derivation` function?**

Functions in Nix often take a single argument which is an attribute set. For
`derivation` and `mkDerivation`, this takes the form
`functionName { attribute1 = value1; attribute2 = value2; ... }`, where the `{}`
encloses the set of attributes being passed as the function's argument.

Remember that `derivation` and `mkDerivation` take a set (i.e. `{}`) of
attributes as its first argument. So, in order to pass the required attributes
you would do something like this:

```nix
nix-repl> pkgs = import <nixpkgs> {}

nix-repl> d = derivation {
            name = "mydrv";
            builder = "${pkgs.bash}/bin/bash";
            args = [
              "-c" # Tells bash to execute the following string as a command
              ''
                # Explicitly set PATH to include coreutils bin directory
                export PATH="${pkgs.coreutils}/bin:$PATH"
                mkdir $out
              ''
            ];
            system = builtins.currentSystem;
          }

nix-repl> :b d
```

- When I was starting out, seeing the above written in the following format made
  it clearer in my mental map that we were passing these attributes as arguments
  but both accomplish the same thing.

```nix
d = derivation { name = "myname"; builder = "${coreutils}/bin/true"; system = builtins.currentSystem; }
```

- When you write `pkgs = import <nixpkgs> {};`, you are importing the Nixpkgs
  `default.nix` file, which resolves to a function. Calling that function by
  passing it an empty attribute set `{}` as its argument. The function then
  evaluates and returns the entire `pkgs` attribute set. To specify a different
  system for example, you could do something like:

```nix
pkgsForAarch64 = import <nixpkgs> { system = "aarch64-linux"; };
```

So when you see:

```nix
import <nixpkgs> { overlays = []; config = {}; }
```

- Instead, these empty sets explicitly override any global or implicit
  overlays/configurations that Nix might otherwise pick up from environment
  variables (like `NIXPKGS_CONFIG`), default locations (like
  `~/.config/nixpkgs/config.nix` or `~/.config/nixpkgs/overlays`), or other
  mechanisms.

- This is to prevent accidental partial application from other parts of your
  configuration and is saying "Do not pass any custom configuration options for
  this particular import"

- `derivation` is a pre-made, built-in function in the Nix language. Here, we
  are passing it an attribute set as argument with the three required
  attributes. (`name`, `builder`, `system`, and we added an extra argument
  `args`.)

## The Hello World Derivation

For this example, first create a `hello` directory and add the
[Hello tarball](https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz) to said
directory.

Now lets create the classic Hello derivation:

```nix
# hello.nix
let
  pkgs = import <nixpkgs> { };
in
derivation {
  name = "hello";
  builder = "${pkgs.bash}/bin/bash";
  args = [ ./hello_builder.sh ];
  inherit (pkgs)
    gnutar
    gzip
    gnumake
    gcc
    coreutils
    gawk
    gnused
    gnugrep
    ;
  bintools = pkgs.binutils.bintools;
  src = ./hello-2.12.1.tar.gz;
  system = builtins.currentSystem;
}
```

- As you can see, this isn't the only required file but is a recipe outlining
  how to build the `hello` package. The `tar.gz` package can be found
  [here](https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz) You would just place
  the tarball in the same directory as the derivation along with the following
  `hello_builder.sh`:

```bash
# hello_builder.sh
export PATH="$gnutar/bin:$gcc/bin:$gnumake/bin:$coreutils/bin:$gawk/bin:$gzip/bin:$gnugrep/bin:$gnused/bin:$bintools/bin"
tar -xzf $src
cd hello-2.12.1
./configure --prefix=$out
make
make install
```

And build it with:

```bash
nix-build hello.nix
```

Finally execute it with:

```bash
./result/bin/hello
Hello, world!
```

## Simple Rust Derivation

Create a `simple.rs` with the following contents:

```rust
fn main() {
  println!("Simple Rust!")
}
```

And a `rust_builder.sh` like this (this is our builder script):

```bash
# rust_builder.sh
# Set up the PATH to include rustc coreutils and gcc
export PATH="$rustc/bin:$coreutils/bin:$gcc/bin"

# IMPORTANT: Create the $out directory BEFORE rustc tries to write to it
mkdir -p "$out"

# Compile the Rust source code and place the executable inside $out
rustc -o "$out/simple_rust" "$src"
```

Now we'll enter the `nix repl` and build it:

```bash
❯ nix repl
Nix 2.28.3
Type :? for help.

nix-repl> :l <nixpkgs>
added 3950 variables.

# Define the variables for rustc, coreutils, bash, AND gcc from the loaded nixpkgs
nix-repl> rustc = pkgs.rustc

nix-repl> coreutils = pkgs.coreutils

nix-repl> bash = pkgs.bash

nix-repl> gcc = pkgs.gcc

# Now define the derivation
nix-repl> simple_rust_program = derivation {
            name = "simple-rust-program";
            builder = "${bash}/bin/bash";
            args = [ ./rust_builder.sh ];
            rustc = rustc;
            coreutils = coreutils;
            gcc = gcc;
            src = ./simple.rs;
            system = builtins.currentSystem;
          }

nix-repl> :b simple_rust_program
This derivation produced the following outputs:
out -> /nix/store/fmyqr2d3ph0lpnxd0xppwvwyhv3iyb7y-simple-rust-program
```

```bash
nix-store -r /nix/store/fmyqr2d3ph0lpnxd0xppwvwyhv3iyb7y-simple-rust-program

warning: you did not specify '--add-root'; the result might be removed by the garbage collector
/nix/store/fmyqr2d3ph0lpnxd0xppwvwyhv3iyb7y-simple-rust-program
```

This simple Rust example, built with a direct derivation call, illustrates:

- How Nix explicitly manages every single tool in your build environment
  (`bash`, `rustc`, `gcc`, `coreutils`).

- The strict isolation of Nix builds, where nothing is implicitly available.

- The deterministic mapping of inputs to unique output paths in the Nix store.

- The above example shows the fundamental structure of a Nix derivation, how
  it's defined within the `nix-repl`.

- `.drv` files are intermediate files that describe how to build a derivation;
  it's the bare minimum information.

## When Derivations are Built

Nix doesn't build derivations during the evaluation of your Nix expressions.
Instead, it processes your code in two main phases (and why you need to use
`:b simple_rust_program` or `nix-store -r` to actually build or realize it):

1.  Evaluation/Instantiate Phase: This is when Nix parses and interprets your
    .nix expression. The result is a precise derivation description (often
    represented as a .drv file on disk), and the unique "out paths" where the
    final built products will go are calculated. No actual code is compiled or
    executed yet. Achieved with `nix-instantiate`

2.  Realize/Build Phase: Only after a derivation has been fully described does
    Nix actually execute its build instructions. It first ensures all the
    derivation's inputs (dependencies) are built, then runs the builder script
    in an isolated environment, and places the resulting products into their
    designated "out paths" in the Nix store. Achieved with `nix-store -r`

## Referring to other derivations

The way that we can refer to other packages/derivations is to use the `outPath`.

The `outPath` describes the location of the files of that derivation. Nix can
then convert the derivation set into a string:

```bash
nix repl
nix-repl> :l <nixpkgs>
nix-repl> fzf
«derivation /nix/store/vw1zag9q4xvf10z24j1qybji7wfsz78v-fzf-0.62.0.drv»
nix-repl> fzf.outPath
"/nix/store/z3ayhjslz72ldiwrv3mn5n7rs96p2g8a-fzf-0.62.0"
nix-repl> builtins.toString fzf
"/nix/store/z3ayhjslz72ldiwrv3mn5n7rs96p2g8a-fzf-0.62.0"
```

- As long as there is an `outPath` attribute, Nix will do the "set to string
  conversion".

## Produce a development shell from a derivation

Building on the concept of a derivation as a recipe, let's create our first
practical derivation. This example shows how to define a temporary development
environment (a shell) using stdenv.mkDerivation, which is the primary function
for defining packages in Nix.

```nix
# my-shell.nix
# We use a `let` expression to bring `pkgs` and `stdenv` into scope.
# This is a recommended practice over `with import <nixpkgs> {}`
# for clarity and to avoid potential name collisions.
let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv; # Access stdenv from the imported nixpkgs
in

# Make a new "derivation" that represents our shell
stdenv.mkDerivation {
  name = "my-environment";

  # The packages in the `buildInputs` list will be added to the PATH in our shell
  buildInputs = [
    # cowsay is an arbitrary package
    # see https://nixos.org/nixos/packages.html to search for more
    pkgs.cowsay
    pkgs.fortune
  ];
}
```

**Usage**

```bash
nix-shell my-shell.nix
fortune | cowsay
 _________________________________________
/ "Lines that are parallel meet at        \
| Infinity!" Euclid repeatedly, heatedly, |
| urged.                                  |
|                                         |
| Until he died, and so reached that      |
| vicinity: in it he found that the       |
| damned things diverged.                 |
|                                         |
\ -- Piet Hein                            /
 -----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

- To exit type: `exit`

This Nix expression defines a temporary development shell. Let's break it down:

- `pkgs = import <nixpkgs> {};`: Standard way to get access to all the packages
  and helper functions (i.e. `nixpkgs.lib`)

- `stdenv = pkgs.stdenv;`: `stdenv` provides us `mkDerivation` and is from the
  `nixpkgs` collection.

- `stdenv.mkDerivation { ... };`: This is the core function for creating
  packages.
  - `stdenv` provides a set of common build tools and conventions.

- `mkDerivation` takes an attribute set (a collection of key-value pairs) as its
  argument.

- `name = "my-environment";`: This gives your derivation a human-readable name.

- `buildInputs = [ pkgs.cowsay ];`: This is a list of dependencies that will be
  available in the build environment of this derivation (or in the `PATH` if you
  enter the shell created by this derivation). `pkgs.cowsay` refers to the
  `cowsay` package from the imported `pkgs` collection.

The command `nix-instantiate --eval my-shell.nix` evaluates the Nix expression
in the file. It does not build the derivation. Instead, it returns the Nix value
that the expression evaluates to.

```bash
nix-instantiate --eval my-shell.nix
```

This value is a structured data type that encapsulates all the attributes (like
`name`, `system`, `buildInputs`, etc.) required to build the derivation. Your
output shows this detailed internal representation of the derivation's "recipe"
as understood by Nix. This is useful for debugging and inspecting the
derivation's definition.

## Our Second Derivation: Understanding the Builder

<details>
<summary> Understanding the Builder (Click to Expand) </summary>

- To understand how derivations work, let's create a very basic example using a
  bash script as our `builder`.

### Why a Builder Script?

- The `builder` attribute in a derivation tells Nix _how_ to perform the build
  steps. A simple and common way to define these steps is with a bash script.

### The Challenge with Shebangs in Nix

- In typical Unix-like systems, you might start a bash script with a shebang
  (`#!/bin/bash` or `#!/usr/bin/env bash`) to tell the system how to execute it.
  However, in Nix derivations, we generally avoid this.

- **Reason:** Nix builds happen in an isolated environment where the exact path
  to common tools like `bash` isn't known beforehand (it resides within the Nix
  store). Hardcoding a path or relying on the system's `PATH` would break Nix's
  stateless property.

### The Importance of Statelessness in Nix

- **Stateful Systems (Traditional):** When you install software traditionally,
  it often modifies the core system environment directly. This can lead to
  dependency conflicts and makes rollbacks difficult.

- **Stateless Systems (Nix):** Nix takes a different approach. When installing a
  package, it creates a unique, immutable directory in the Nix store. This
  means:
  - **No Conflicts:** Different versions of the same package can coexist without
    interfering with each other.

  - **Reliable Rollback:** You can easily switch back to previous versions
    without affecting system-wide files.

  - **Reproducibility:** Builds are more likely to produce the same result
    across different machines if they are "pure" (don't rely on external system
    state).

### The Isolated Nix Build Environment: A Quick Overview

When Nix executes a builder script, it sets up a highly controlled and pristine
environment to ensure **reproducibility** and **isolation**. Here's what
happens:

1.  **Fresh Start:** Nix creates a temporary, empty directory for the build and
    makes it the current working directory.

2.  **Clean Environment:** It completely clears the environment variables from
    your shell.

3.  **Controlled Inputs:** Nix then populates the environment with _only_ the
    variables essential for the build, such as:
    - `$NIX_BUILD_TOP`: The path to the temporary build directory.

    - `$PATH`: Carefully set to include only the explicit `buildInputs` you've
      specified, preventing reliance on arbitrary system tools.

    - `$HOME`: Set to `/homeless-shelter` to prevent programs from reading
      user-specific configuration files.

    - Variables for each declared output (`$out`, etc.), indicating where the
      final results should be placed in the Nix store.

4.  **Execution & Logging:** The builder script is run with its specified
    arguments. All its output (stdout/stderr) is captured in a log.

5.  **Clean Up & Registration:** If successful, the temporary directory is
    removed. Nix then scans the build outputs for references to other store
    paths, ensuring all dependencies are correctly tracked for future use and
    garbage collection. Finally, it normalizes file permissions and timestamps
    in the output for consistent hashing.

This meticulous setup ensures that your builds are independent of the machine
they run on and always produce the same result, given the same inputs.

## Our builder Script

- For our first derivation, we'll create a simple `builder.sh` file in the
  current directory:

```bash
# builder.sh
declare -xp
echo foo > $out
```

- The command `declare -xp` lists exported variables (it's a bash builtin
  function).

- Nix needs to know where the final built product (the "cake" in our earlier
  analogy) should be placed. So, during the derivation process, Nix calculates a
  unique output path within the Nix store. This path is then made available to
  our builder script as an environment variable named `$out`. The `.drv` file,
  which is the recipe, contains instructions for the builder, including setting
  up this `$out` variable. Our builder script will then put the result of its
  work (in this case, the "foo" file) into this specific `$out` directory.

- As mentioned earlier we need to find the nix store path to the bash
  executable, common way to do this is to load Nixpkgs into the repl and check:

```bash
nix-repl> :l <nixpkgs>
Added 3950 variables.
nix-repl> "${bash}"
"/nix/store/ihmkc7z2wqk3bbipfnlh0yjrlfkkgnv6-bash-4.2-p45"
```

So, with this little trick we are able to refer to `bin/bash` and create our
derivation:

```bash
nix-repl> d = derivation { name = "foo"; builder = "${bash}/bin/bash";
 args = [ ./builder.sh ]; system = builtins.currentSystem; }
nix-repl> :b d
[1 built, 0.0 MiB DL]

this derivation produced the following outputs:
  out -> /nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo
```

- The contents of the resulting store path (`/nix/store/...-foo`) now contain
  the file `foo`, as intended. We have successfully built a derivation!

- Derivations are the primitive that Nix uses to define packages. “Package” is a
  loosely defined term, but a derivation is simply the result of calling
  `builtins.derivation`.

</details>

## Our Last Derivation

Create a new directory and a `hello.nix` with the following contents:

```nix
# hello.nix
{
  stdenv,
  fetchzip,
}:

stdenv.mkDerivation {
  pname = "hello";
  version = "2.12.1";

  src = fetchzip {
    url = "https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz";
    sha256 = "";
  };
}
```

Save this file to `hello.nix` and run `nix-build` to observe the build failure:

- Click to expand output:

```nix
$ nix-build hello.nix
~error: cannot evaluate a function that has an argument without a value ('stdenv')
~       Nix attempted to evaluate a function as a top level expression; in
~       this case it must have its arguments supplied either by default
~       values, or passed explicitly with '--arg' or '--argstr'. See
~       https://nix.dev/manual/nix/stable/language/constructs.html#functions.
~
~       at /home/nix-user/hello.nix:3:3:
~
~            2| {
~            3|   stdenv,
~             |   ^
~            4|   fetchzip,
```

**Problem**: The expression in `hello.nix` is a _function_, which only produces
it's intended output if it is passed the correct _arguments_.(i.e. `stdenv` is
available from `nixpkgs` so we need to import `nixpkgs` before we can use
`stdenv`):

The recommended way to do this is to create a `default.nix` file in the same
directory as the `hello.nix` with the following contents:

```nix
# default.nix
let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
{
  hello = pkgs.callPackage ./hello.nix { };
}
```

This allows you to run `nix-build -A hello` to realize the derivation in
`hello.nix`, similar to the current convention used in Nixpkgs:

- Click to expand Output:

```nix
nix-build -A hello
~error: hash mismatch in fixed-output derivation '/nix/store/pd2kiyfa0c06giparlhd1k31bvllypbb-source.drv':
~         specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
~            got:    sha256-1kJjhtlsAkpNB7f6tZEs+dbKd8z7KoNHyDHEJ0tmhnc=
~error: 1 dependencies of derivation '/nix/store/b4mjwlv73nmiqgkdabsdjc4zq9gnma1l-hello-2.12.1.drv' failed to build
```

- Another way to do this is with
  [nix-prefetch-url](https://nix.dev/manual/nix/2.24/command-ref/nix-prefetch-url)
  It is a utility to calculate the sha beforehand.

```bash
nix-prefetch-url https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz
path is '/nix/store/pa10z4ngm0g83kx9mssrqzz30s84vq7k-hello-2.12.1.tar.gz'
086vqwk2wl8zfs47sq2xpjc9k066ilmb8z6dn0q6ymwjzlm196cd
```

- When you use `nix-prefetch-url`, you get a Base32 hash when nix needs SRI
  format.

Run the following command to convert from Base32 to SRI:

```bash
nix hash to-sri --type sha256 086vqwk2wl8zfs47sq2xpjc9k066ilmb8z6dn0q6ymwjzlm196cd
sha256-jZkUKv2SV28wsM18tCqNxoCZmLxdYH2Idh9RLibH2yA=
```

- This actually fetched a different sha than the Nix compiler returned in the
  example where we replace the empty sha with the one Nix gives us. The
  difference was that `fetchzip` automatically extracts archives before
  computing the hash and slight differences in the metadata cause different
  results. I had to switch from `fetchzip` to `fetchurl` to get the correct
  results.
  - Extracted archives can differ in timestamps, permissions, or compression
    details, causing different hash values.

  - A simple takeaway is to use `fetchurl` when you need an exact match, and
    `fetchzip` when working with extracted contents.

  - [fetchurl](https://nixos.org/manual/nixpkgs/stable/#fetchurl)

  - `fetchurl` returns a `fixed-output derivation`(FOD): A derivation where a
    cryptographic hash of the output is determined in advance using the
    outputHash attribute, and where the builder executable has access to the
    network.

Lastly replace the empty sha256 placeholder with the returned value from the
last command:

```nix
# hello.nix
{
  stdenv,
  fetchzip,
}:

stdenv.mkDerivation {
  pname = "hello";
  version = "2.12.1";

  src = fetchzip {
    url = "https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz";
    sha256 = "sha256-1kJjhtlsAkpNB7f6tZEs+dbKd8z7KoNHyDHEJ0tmhnc=";
  };
}
```

Run `nix-build -A hello` again and you'll see the derivation successfully
builds.

## Best Practices

**Reproducible source paths**: If we built the following derivation in
`/home/myuser/myproject` then the store path of `src` will be
`/nix/store/<hash>-myproject` causing the build to no longer be reproducible:

```nix
let pkgs = import <nixpkgs> {}; in

pkgs.stdenv.mkDerivation {
  name = "foo";
  src = ./.;
}
```

> ❗ TIP: Use `builtins.path` with the `name` attribute set to something fixed.
> This will derive the symbolic name of the store path from the `name` instead
> of the working directory:
>
> ```nix
> let pkgs = import <nixpkgs> {}; in
>
> pkgs.stdenv.mkDerivation {
>   name = "foo";
>   src = builtins.path { path = ./.; name = "myproject"; };
> }
> ```

### Conclusion

In this chapter, we've laid the groundwork for understanding Nix derivations,
the fundamental recipes that define how software and other artifacts are built
within the Nix ecosystem. We've explored their key components – inputs, builder,
build phases, and outputs – and how they contribute to Nix's core principles of
reproducibility and isolated environments. Derivations are the workhorses behind
the packages and tools we use daily in Nix.

As you've learned, derivations offer a powerful and principled approach to
software management. However, the way we organize and manage these derivations,
along with other Nix expressions and dependencies, has evolved over time.
Traditionally, Nix projects often relied on patterns involving `default.nix`
files, channel subscriptions, and manual dependency management.

A more recent and increasingly popular approach to structuring Nix projects and
managing dependencies is through Nix Flakes. Flakes introduce a standardized
project structure, explicit input tracking, and a more robust way to ensure
reproducible builds across different environments.

In our next chapter,
[Comparing Flakes and Traditional Nix](https://saylesss88.github.io/Comparing_Flakes_and_Traditional_Nix_8.html),
we will directly compare and contrast these two approaches. We'll examine the
strengths and weaknesses of traditional Nix practices in contrast to the
benefits and features offered by Nix Flakes. This comparison will help you
understand the motivations behind Flakes and when you might choose one approach
over the other for your Nix projects.

As you can see below, there is a ton of information on derivations freely
available.

#### Links To Articles about Derivations

<details>
<summary> Click To Expand Resources </summary>

- [NixPillsOurFirstDerivation](https://nixos.org/guides/nix-pills/06-our-first-derivation)

- [NixPills-WorkingDerivation](https://nixos.org/guides/nix-pills/07-working-derivation)

- [nix.dev-Derivations](https://nix.dev/manual/nix/2.24/language/derivations)

- [nix.dev-packagingExistingSoftware](https://nix.dev/tutorials/packaging-existing-software)

- [howToLearnNix-MyFirstDerivation](https://ianthehenry.com/posts/how-to-learn-nix/my-first-derivation/)

- [howToLearnNix-DerivationsInDetail](https://ianthehenry.com/posts/how-to-learn-nix/derivations-in-detail/)

- [Sparky/blog-creatingASuperSimpleDerivation](https://www.sam.today/blog/creating-a-super-simple-derivation-learning-nix-pt-3) #
  How to learn Nix

- [Sparky/blog-Derivations102](https://www.sam.today/blog/derivations-102-learning-nix-pt-4)

- [ScriveNixWorkshop-nixDerivationBasics](https://scrive.github.io/nix-workshop/04-derivations/01-derivation-basics.html)

- [zeroToNix-Derivations](https://zero-to-nix.com/concepts/derivations/)

- [Tweag-derivationOutputs](https://www.tweag.io/blog/2021-02-17-derivation-outputs-and-output-paths/)

- [theNixLectures-Derivations](https://ayats.org/blog/nix-tuto-2)

- [bmcgee-whatAreFixed-OutputDerivations](https://bmcgee.ie/posts/2023/02/nix-what-are-fixed-output-derivations-and-why-use-them/)

</details>
