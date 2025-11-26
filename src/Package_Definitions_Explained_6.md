---
title: Package Definitions Explained
date: 2025-11-21
author: saylesss88
---

# Chapter 8

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![coding2](images/coding2.png)

<!-- ![gruv1](images/gruv1.png) -->

## Package Definitions Explained

In Nix, the concept of a **package** can refer to two things:

- A collection of files and data that constitute a piece of software or an
  artifact.

- A Nix **expression** that describes how to create such a collection. This
  expression acts as a blueprint before the package exists in a tangible form.

The process begins with writing a **package definition** using the Nix language.
This definition contains the necessary instructions and metadata about the
software you intend to "package."

## The Journey from Definition to Package

<details>
<summary> ✔️ Click to Expand</summary>

1.  **Package Definition:**
    - This is essentially a function written in the Nix language.

    - Nix language shares similarities with JSON but includes the crucial
      addition of functions.

    - It acts as the blueprint for creating a package.

2.  **Derivation:**
    - When the package definition is evaluated by Nix, it results in a
      **derivation**.

    - A derivation is a concrete and detailed build plan.

    - It outlines the exact steps Nix needs to take: fetching source code,
      building dependencies, compiling code, and ultimately producing the
      desired output (the package).

3.  **Realization (Building the Package):**
    - You don't get a pre-built "package" directly from the definition or the
      derivation.

    - The package comes into being when Nix **executes** the derivation. This
      process is often referred to as "realizing" the derivation.

**Analogy:** Think of a package definition as an architectural blueprint, the
derivation as the detailed construction plan, and the realized package as the
finished building.

</details>
## Skeleton of a Derivation

The most basic derivation structure in Nix looks like this:

```nix
{ stdenv }:

stdenv.mkDerivation { }
```

- This is a function that expects an attribute set containing `stdenv` as its
  argument.

- It then calls `stdenv.mkDerivation` (a function provided by `stdenv`) to
  produce a derivation.

- Currently, this derivation doesn't specify any build steps or outputs.

- Further Reading:

- [The Standard Environment](https://ryantm.github.io/nixpkgs/stdenv/stdenv/)

- [Fundamentals of Stdenv](https://nixos.org/guides/nix-pills/19-fundamentals-of-stdenv.html)

## Example: A Simple "Hello" Package Definition

Here's a package definition for the classic "hello" program:

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
    url = "[https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz](https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz)";
    sha256 = "";
  };
}
```

- This is a Nix function that takes stdenv and fetchzip as arguments.

- It uses `stdenv.mkDerivation` to define the build process for the "hello"
  package.
  - `pname`: The package name.

  - `version`: The package version.

  - `src`: Specifies how to fetch the source code using `fetchzip`.

**Handling Dependencies: Importing Nixpkgs**

- If you try to build `hello.nix` directly with `nix-build hello.nix`, it will
  fail because `stdenv` and `fetchzip` are part of Nixpkgs, which isn't included
  in this isolated file.

- To make this package definition work, you need to pass the correct arguments
  (`stdenv`, `fetchzip`) to the function.

The recommended approach is to create a `default.nix` file in the same
directory:

```nix
# default.nix

let
  nixpkgs = fetchTarball "[https://github.com/NixOS/nixpkgs/tarball/nixos-24.05](https://github.com/NixOS/nixpkgs/tarball/nixos-24.05)";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
{
  hello = pkgs.callPackage ./hello.nix { };
}
```

- This `default.nix` imports Nixpkgs.

- It then uses `pkgs.callPackage` to call the function in `hello.nix`, passing
  the necessary dependencies from Nixpkgs.

- You can now build the "hello" package using: `nix-build -A hello`. The `-A`
  flag tells Nix to build the attribute named hello from the top-level
  expression in default.nix.

**Realizing the Derivation and Handling sha256**

- **Evaluation vs. Realization**: While "evaluate" refers to Nix processing an
  expression, "realize" often specifically means building a derivation and
  producing its output in the Nix store.

- When you first run `nix-build -A hello`, it will likely fail due to a missing
  sha256 hash for the source file. Nix needs this hash for security and
  reproducibility. The error message will provide the correct sha256 value.

- **Example Error**:

```bash
  nix-build -A hello
  error: hash mismatch in fixed-output derivation '/nix/store/pd2kiyfa0c06giparlhd1k31bvllypbb-source.drv':
  specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
  got: sha256-1kJjhtlsAkpNB7f6tZEs+dbKd8z7KoNHyDHEJ0tmhnc=
  error: 1 dependencies of derivation '/nix/store/b4mjwlv73nmiqgkdabsdjc4zq9gnma1l-hello-2.12.1.drv' failed to build
