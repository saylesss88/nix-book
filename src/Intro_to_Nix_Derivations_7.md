# Chapter 7

<!--toc:start-->

- [Chapter 7](#chapter-7)
  - [Introduction to Nix Derivations](#introduction-to-nix-derivations)
  - [Creating Derivations in Nix](#creating-derivations-in-nix)
  - [Produce a development shell from a derivation](#produce-a-development-shell-from-a-derivation)
  - [Our Second Derivation: Understanding the Builder](#our-second-derivation-understanding-the-builder)
    - [Why a Builder Script?](#why-a-builder-script)
    - [The Challenge with Shebangs in Nix](#the-challenge-with-shebangs-in-nix)
    - [The Importance of Statelessness in Nix](#the-importance-of-statelessness-in-nix)
  - [Our builder Script](#our-builder-script)
  - [Our Last Derivation](#our-last-derivation)
  - [Best Practices](#best-practices)
  - [Conclusion](#conclusion)
  - [Links To Articles about Derivations](#links-to-articles-about-derivations)
  <!--toc:end-->

## Introduction to Nix Derivations

![gruv10](images/gruv10.png)

- A derivation in Nix is a fundamental concept that describes how to build
  a piece of software or a resource (e.g., a package, library, or configuration
  file). Think of it as a recipe for creating something within the Nix ecosystem.

  - Nix building instructions are called “derivations” and are written in the
    Nix programming language. Derivations can be written for packages or even
    entire systems. After that, they can then be deterministically “realised”
    (built) via Nix, the package manager. Derivations can only depend on a
    pre-defined set of inputs, so they are somewhat reproducible. -- Practical Nix Flakes

  - Most things in NixOS are build around derivations:

    - Programs/Applications: Are derivations

    - Config Files: Are a derivation that takes the nix configuration and produces
      an appropriate config file for the application.

    - The system configuration (i.e. `/run/current-system`) is a derivation

> ```nix
>  ls -lsah /run/current-system
>  0 lrwxrwxrwx 1 root root 85 May 23 12:11 /run/current-system -> /nix/store/cy2c0kxpjrl7ajlg9v3zh898mhj4dyjv-nixos-system-magic-25.11.20250520.2795c50
> ```

- The `->` indicates a symlink and it's pointing to a **store path** which is
  the result of a derivation being built (the system closure)

- For beginners, the analogy of a cooking recipe is helpful:

  - **Ingredients (Dependencies):** What other software or libraries are needed.
  - **Steps (Build Instructions):** The commands to compile, configure, and install.
  - **Final Dish (Output):** The resulting package or resource.

- A Nix derivation encapsulates all this information, telling Nix what inputs
  to use, how to build it, and what the final output should be.

- Nix derivations run in **pure**, **isolated environments**, meaning they
  **cannot** access the internet during the build phase. This ensures that
  builds are reproducible -- they don't depend on external sources that might
  change over time.

  - There are `Fixed-output-derivations` that allow fetching resources during
    the build process by explicitly specifying the expected hash upfront. Just
    keep this in mind that normal derivations don't have network access.

## Creating Derivations in Nix

- The primary way to define packages in Nix is through the `mkDerivation` function,
  which is part of the standard environment (`stdenv`). While a
  lower-level `derivation` function exists for advanced use cases,
  `mkDerivation` simplifies the process by automatically managing dependencies
  and the build environment.

- `mkDerivation` (and `derivation`) takes a set of attributes as its argument.
  At a minimum, you'll often encounter these essential attributes:

  1.  **name:** A human-readable identifier for the derivation
      (e.g., "foo", "hello.txt"). This helps you and Nix refer to the package.
  2.  **system:** Specifies the target architecture for the build
      (e.g., `builtins.currentSystem` for your current machine).
  3.  **builder:** Defines the program that will execute the build instructions
      (e.g., `bash`).

> Our First fake derivation
>
> ```nix
> nix-repl> :l <nixpkgs> # Makes Nixpkgs available for ${pkgs.bash}
> nix-repl> d = derivation { name = "myname"; builder = "${pkgs.bash}/bin/bash"; system = "mysystem"; }
> nix-repl> :b d
> [...]
> these derivations will be built:
> error: a 'mysystem' with features {} is required to build '/nix/store/fq6843vfzzbhy3s6iwcd0hm10l578883-myname.drv',
> but I am a 'x86_64-linux' with features {benchmark, big-parallel, kvm, nixos-test}
> ```
>
> - The build failure is expected here due to the inaccurate attributes
> - The `:b` is a `nix repl` specific command to build a derivation.
> - To realise this outside of the `nix repl` you can use `nix-store -r`:
>
> ```nix
>  $ nix-store -r /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv
> ```
>
> - `nix derivation show`: Pretty print the contents of a store derivation:
>
> ```nix
>  $ nix derivation show /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv
> ```
>
> -- [Nix Pills](https://nixos.org/guides/nix-pills/06-our-first-derivation.html)

- The above example shows the fundamental structure of a Nix derivation, how it's
  defined within the `nix-repl`, and the importance of correctly specifying attributes
  like `system`.

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
  packages. `stdenv` provides a set of common build tools and conventions.
  `mkDerivation` takes an attribute set (a collection of key-value pairs) as its argument.
- `name = "my-environment";`: This gives your derivation a human-readable name.
- `buildInputs = [ pkgs.cowsay ];`: This is a list of dependencies that will
  be available in the build environment of this derivation (or in the `PATH` if
  you enter the shell created by this derivation). `pkgs.cowsay` refers to the
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

- **Stateless Systems (Nix):** Nix takes a different approach. When installing
  a package, it creates a unique, immutable directory in the Nix store. This
  means:
  - **No Conflicts:** Different versions of the same package can coexist
    without interfering with each other.
  - **Reliable Rollback:** You can easily switch back to previous versions
    without affecting system-wide files.
  - **Reproducibility:** Builds are more likely to produce the same result
    across different machines if they are "pure" (don't rely on external
    system state).

## Our builder Script

- For our first derivation, we'll create a simple `builder.sh` file in the current directory:

```bash
# builder.sh
declare -xp
echo foo > $out
```

- The command `declare -xp` lists exported variables (it's a bash builtin
  function).

- Nix needs to know where the final built product (the "cake" in our earlier
  analogy) should be placed. So, during the derivation process, Nix calculates
  a unique output path within the Nix store. This path is then made available
  to our builder script as an environment variable named `$out`. The `.drv`
  file, which is the recipe, contains instructions for the builder, including
  setting up this `$out` variable. Our builder script will then put the result
  of its work (in this case, the "foo" file) into this specific `$out` directory.

- As mentioned earlier we need to find the nix store path to the bash
  executable, common way to do this is to load Nixpkgs into the repl
  and check:

```bash
nix-repl> :l <nixpkgs>
Added 3950 variables.
nix-repl> "${bash}"
"/nix/store/ihmkc7z2wqk3bbipfnlh0yjrlfkkgnv6-bash-4.2-p45"
```

So, with this little trick we are able to refer to `bin/bash` and create
our derivation:

```bash
nix-repl> d = derivation { name = "foo"; builder = "${bash}/bin/bash";
 args = [ ./builder.sh ]; system = builtins.currentSystem; }
nix-repl> :b d
[1 built, 0.0 MiB DL]

this derivation produced the following outputs:
  out -> /nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo
```

- The contents of the resulting store path (`/nix/store/...-foo`) now contain the
  file `foo`, as intended. We have successfully built a derivation!

- Derivations are the primitive that Nix uses to define packages. “Package”
  is a loosely defined term, but a derivation is simply the result of calling
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

This allows you to run `nix-build -A hello` to realize the derivation in `hello.nix`,
similar to the current convention used in Nixpkgs:

- Click to expand Output:

```nix
nix-build -A hello
~error: hash mismatch in fixed-output derivation '/nix/store/pd2kiyfa0c06giparlhd1k31bvllypbb-source.drv':
~         specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
~            got:    sha256-1kJjhtlsAkpNB7f6tZEs+dbKd8z7KoNHyDHEJ0tmhnc=
~error: 1 dependencies of derivation '/nix/store/b4mjwlv73nmiqgkdabsdjc4zq9gnma1l-hello-2.12.1.drv' failed to build
```

- Another way to do this is with [nix-prefetch-url](https://nix.dev/manual/nix/2.24/command-ref/nix-prefetch-url)
  It is a utility to calculate the sha beforehand.

```bash
nix-prefetch-url https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz
path is '/nix/store/pa10z4ngm0g83kx9mssrqzz30s84vq7k-hello-2.12.1.tar.gz'
086vqwk2wl8zfs47sq2xpjc9k066ilmb8z6dn0q6ymwjzlm196cd
```

- When you use `nix-prefetch-url`, you get a Base32 hash when nix needs SRI format.

Run the following command to convert from Base32 to SRI:

```bash
nix hash to-sri --type sha256 086vqwk2wl8zfs47sq2xpjc9k066ilmb8z6dn0q6ymwjzlm196cd
sha256-jZkUKv2SV28wsM18tCqNxoCZmLxdYH2Idh9RLibH2yA=
```

- This actually fetched a different sha than the Nix compiler returned in the
  example where we replace the empty sha with the one Nix gives us. The difference
  was that `fetchzip` automatically extracts archives before computing the hash
  and slight differences in the metadata cause different results. I had to switch
  from `fetchzip` to `fetchurl` to get the correct results.

  - Extracted archives can differ in timestamps, permissions, or compression
    details, causing different hash values.

  - A simple takeaway is to use `fetchurl` when you need an exact match, and
    `fetchzip` when working with extracted contents.

  - [fetchurl](https://nixos.org/manual/nixpkgs/stable/#fetchurl)

  - `fetchurl` returns a `fixed-output derivation`(FOD): A derivation where a
    cryptographic hash of the output is determined in advance using the outputHash
    attribute, and where the builder executable has access to the network.

Lastly replace the empty sha256 placeholder with the returned value from the last
command:

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

Run `nix-build -A hello` again and you'll see the derivation successfully builds.

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

> ❗ TIP:
> Use `builtins.path` with the `name` attribute set to something fixed.
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

## Conclusion

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

In our next chapter, [Comparing Flakes and Traditional Nix](https://saylesss88.github.io/Comparing_Flakes_and_Traditional_Nix_8.html),
we will directly compare and contrast these two approaches. We'll examine the strengths and
weaknesses of traditional Nix practices in contrast to the benefits and features
offered by Nix Flakes. This comparison will help you understand the motivations
behind Flakes and when you might choose one approach over the other for your Nix
projects.

As you can see below, there is a ton of information on derivations freely available.

## Links To Articles about Derivations

<details>
<summary> Click To Expand Resources </summary>

- [NixPillsOurFirstDerivation](https://nixos.org/guides/nix-pills/06-our-first-derivation)

- [NixPills-WorkingDerivation](https://nixos.org/guides/nix-pills/07-working-derivation)

- [nix.dev-Derivations](https://nix.dev/manual/nix/2.24/language/derivations)

- [nix.dev-packagingExistingSoftware](https://nix.dev/tutorials/packaging-existing-software)

- [howToLearnNix-MyFirstDerivation](https://ianthehenry.com/posts/how-to-learn-nix/my-first-derivation/)

- [howToLearnNix-DerivationsInDetail](https://ianthehenry.com/posts/how-to-learn-nix/derivations-in-detail/)

- [Sparky/blog-creatingASuperSimpleDerivation](https://www.sam.today/blog/creating-a-super-simple-derivation-learning-nix-pt-3) # How to learn Nix

- [Sparky/blog-Derivations102](https://www.sam.today/blog/derivations-102-learning-nix-pt-4)

- [ScriveNixWorkshop-nixDerivationBasics](https://scrive.github.io/nix-workshop/04-derivations/01-derivation-basics.html)

- [zeroToNix-Derivations](https://zero-to-nix.com/concepts/derivations/)

- [Tweag-derivationOutputs](https://www.tweag.io/blog/2021-02-17-derivation-outputs-and-output-paths/)

- [theNixLectures-Derivations](https://ayats.org/blog/nix-tuto-2)

- [bmcgee-whatAreFixed-OutputDerivations](https://bmcgee.ie/posts/2023/02/nix-what-are-fixed-output-derivations-and-why-use-them/)

</details>
