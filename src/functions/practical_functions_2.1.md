---
title: Practical Nix Functions
date: 2025-11-22
author: saylesss88
collection: blog
tags: ["nixos", "functions"]
draft: false
---

# Practical Nix Functions

<details>
<summary>
✔️
If you want to follow along with this example you'll have to place the following
in your project directory. Section is collapsed to focus on functions:

</summary>

![coding6](images/coding6.png)

1. [graphviz](https://gitlab.com/api/v4/projects/4207231/packages/generic/graphviz-releases/2.49.3/graphviz-2.49.3.tar.gz)

2. [hello](https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz)

3. `autotools.nix`:

```nix
# autotools.nix
pkgs: attrs:
with pkgs; let
  defaultAttrs = {
    builder = "${bash}/bin/bash";
    args = [./builder.sh];
    setup = ./setup.sh;
    baseInputs = [gnutar gzip gnumake gcc binutils-unwrapped coreutils gawk gnused gnugrep patchelf findutils];
    buildInputs = [];
    system = builtins.currentSystem;
  };
in
  derivation (defaultAttrs // attrs)
```

4. `setup.sh`:

```bash
# setup.sh (This is a library of functions setting up the environment, not directly executable)
unset PATH
for p in $baseInputs $buildInputs; do
  if [ -d $p/bin ]; then
    export PATH="$p/bin${PATH:+:}$PATH"
  fi
  if [ -d $p/lib/pkgconfig ]; then
    export PKG_CONFIG_PATH="$p/lib/pkgconfig${PKG_CONFIG_PATH:+:}$PKG_CONFIG_PATH"
  fi
done

function unpackPhase() {
  tar -xzf $src

  for d in *; do
    if [ -d "$d" ]; then
      cd "$d"
      break
    fi
  done
}

function configurePhase() {
  ./configure --prefix=$out
}

function buildPhase() {
  make
}

function installPhase() {
  make install
}

function fixupPhase() {
  find $out -type f -exec patchelf --shrink-rpath '{}' \; -exec strip '{}' \; 2>/dev/null
}

function genericBuild() {
  unpackPhase
  configurePhase
  buildPhase
  installPhase
  fixupPhase
}
```

5. And finally `builder.sh`:

```bash
# builder.sh (This is the actual builder script specified in the derivation and
# what `nix-build` expects)
set -e
source $setup
genericBuild
```

</details>

This is another example from the Nix-Pill series shown in another way to show
some powerful aspects of functions.

If you have a `default.nix` like this:

```nix
# default.nix
{
  hello = import ./hello.nix;
  graphviz = import ./graphviz.nix;
}
```

It expects the files that it imports to look like this:

```nix
# graphviz.nix
let
  pkgs = import <nixpkgs> { };
  mkDerivation = import ./autotools.nix pkgs;
in
mkDerivation {
  name = "graphviz";
  src = ./graphviz-2.49.3.tar.gz;
}
```

And `hello.nix`:

```nix
# hello.nix
let
  pkgs = import <nixpkgs> { };
  mkDerivation = import ./autotools.nix pkgs;
in
mkDerivation {
  name = "hello";
  src = ./hello-2.12.1.tar.gz;
}
```

You would build these with:

```bash
nix-build -A hello
nix-build -A graphviz
```

As you can see both derivations are dependendent on `nixpkgs` which they
**both** import directly. To centralize our dependencies and avoid redundant
imports, we'll refactor our individual package definitions (`hello.nix`,
`graphviz.nix`) into functions. Our `default.nix` will then be responsible for
setting up the common inputs (like `pkgs` and `mkDerivation`) and passing them
as arguments when it imports and calls these package functions.

Here is what our `default.nix` will look like:

```nix
let
  pkgs = import <nixpkgs> { };
  mkDerivation = import ./autotools.nix pkgs;
in
with pkgs;
{
  hello = import ./hello.nix { inherit mkDerivation; };
  graphviz = import ./graphviz.nix {
    inherit
      mkDerivation
      lib
      gd
      pkg-config
      ;
  };
  graphvizCore = import ./graphviz.nix {
    inherit
      mkDerivation
      lib
      gd
      pkg-config
      ;
    gdSupport = false;
  };
}
```

We define some local variables in the `let` expression and pass them around.

The whole expression in the above `default.nix` returns an attribute set with
the keys `hello`, `graphviz`, and `graphvizCore`

We import `hello.nix` and `graphviz.nix`, which both return a function. We call
the functions, passing them a set of inputs with the `inherit` construct.

Let's change `hello.nix` into a function to match what the `default.nix` now
expects.

```nix
# hello.nix
{mkDerivation}:
mkDerivation {
  name = "hello";
  src = ./hello-2.12.1.tar.gz;
}
```

Now our `graphviz` attribute expects `graphviz.nix` to be a function that takes
the arguments listed in the above `default.nix`, here's what `graphviz.nix` will
look like as a function:

```nix
# graphviz.nix
{
  mkDerivation,
  lib,
  gdSupport ? true,
  gd,
  pkg-config,
}:
mkDerivation {
  name = "graphviz";
  src = ./graphviz-2.49.3.tar.gz;
  buildInputs =
    if gdSupport
    then [
      pkg-config
      (lib.getLib gd)
      (lib.getDev gd)
    ]
    else [];
}
```

We factorized the import of `nixpkgs` and `mkDerivation`, and also added a
variant of `graphviz` with gd support disabled. The result is that both
`hello.nix` and `graphviz.nix` are independent of the repository and
customizable by passing specific inputs.

Now, we can build the package with `gd` support disabled with the `graphvizCore`
attribute:

```bash
nix-build -A graphvizCore
# or we can still build the package that now defaults to gd support
nix-build -A graphviz
```

This example showed us how to turn expressions into functions. We saw how
functions are passed around and shared between Nix expressions and derivations.
