---
title: Local Nixpkgs
date: 2025-11-22
author: saylesss88
---

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

# Creating and Building a Local Package within a Nixpkgs Clone

This chapter demonstrates the fundamental pattern for creating a package. Every
package recipe is a file that declares a function. This function takes the
packages dependencies as argument.

In this example we'll make a simple package with `coreutils` and build it.
Demonstrating the process of building and testing a local package.

This chapter will assume you have already have a cloned fork of Nixpkgs. I
choose to clone mine to the `~/src/` directory.

You can check out the `nixpkgs/pkgs/README.md`
[Here](https://github.com/NixOS/nixpkgs/tree/master/pkgs)

The Nixpkgs Contributing Guide can be found
[Here](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md)

## Create your Package directory and a `default.nix`

For this example, we'll create a package called `testPackage` and will place it
in the `nixpkgs/pkgs/misc` directory.

```bash
cd ~/src/nixpkgs/pkgs/misc
mkdir testPackage && cd testPackage
hx default.nix
```

```nix
# default.nix
{
  runCommand,
  coreutils,
}:
runCommand "testPackage" {
  nativeBuildInputs = [
    coreutils
  ];
} ''

  echo 'This is a Test' > $out
''
```

Now we need to add our `testPackage` to `all-packages.nix`

```bash
cd pkgs/top-level
hx all-packages.nix
```

`all-packages.nix` is a centralized module that defines all available package
expressions.

We'll add our package in the list alphabetically:

```nix
# all-packages.nix
# `/msc` # editor search inside file
# Scroll down to t's
# snip ...
termusic = callPackage ../applications/autio/termusic { };

# we add our package here
testPackage = callPackage ../misc/testPackage { };

tfk8s = callPackage ../applications/misc/tfk8s { };
# snip ...
```

> `callPackage` is a core utility in Nixpkgs. It takes a Nix expression (like
> our `default.nix` file, which defines a function) and automatically provides
> the function with any arguments it declares, by looking them up within the
> `pkgs` set (or the scope where `callPackage` is invoked). This means you only
> need to list the dependencies your package needs in its `default.nix` function
> signature, and `callPackage` will "inject" the correct versions of those
> packages. This is what the `callPackage` Nix Pill demonstrates at a lower
> level.

## Understanding `pkgs/by-name/` and other locations

Nixpkgs uses different conventions for package placement:

- **Older categories (e.g., `pkgs/misc/`, `pkgs/applications/`):** Packages
  within these directories typically use `default.nix` as their definition file
  (e.g., `pkgs/misc/testPackage/default.nix`). **These packages are NOT
  automatically included** in the top-level `pkgs` set; they _must_ be This
  chapter will assume you have already have a cloned fork of Nixpkgs. explicitly
  added via a `callPackage` entry in `pkgs/top-level/all-packages.nix`. This is
  the method demonstrated in this chapter for our `testPackage`.

- **The new `pkgs/by-name/` convention:** This is the _preferred location for
  new packages_.
  - Packages here are placed in a directory structure like
    `pkgs/by-name/<first-two-letters>/<package-name>/`.

  - Crucially, their main definition file is named `package.nix` (e.g.,
    `pkgs/by-name/te/testPackage/package.nix`).

  - **Packages placed within `pkgs/by-name/` are automatically discovered and
    exposed** by Nixpkgs' top-level `pkgs` set. They **do not** require a manual
    `callPackage` entry in `all-packages.nix`. This results in a more modular
    and scalable approach, reducing manual maintenance.

> ❗ : While this example uses `pkgs/misc/` to demonstrate explicit
> `callPackage` usage, when contributing a _new_ package to Nixpkgs, you should
> nearly always place it within `pkgs/by-name/` and name its definition file
> `package.nix`.

- [pkgs/by-name/README](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/README.md)

- There are some
  [Limitations](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/README.md#limitations)
  to this approach.

- [nixpkgs-vet](https://github.com/NixOS/nixpkgs-vet)

Previously, packages were manually added to `all-packages.nix`. While this is no
longer needed in most cases, understanding the old method provides useful
context for troubleshooting legacy configurations or custom integrations.

## Try Building the Package

Move to the root directory of Nixpkgs:

```bash
cd ~/src/nixpkgs
```

Try building it:

```bash
nix-build -A testPackage
this derivation will be built:
this derivation will be built:
  /nix/store/yrbjsxmgzkl24n75sqjfxbpv5cs3b9hc-testPackage.drv
building '/nix/store/yrbjsxmgzkl24n75sqjfxbpv5cs3b9hc-testPackage.drv'...
/nix/store/3012zlv30vn6ifihr1jxbg5z3ysw0hl3-testPackage
```

`runCommand` is a simple builder, it takes 3 arguments. The first is the package
name the second is the derivation attributes, and the third is the script to
run.

```bash
cat ~/src/nixpkgs/result
───────┬──────────────────────────────
       │ File: result
───────┼──────────────────────────────
   1   │ This is a Test
───────┴──────────────────────────────
```

```bash
nix-instantiate --eval -A testPackage.meta.position
"/home/jr/src/nixpkgs/pkgs/misc/testPackage/default.nix:6"
```

Tools like `nix search` and the Nixpkgs website use the `meta` information for
documentation and discoverability. It can also be useful for debugging and helps
to provide better error messages. The above command shows that the
`meta.position` attribute points to the file and line where the package
definition begins, which is very useful for debugging.

Typically a file will have a `meta` attribute that looks similar to the
following:

```nix
meta = with lib; {
    homepage = "https://www.openssl.org/";
    description = "A cryptographic library that implements the SSL and TLS protocols";
    license = licenses.openssl;
    platforms = platforms.all;
} // extraMeta;
```

For example, the following shows how Nix is able to discover different parts of
your configuration:

Launch the `nix repl` and load your local flake:

```bash
cd /src
nix repl
nix-repl> :lf nixpkgs
nix-repl> outputs.legacyPackages.x86_64-linux.openssl.meta.position
"/nix/store/syvnmj3hhckkbncm94kfkbl76qsdqqj3-source/pkgs/development/libraries/openssl/default.nix:303"
nix-repl> builtins.unsafeGetAttrPos "description" outputs.legacyPackages.x86_64-linux.openssl.meta
{
  column = 9;
  file = "/nix/store/syvnmj3hhckkbncm94kfkbl76qsdqqj3-source/pkgs/development/libraries/openssl/default.nix";
  line = 303;
}
```

Lets create just the `meta.description` for demonstration purposes.

## Adding the meta attribute

Since we don't have a `meta` attribute this points to a default value that's
incorrect.

Let's add the `meta` attribute and try it again:

```nix
# default.nix
{
  runCommand,
  coreutils,
}:
runCommand "testPackage" {
  nativeBuildInputs = [
    coreutils
  ];

  meta = {
    description = "test package";
};
} ''

  echo 'This is a Test' > $out
''
```

```nix
nix-instantiate --eval -A testPackage.meta.position
"/home/jr/src/nixpkgs/pkgs/misc/testPackage/default.nix:11"
```

Now it points us to the 11'th line, right where our `meta.description` is.

Let's stage our package so nix recognises it:

```bash
cd ~/nixpkgs
git add pkgs/misc/testPackage/
nix edit .#testPackage
```

I used `nix edit` here to ensure it was picked up properly.

The `default.nix` that we've been working on should open in your `$EDITOR`