```

- Replace the empty `sha256 = "";` in `hello.nix` with the provided correct
  value: `sha256 = "1kJjhtlsAkpNB7f6tZEs+dbKd8z7KoNHyDHEJ0tmhnc=";`.

**Building and Running the Result**

After updating the `sha256`, you can successfully build the package:

```bash
nix-build -A hello
```

The output will be a result symlink pointing to the built package in the Nix
store. You can then run the "hello" program:

```bash
./result/bin/hello
Hello, world!
```

### Swaytools Package Definition

**Example: The swaytools Package Definition**

Let's examine a more complex, real-world package definition from Nixpkgs:
`nixpkgs/pkgs/tools/wayland/swaytools/default.nix`.

```nix
# default.nix
{
  lib,
  setuptools,
  buildPythonApplication,
  fetchFromGitHub,
  slurp,
}:

buildPythonApplication rec {
  pname = "swaytools";
  version = "0.1.2";

  format = "pyproject";

  src = fetchFromGitHub {
    owner = "tmccombs";
    repo = "swaytools";
    rev = version;
    sha256 = "sha256-UoWK53B1DNmKwNLFwJW1ZEm9dwMOvQeO03+RoMl6M0Q=";
  };

  nativeBuildInputs = [ setuptools ];

  propagatedBuildInputs = [ slurp ];

  meta = with lib; {
    homepage = "https://github.com/tmccombs/swaytools";
    description = "Collection of simple tools for sway (and i3)";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [ atila ];
    platforms = platforms.linux;
  };
}
```

### Breakdown of the Above default.nix

<details>
<summary>Click to Expand</summary>

1 **Function Structure**:

- The file starts with a function taking an attribute set of dependencies from
  Nixpkgs: `{ lib, setuptools, buildPythonApplication, fetchFromGitHub, slurp }`
  :.

2. **Derivation Creation**:

- It calls `buildPythonApplication`, a specialized helper for Python packages
  (similar to `stdenv.mkDerivation` but pre-configured for Python). The `rec`
  keyword allows attributes within the derivation to refer to each other.

3. **Package Metadata**:

- `pname` and `version` define the package's name and version.

- The `meta` attribute provides standard package information like the homepage,
  description, license, maintainers, and supported platforms.

4. **Source Specification**:

- The `src` attribute uses `fetchFromGitHub` to download the source code from
  the specified repository and revision, along with its `sha256` hash for
  verification.

5. **Build and Runtime Dependencies**:

- `nativeBuildInputs`: Lists tools required during the build process (e.g.,
  `setuptools` for Python).

- `propagatedBuildInputs`: Lists dependencies needed at runtime (e.g., `slurp`).

6. **Build Format**:

- `format = "pyproject";` indicates that the package uses a `pyproject.toml`
  file for its Python build configuration.

**Integration within Nixpkgs**

- **Location**: The swaytools definition resides in
  `pkgs/tools/wayland/swaytools/default.nix`.

- **Top-Level Inclusion**: It's made available as a top-level package in
  `pkgs/top-level/all-packages.nix` like this:

```nix
# all-packages.nix
swaytools = python3Packages.callPackage ../tools/wayland/swaytools { };
```

- `python3Packages.callPackage` is used here because `swaytools` is a Python
package, and it ensures the necessary Python-related dependencies are correctly
passed to the `swaytools` definition.
</details>

## Conclusion

In this chapter, we've journeyed through the fundamental concept of package
definitions in Nix. We've seen how these Nix expressions act as blueprints,
leading to the creation of derivations – the detailed plans for building
software. Finally, we touched upon the realization process where Nix executes
these derivations to produce tangible packages in the Nix store. Examining the
simple "hello" package and the more complex "swaytools" definition provided
practical insights into the structure and key attributes involved in defining
software within the Nix ecosystem.

The crucial step in this process, the transformation from a package definition
to a concrete build plan, is embodied by the **derivation**. This detailed
specification outlines every step Nix needs to take to fetch sources, build
dependencies, compile code, and produce the final package output. Understanding
the anatomy and lifecycle of a derivation is key to unlocking the full power and
flexibility of Nix.

In the **next chapter**,
[Introduction to Nix Derivations](https://saylesss88.github.io/Intro_to_Nix_Derivations_7.html),
we will delve deeper into the structure and components of these derivations. We
will explore the attributes that define a build process, how dependencies are
managed within a derivation, and how Nix ensures the reproducibility and
isolation of your software builds through this fundamental concept.

## Resources

- [Packaging Existing Software](https://nix.dev/tutorials/packaging-existing-software.html)
