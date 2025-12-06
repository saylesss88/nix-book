---
title: Understanding the Helix Flake
date: 2025-12-05
author: saylesss88
collection: "blog"
tags: ["nixos", "flakes", "helix"]
draft: false
---

# Chapter 4.4

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![Helix Logo](../images/helix.png)--[helix-editor.com](https://helix-editor.com/)

# Understanding the Helix Flake and Modifying its Behavior

As we've seen from previous examples, the helix editor repository includes a few
`.nix` files including a `flake.nix`. Their flake uses a lot of idiomatic Nix
code and advanced features. First I will break down their `flake.nix` and
`default.nix` to understand why they do certain things. And finally, we will
change the build to "debug" mode demonstrating how easily you can modify the
behavior of a package defined within a Nix flake without changing the original
source code or the upstream flake directly.

1. Let's clone the Helix repository:

```bash
git clone https://github.com/helix-editor/helix.git
cd helix
```

When you enter the `helix` directory, `direnv` is setup for you already. All you
would have to do is `direnv allow` and it will ask you a few questions then you
are good to go. Looking at their `.envrc` it mentions "try to use flakes, if it
fails use normal nix (i.e., shell.nix)". If it's successful you'll see a long
list of environment variables displayed.

2. Enter the Development Shell:

The Helix project's `flake.nix` includes a `devShells.default` output,
specifically designed for development.

```bash
nix develop
```

3. You're now in a fully configured development environment:

- When you run `nix develop`, Nix builds and drops you into a shell environment
  with all the dependencies specified in `devShells.default`. This means you
  don’t have to manually install or manage tools like Rust, Cargo, or Clang,
  it’s all handled declaratively through Nix.

You can now build and run the project using its standard tooling:

```bash
cargo check
cargo build
cargo run
```

4. Making Changes and Testing Them

Since you're in a reproducible environment, you can confidently hack on the
project without worrying about your system setup. Try modifying some code in
`helix` and rebuilding with Cargo. The Nix shell ensures consistency for every
contributor or device you work on.

5. Run Just the Binary

If you only want to run the compiled program without entering the shell, use the
nix run command:

```bash
nix run
```

This builds and runs the default package defined by the flake. In the case of
Helix, this launches the `hx` editor directly.

6. Build Without Running

To just build the project and get the path to the output binary:

```bash
nix build
```

You’ll find the compiled binary under `./result/bin`.

7. Pinning and Reproducing

Because the project uses a flake, you can ensure full reproducibility by pinning
the inputs. For example, you can clone with `--recurse-submodules` and copy the
`flake.lock` to ensure you're using the same dependency versions as upstream.
This is great for debugging or sharing exact builds.

✅ Recap:

With flakes, projects like Helix provide everything you need for development and
running in a single `flake.nix`. You can nix develop to get started hacking, nix
run to quickly try it out, and nix build to produce binaries all without
installing or polluting your system.

## Understanding the Helix flake.nix

The helix flake is full of idiomatic Nix code and displays some of the more
advanced things a flake can provide:

```nix
{
  description = "A post-modern text editor.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    inherit (nixpkgs) lib;
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    eachSystem = lib.genAttrs systems;
    pkgsFor = eachSystem (system:
      import nixpkgs {
        localSystem.system = system;
        overlays = [(import rust-overlay) self.overlays.helix];
      });
    gitRev = self.rev or self.dirtyRev or null;
  in {
    packages = eachSystem (system: {
      inherit (pkgsFor.${system}) helix;
      /*
      The default Helix build. Uses the latest stable Rust toolchain, and unstable
      nixpkgs.

      The build inputs can be overridden with the following:

      packages.${system}.default.override { rustPlatform = newPlatform; };

      Overriding a derivation attribute can be done as well:

      packages.${system}.default.overrideAttrs { buildType = "debug"; };
      */
      default = self.packages.${system}.helix;
    });
    checks =
      lib.mapAttrs (system: pkgs: let
        # Get Helix's MSRV toolchain to build with by default.
        msrvToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        msrvPlatform = pkgs.makeRustPlatform {
          cargo = msrvToolchain;
          rustc = msrvToolchain;
        };
      in {
        helix = self.packages.${system}.helix.override {
          rustPlatform = msrvPlatform;
        };
      })
      pkgsFor;

    # Devshell behavior is preserved.
    devShells =
      lib.mapAttrs (system: pkgs: {
        default = let
          commonRustFlagsEnv = "-C link-arg=-fuse-ld=lld -C target-cpu=native --cfg tokio_unstable";
          platformRustFlagsEnv = lib.optionalString pkgs.stdenv.isLinux "-Clink-arg=-Wl,--no-rosegment";
        in
          pkgs.mkShell {
            inputsFrom = [self.checks.${system}.helix];
            nativeBuildInputs = with pkgs;
              [
                lld
                cargo-flamegraph
                rust-bin.nightly.latest.rust-analyzer
              ]
              ++ (lib.optional (stdenv.isx86_64 && stdenv.isLinux) cargo-tarpaulin)
              ++ (lib.optional stdenv.isLinux lldb)
              ++ (lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.CoreFoundation);
            shellHook = ''
              export RUST_BACKTRACE="1"
              export RUSTFLAGS="''${RUSTFLAGS:-""} ${commonRustFlagsEnv} ${platformRustFlagsEnv}"
            '';
          };
      })
      pkgsFor;

    overlays = {
      helix = final: prev: {
        helix = final.callPackage ./default.nix {inherit gitRev;};
      };

      default = self.overlays.helix;
    };
  };
  nixConfig = {
    extra-substituters = ["https://helix.cachix.org"];
    extra-trusted-public-keys = ["helix.cachix.org-1:ejp9KQpR1FBI2onstMQ34yogDm4OgU2ru6lIwPvuCVs="];
  };
}
```

**Top-Level Metadata**:

```nix
{
  description = "A post-modern text editor.";
}
```

- This sets a human-readable description for the flake.

## Inputs

```nix
inputs = {
  nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};
```

- `nixpkgs`: Uses the `nixos-unstable` branch of the Nixpkgs repository.

- `rust-overlay`: follows the same `nixpkgs`, ensuring compatibility between
  inputs.

**Outputs Function**:

```nix
outputs = { self, nixpkgs, rust-overlay, ... }:
```

- This defines what this flake exports, including `packages`, `devShells`, etc.

**Common Setup**:

```nix
let
  inherit (nixpkgs) lib;
  systems = [ ... ];
  eachSystem = lib.genAttrs systems;
```

- `systems`: A list of the supported systems

- `eachSystem`: A Helper to map over all platforms.

```nix
pkgsFor = eachSystem (system:
  import nixpkgs {
    localSystem.system = system;
    overlays = [(import rust-overlay) self.overlays.helix];
  });
```

- This imports `nixpkgs` for each system and applies overlays

📦 `packages`

```nix
packages = eachSystem (system: {
  inherit (pkgsFor.${system}) helix;
  default = self.packages.${system}.helix;
});
```

- For each platform:
  - Includes a `helix` package (defined in `./default.nix`)

  - Sets `default` to `helix` (used by `nix build`, `nix run`)

Let's look at the helix `default.nix`:

```nix
{
  lib,
  rustPlatform,
  callPackage,
  runCommand,
  installShellFiles,
  git,
  gitRev ? null,
  grammarOverlays ? [],
  includeGrammarIf ? _: true,
}: let
  fs = lib.fileset;

  src = fs.difference (fs.gitTracked ./.) (fs.unions [
    ./.envrc
    ./rustfmt.toml
    ./screenshot.png
    ./book
    ./docs
    ./runtime
    ./flake.lock
    (fs.fileFilter (file: lib.strings.hasInfix ".git" file.name) ./.)
    (fs.fileFilter (file: file.hasExt "svg") ./.)
    (fs.fileFilter (file: file.hasExt "md") ./.)
    (fs.fileFilter (file: file.hasExt "nix") ./.)
  ]);

  # Next we actually need to build the grammars and the runtime directory
  # that they reside in. It is built by calling the derivation in the
  # grammars.nix file, then taking the runtime directory in the git repo
  # and hooking symlinks up to it.
  grammars = callPackage ./grammars.nix {inherit grammarOverlays includeGrammarIf;};
  runtimeDir = runCommand "helix-runtime" {} ''
    mkdir -p $out
    ln -s ${./runtime}/* $out
    rm -r $out/grammars
    ln -s ${grammars} $out/grammars
  '';
in
  rustPlatform.buildRustPackage (self: {
    cargoLock = {
      lockFile = ./Cargo.lock;
      # This is not allowed in nixpkgs but is very convenient here: it allows us to
      # avoid specifying `outputHashes` here for any git dependencies we might take
      # on temporarily.
      allowBuiltinFetchGit = true;
    };

    nativeBuildInputs = [
      installShellFiles
      git
    ];

    buildType = "release";

    name = with builtins; (fromTOML (readFile ./helix-term/Cargo.toml)).package.name;
    src = fs.toSource {
      root = ./.;
      fileset = src;
    };

    # Helix attempts to reach out to the network and get the grammars. Nix doesn't allow this.
    HELIX_DISABLE_AUTO_GRAMMAR_BUILD = "1";

    # So Helix knows what rev it is.
    HELIX_NIX_BUILD_REV = gitRev;

    doCheck = false;
    strictDeps = true;

    # Sets the Helix runtime dir to the grammars
    env.HELIX_DEFAULT_RUNTIME = "${runtimeDir}";

    # Get all the application stuff in the output directory.
    postInstall = ''
      mkdir -p $out/lib
      installShellCompletion ${./contrib/completion}/hx.{bash,fish,zsh}
      mkdir -p $out/share/{applications,icons/hicolor/{256x256,scalable}/apps}
      cp ${./contrib/Helix.desktop} $out/share/applications/Helix.desktop
      cp ${./logo.svg} $out/share/icons/hicolor/scalable/apps/helix.svg
      cp ${./contrib/helix.png} $out/share/icons/hicolor/256x256/apps/helix.png
    '';

    meta.mainProgram = "hx";
  })
```

### Breaking Down `helix/default.nix`

<details>
<summary> ✔️ Click to Expand `helix/default.nix` breakdown </summary>

This `default.nix` file is a Nix derivation that defines how to build the Helix
editor itself. It's designed to be called by the main `flake.nix` as part of its
`packages` output.

Here's a breakdown of its components:

1. **Function Arguments**:

```nix
{
  lib,
  rustPlatform,
  callPackage,
  runCommand,
  installShellFiles,
  git,
  gitRev ? null,
  grammarOverlays ? [],
  includeGrammarIf ? _: true,
}:
```

`lib`: The Nixpkgs `lib` (library) functions, essential for common operations
like `fileset` and `strings`.

`rustPlatform`: A helper function from Nixpkgs specifically for building Rust
projects. It provides a `buildRustPackage` function, which simplifies the
process significantly.

`callPackage`: A Nixpkgs function used to instantiate a Nix expression (like
`grammars.nix`) with its dependencies automatically supplied from the current
Nix environment.

`runCommand`: A Nixpkgs primitive that creates a derivation by running a shell
command. It's used here to construct the `runtimeDir`.

`installShellFiles`: A utility from Nixpkgs for installing shell completion
files.

`git`: The Git package, needed for determining the `gitRev`.

`gitRev ? null`: The Git revision of the Helix repository. It's an optional
argument, defaulting to null. This is passed in from the main `flake.nix`.

`grammarOverlays ? []`: An optional list of overlays for grammars, allowing
customization.

`includeGrammarIf ? _: true`: An optional function to control which grammars are
included.

2. **Local Variables** (`let ... in`)

```nix
let
  fs = lib.fileset;

  src = fs.difference (fs.gitTracked ./.) (fs.unions [
    ./.envrc
    ./rustfmt.toml
    ./screenshot.png
    ./book
    ./docs
    ./runtime
    ./flake.lock
    (fs.fileFilter (file: lib.strings.hasInfix ".git" file.name) ./.)
    (fs.fileFilter (file: file.hasExt "svg") ./.)
    (fs.fileFilter (file: file.hasExt "md") ./.)
    (fs.fileFilter (file: file.hasExt "nix") ./.)
  ]);

  grammars = callPackage ./grammars.nix { inherit grammarOverlays includeGrammarIf; };
  runtimeDir = runCommand "helix-runtime" {} ''
    mkdir -p $out
    ln -s ${./runtime}/* $out
    rm -r $out/grammars
    ln -s ${grammars} $out/grammars
  '';
in
```

`fs = lib.fileset;`: Aliases `lib.fileset` for convenient file set operations.

`src`: This is a crucial part. It defines the source files that will be used to
build Helix by:

- Taking all Git-tracked files in the current directory (`fs.gitTracked ./.`).

- Excluding configuration files (e.g., `.envrc`, `flake.lock`), documentation
  (`.md`), images (`.svg`), and Nix files (`.nix`) using `fs.difference` and
  `fs.unions`. This ensures a clean build input, reducing Nix store size and
  avoiding unnecessary rebuilds.

- `grammars`: Builds syntax grammars by calling `grammars.nix`, passing
  `grammarOverlays` (for customizing grammar builds) and `includeGrammarIf` (a
  filter for selecting grammars).

- `runtimeDir`: Creates a runtime directory for Helix by:
  - Symlinking the `runtime` directory from the source.

  - Replacing the `grammars` subdirectory with a symlink to the `grammars`
    derivation, ensuring Helix uses Nix-managed grammars.

3. **The Build Derivation** (`rustPlatform.buildRustPackage`)

The core of this `default.nix` is the `rustPlatform.buildRustPackage` call,
which is a specialized builder for Rust projects:

```nix
in
  rustPlatform.buildRustPackage (self: {
    cargoLock = {
      lockFile = ./Cargo.lock;
      # ... comments ...
      allowBuiltinFetchGit = true;
    };
```

`cargoLock`: Specifies how Cargo dependencies are handled.

`lockFile = ./Cargo.lock;` Points to the `Cargo.lock` file for reproducible
builds.

`allowBuiltinFetchGit = true`: Allows Cargo to fetch Git dependencies directly
from repositories specified in `Cargo.lock`. This is discouraged in Nixpkgs
because it can break build reproducibility, but it’s used here for convenience
during development, eliminating the need to manually specify `outputHashes` for
Git dependencies.

```nix
nativeBuildInputs = [
      installShellFiles
      git
    ];
```

`nativeBuildInputs`: Are tools needed during the build process but not
necessarily at runtime.

```nix
buildType = "release";
```

`buildType`: Specifies that Helix should be built in "release" mode (optimized).

```nix
name = with builtins; (fromTOML (readFile ./helix-term/Cargo.toml)).package.name;
    src = fs.toSource {
      root = ./.;
      fileset = src;
    };
```

`name`: Dynamically sets the package name by reading it from the `Cargo.toml`
file.

`src`: Uses the `src` file set defined earlier as the source for the build.

```nix
# Helix attempts to reach out to the network and get the grammars. Nix doesn't allow this.
    HELIX_DISABLE_AUTO_GRAMMAR_BUILD = "1";

    # So Helix knows what rev it is.
    HELIX_NIX_BUILD_REV = gitRev;
```

**Environment Variables**: Sets environment variables that Helix uses.

`HELIX_DISABLE_AUTO_GRAMMAR_BUILD = "1"`: Prevents Helix from downloading
grammars during the build, as Nix’s sandboxed environment disallows network
access. Instead, grammars are provided via the `runtimeDir` derivation.

`HELIX_NIX_BUILD_REV = gitRev`: Embeds the specified Git revision (or `null` if
unspecified) into the Helix binary, allowing Helix to display its version or
commit hash.

```nix
doCheck = false;
   strictDeps = true;
```

`doCheck = false;`: Skips running tests during the build. This is common for
faster builds, especially in CI/CD, but tests are often run in a separate
`checks` output (as seen in the `flake.nix`).

`strictDeps = true;`: Ensures that all dependencies are explicitly declared.

```nix
# Sets the Helix runtime dir to the grammars
env.HELIX_DEFAULT_RUNTIME = "${runtimeDir}";
```

```nix
# Sets the Helix runtime dir to the grammars
env.HELIX_DEFAULT_RUNTIME = "${runtimeDir}";
```

`env.HELIX_DEFAULT_RUNTIME`: Tells Helix where to find its runtime files
(including the Nix-managed grammars).

```nix
# Get all the application stuff in the output directory.
postInstall = ''
  mkdir -p $out/lib
  installShellCompletion ${./contrib/completion}/hx.{bash,fish,zsh}
  mkdir -p $out/share/{applications,icons/hicolor/{256x256,scalable}/apps}
  cp ${./contrib/Helix.desktop} $out/share/applications/Helix.desktop
  cp ${./logo.svg} $out/share/icons/hicolor/scalable/apps/helix.svg
  cp ${./contrib/helix.png} $out/share/icons/hicolor/256x256/apps/helix.png
'';
```

`postInstall`: A shell script that runs after the main build is complete. This
is used for installing additional files that are part of the Helix distribution
but not directly built by Cargo.

Installs shell completion files (`hx.bash`, `hx.fish`, `hx.zsh`). This enables
tab completion.

Installs desktop entry files (`Helix.desktop`) and icons (`logo.svg`,
`helix.png`) for desktop integration for GUI environments.

```nix
    meta.mainProgram = "hx";

})
```

`meta.mainProgram`: Specifies the primary executable provided by this package,
allowing `nix run` to automatically execute `hx`.

A lot going on in this derivation!

</details>

### Making Actual Changes

1. Locate the `packages` output section. It looks like this:

```nix
packages = eachSystem (system: {
      inherit (pkgsFor.${system}) helix;
      /*
      The default Helix build. Uses the latest stable Rust toolchain, and unstable
      nixpkgs.

      The build inputs can be overridden with the following:

      packages.${system}.default.override { rustPlatform = newPlatform; };

      Overriding a derivation attribute can be done as well:

      packages.${system}.default.overrideAttrs { buildType = "debug"; };
      */
      default = self.packages.${system}.helix;
    });
```

2. Modify the `default` package. The comments actually tell us exactly how to do
   this. We want to use `overrideAttrs` to change the `buildType`

Change this line:

```nix
default = self.packages.${system}.helix;
```

To this:

```nix
default = self.packages.${system}.helix.overrideAttrs { buildType = "debug"; };
```

- This tells Nix to take the standard Helix package definition and override one
  of its internal attributes (`buildType`) to "debug" instead of "release".

3. Build the "Hacked" Helix:

```bash
nix build
```

- Nix will now rebuild Helix, but this time, it will compile it in debug mode.
  You'll likely notice the build takes a bit longer, and the resulting binary
  will be larger due to the included debugging symbols.

4. Run the Debug Binary:

```bash
./result/bin/hx
```

- You're now running your custom-built debug version of Helix! This is useful if
  you were, for example, attatching a debugger.

This is a simple yet powerful "hack" that demonstrates how easily you can modify
the behavior of a package defined within a Nix flake without changing the
original source code or the upstream flake directly. You're simply telling Nix
how you'd like your version of the package to be built.

### Another way to Modify Behavior

Since we are already familiar with the structure and behavior of Helix’s
`flake.nix`, we can leverage that understanding to create our own Nix flake. By
analyzing how Helix organizes its `inputs`, `outputs`, and package definitions,
we gain the confidence to modify and extend a flake’s functionality to suit our
specific needs—whether that’s customizing builds, adding overlays, or
integrating with home-manager.

1. Create a `flake.nix` in your own directory (outside the helix repo):

```nix
{
  description = "Customized Helix build with debug features";

  inputs = {
    helix.url = "github:helix-editor/helix";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    helix,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      system = system;
      overlays = [rust-overlay.overlay.overlays.default];
    };
  in {
    packages.${system}.default = helix.packages.${system}.helix.overrideAttrs (old: {
      buildType = "debug";

      # Add additional cargo features
      cargoBuildFlags =
        (old.cargoBuildFlags or [])
        ++ [
          "--features"
          "tokio-console"
        ];

      # Inject custom RUSTFLAGS
      RUSTFLAGS = (old.RUSTFLAGS or "") + " -C debuginfo=2 -C opt-level=1";
    });
  };
}
```

Check it:

```bash
nix flake check
warning: creating lock file '"/home/jr/world/flake.lock"':
• Added input 'helix':
    'github:helix-editor/helix/8961ae1dc66633ea6c9f761896cb0d885ae078ed?narHash=sha256-f14perPUk%2BH15GyGRbg0Akqhn3rxFnc6Ez5onqpzu6A%3D' (2025-05-29)
• Added input 'helix/nixpkgs':
    'github:nixos/nixpkgs/5135c59491985879812717f4c9fea69604e7f26f?narHash=sha256-Vr3Qi346M%2B8CjedtbyUevIGDZW8LcA1fTG0ugPY/Hic%3D' (2025-02-26)
• Added input 'helix/rust-overlay':
    'github:oxalica/rust-overlay/d342e8b5fd88421ff982f383c853f0fc78a847ab?narHash=sha256-3SdPQrZoa4odlScFDUHd4CUPQ/R1gtH4Mq9u8CBiK8M%3D' (2025-02-27)
• Added input 'helix/rust-overlay/nixpkgs':
    follows 'helix/nixpkgs'
• Added input 'nixpkgs':
    'github:nixos/nixpkgs/96ec055edbe5ee227f28cdbc3f1ddf1df5965102?narHash=sha256-7doLyJBzCllvqX4gszYtmZUToxKvMUrg45EUWaUYmBg%3D' (2025-05-28)
• Added input 'rust-overlay':
    'github:oxalica/rust-overlay/405ef13a5b80a0a4d4fc87c83554423d80e5f929?narHash=sha256-k0nhPtkVDQkVJckRw6fGIeeDBktJf1BH0i8T48o7zkk%3D' (2025-05-30)
• Added input 'rust-overlay/nixpkgs':
    follows 'nixpkgs'
```

- The `nix flake check` command will generate a `flake.lock` file if one doesn't
  exist, and the warnings you see indicate that new inputs are being added and
  locked to specific versions for reproducibility. This is expected behavior for
  a new or modified flake.

Inspect the outputs:

```bash
nix flake show
path:/home/jr/world?lastModified=1748612128&narHash=sha256-WEYtptarRrrm0Jb/0PJ/b5VPqLkCk5iEenjbKYU4Xm8%3D
└───packages
    └───x86_64-linux
        └───default: package 'helix-term'
```

- The `└───packages` line indicates that our flake exposes a top-level
  `packages` attribute.

- `└───x86_64-linux`: System architecture specificity

- `└───default: package 'helix-term'` Signifies that within the `x86_64-linux`
  packages, there's a package named `default`. This is a special name that
  allows you to omit the package name when using commands like `nix build`.

- `package 'helix-term'` This is the most direct confirmation of our "hack". It
  tells us that our `default` package is `helix-term`. This confirms that our
  `overrideAttrs` in the `packages.${system}.default` section successfully
  targeted and modified the Helix editor package, which is internally named
  `helix-term` by the Helix flake.

**What This Does**:

- `overrideAttrs` lets you change _only_ parts of the derivation without
  rewriting everything.

- `buildType = "debug"` enables debug builds.

- `cargoBuildFlags` adds extra features passed to Cargo, e.g.,
  `--features tokio-console`

- `RUSTFLAGS` gives you even more control over compiler behavior, optimization
  levels, etc.

**Run It**:

```bash
nix run
```

Or drop into the dev shell:

```bash
nix develop
```

- (assuming you also wire in a `devShells` output)

**Adding the `devShells` output**:

Since we already have the helix flake as an input to our own `flake.nix` we can
now forward or extend Helix's `devShells` like this:

```nix
outputs = { self, nixpkgs, helix, rust-overlay, ... }: {
  devShells = helix.devShells;
};
```

Or if you want to pick a specific system:

```nix
outputs = { self, nixpkgs, helix, rust-overlay ... }:
  let
    system = "x86_64-linux";
  in {
    devShells.${system} = helix.devShells.${system};
  };
```

**Optional: Combine with your own** `devShell`

You can also extend or merge it with your own shell like so:

```nix
outputs = { self, nixpkgs, helix, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system} = {
      default = pkgs.mkShell {
        name = "my-shell";
        inputsFrom = [ helix.devShells.${system}.default ];
        buildInputs = [ pkgs.git ];
      };
    };
  };
```
