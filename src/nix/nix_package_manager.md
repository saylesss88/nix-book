---
title: Nix Package Manager
date: 2025-11-22
author: saylesss88
---

# Nix Package Manager

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![nix99](../images/nix99.png) -->

## Nix Package Manager

Nix is a _purely functional package manager_. This means that it treats packages
like values in purely functional programming languages -- they are built by
functions that don't have side-effects, and they never change after they have
been built.

Nix stores packages in the _Nix store_, usually the directory `/nix/store`,
where each package has its own unique subdirectory such as:

```bash
/nix/store/y53c0lamag5wpx7vsiv7wmnjdgq97yd6-yazi-25.5.14pre20250526_74a8ea9
```

You can use the Nix on most Linux distributions and Mac OS also has good support
for Nix. It should work on most platforms that support POSIX threads and have a
C++11 compiler.

When I install Nix on a distro like Arch Linux I usually use the Zero to Nix
installer as it automates several steps, such as enabling flakes by default:

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

If you have concerns about the "curl to Bash" approach you could examine the
installation script
[here](https://raw.githubusercontent.com/DeterminateSystems/nix-installer/main/nix-installer.sh)
then download and run it:

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix > nix-installer.sh
chmod +x nix-installer.sh
./nix-installer.sh install
```

I got the above commands from
[zero-to-nix](https://zero-to-nix.com/start/install/)

The main difference between using the nix package manager on another
distribution and NixOS is that NixOS uses Nix not just for package management
but also to manage the system configuration (e.g., to build config files in
`/etc`).

[Home Manager](https://nix-community.github.io/home-manager/) is a Nix-powered
tool for reproducible management of the contents of the users' home directories.
This includes programs, configuration files, environment variables, and
arbitrary files. Home manager uses the same module system as NixOS.

Now that we've discussed some of the basics of the Nix package manager, lets see
how it is used to build and manage software in NixOS.

## Channels

Nix packages are distributed through Nix channels; mechanisms for distributing
Nix expressions and the associated binary caches for them. Channels are what
determine which versions your packages have. (i.e. _stable_ or _unstable_). A
channel is a name for the latest "verified" git commits in Nixpkgs. Each channel
represents a different policy for what "verified" means. Whenever a new commit
in `Nixpkgs` passes the verification process, the respective channel is updated
to point to that new commit.

While channels provide a convenient way to get the latest stable or unstable
packages, they introduce a challenge for strict reproducibility. Because a
channel like `nixos-unstable` is constantly updated, fetching packages from it
today might give you a different set of package versions than fetching from it
tomorrow, even if your configuration remains unchanged. This "rolling release"
nature at a global level can make it harder to share and reproduce exact
development environments or system configurations across different machines or
at different points in time.

## Channels vs. Flakes Enhancing Reproducibility

Before the introduction of **Nix Flakes**, channels were the primary mechanism
for sourcing `Nixpkgs`. While functional, they posed a challenge for exact
reproducibility because they point to a moving target (the latest commit on a
branch). This meant that a `nix-build` command run yesterday might produce a
different result than one run today, simply because the channel updated.

Nix Flakes were introduced to address this. Flakes bring a built-in,
standardized way to define the exact inputs to a Nix build, including the
precise Git revision of `Nixpkgs` or any other dependency.

Here's a quick comparison:

| Feature              | Nix Channels (traditional)                                  | Nix Flakes (modern approach)                                                 |
| :------------------- | :---------------------------------------------------------- | :--------------------------------------------------------------------------- |
| **Input Source**     | Global system configuration (`nix-channel --update`)        | Explicitly defined in `flake.nix` (e.g., `github:NixOS/nixpkgs/nixos-23.11`) |
| **Reproducibility**  | "Rolling release"; less reproducible across time/machines   | Highly reproducible due to locked inputs (`flake.lock`)                      |
| **Dependency Mgmt.** | Implicitly managed by global channel                        | Explicitly declared and version-locked within `flake.nix`                    |
| **Sharing**          | Relies on users having same channel version                 | Self-contained; `flake.lock` ensures everyone gets same inputs               |
| **Learning Curve**   | Simpler initial setup, but tricky reproducibility debugging | Higher initial learning curve, but simplifies reproducibility                |

The ability of Flakes to "lock" the exact version of all dependencies in a
`flake.lock` file is a game-changer for collaboration and long-term
reproducibility, ensuring that your Nix configuration builds the same way, every
time, everywhere.

## Nixpkgs

**Nixpkgs** is the largest repository of Nix packages and NixOS modules.

For **NixOS** users, `nixos-unstable` channel branch is the rolling release,
where the packages are tested and must pass integration tests.

For **standalone Nix** users, `nixpkgs-unstable` channel branch is the rolling
release, where packages pass only basic build tests and are upgraded often.

For Flakes, as mentioned above they don't use channels so `nixpkgs` will be
listed as an `input` to your flake. (e.g.,
`inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";`) When using flakes
you can actually disable channels and actually recommended to avoid conflicts
between traditional channel-based workflows and the flake system.

### Updates

The mechanism for updating your Nix environment differs fundamentally between
channels and flakes, directly impacting reproducibility and control.

#### Updating with Channels (Traditional Approach)

With channels, updates are a global operation that pulls the latest state of a
specific branch.

**How it works**: You typically use `nix-channel --update` to fetch the latest
commit from the channels you've subscribed to. For instance,
`sudo nix-channel --update nixos` (for NixOS) or `nix-channel --update nixpkgs`
(for `nix-env` on other Linux distributions).

**Implication**: This command updates your local system's understanding of what
"nixos" or "nixpkgs-unstable" means. From that point on, any
`nixos-rebuild switch`, `nix-env -iA`, or `nix-build` commands that implicitly
or explicitly refer to `nixpkgs` will use this newly updated version.

**Reproducibility Challenge**: The update itself is not recorded in your
configuration files. If you share your `configuration.nix` with someone, they
might run `nix-channel --update` on a different day and get a different set of
package versions because the channel has moved. This makes it challenging to
guarantee that two users building the "same" configuration will get identical
results. You're effectively relying on the implicit, globally managed state of
your channels.

#### Updating with Flakes (Modern Approach)

**Flakes**, by contrast, use a more explicit and localized update mechanism tied
to your `flake.lock` file.

**How it works**: When you define a `flake.nix`, you specify the exact URL
(e.g., a Git repository with a specific branch or tag) for each input. When you
first use a flake, Nix resolves these URLs to a precise Git commit hash and
records this hash, along with a content hash, in a `flake.lock` file.

To update your flake inputs, you run `nix flake update`.

**Implication**: This command goes to each input's specified URL (e.g.,
`github:NixOS/nixpkgs/nixos-unstable`) and fetches the latest commit for that
input. It then updates your `flake.lock` file with the new, precise Git commit
hash and content hash for that input. Your `flake.nix` itself doesn't change,
but the `flake.lock` file now points to newer versions of your dependencies.

**Reproducibility Advantage**: The `flake.lock` file acts as a manifest of your
exact dependency versions.

**Sharing**: When you share your flake (the `flake.nix` and `flake.lock` files),
anyone using it will fetch precisely the same Git commit hashes recorded in the
`flake.lock`, guaranteeing identical inputs and thus, identical builds (assuming
the same system architecture).

**Updating Selectively**: You can update individual inputs within your flake by
specifying them: `nix flake update nixpkgs`. This provides fine-grained control
over which parts of your dependency graph you want to advance.

**Rolling Back**: Because the `flake.lock` explicitly records the versions, you
can easily revert to a previous state by checking out an older `flake.lock` from
your version control system.

**In essence**: Channels involve a global "pull" of the latest branch state,
making reproducibility harder to guarantee across time and machines. Flakes,
however, explicitly pin all inputs in `flake.lock`, and updates involve
explicitly refreshing these pins, providing strong reproducibility and version
control out of the box.

### Managing software with Nix

**Derivation Overview**

In Nix, the process of managing software starts with **package definitions**.
These are files written in the Nix language that describe how a particular piece
of software should be built. These package definitions, when processed by Nix,
are translated into derivations.

At its core, a derivation in Nix is a blueprint or a recipe that describes how
to build a specific software package or any other kind of file or directory.
It's a declarative specification of:

- **Inputs**: What existing files or other derivations are needed as
  dependencies.

- **Build Steps**: The commands that need to be executed to produce the desired
  output.

- **Environment**: The specific environment (e.g., build tools, environment
  variables) required for the build process.

- **Outputs**: The resulting files or directories that the derivation produces.

Think of a package definition as the initial instructions, and the derivation as
the detailed, low-level plan that Nix uses to actually perform the build.

Again, a derivation is like a blueprint that describes how to build a specific
software package or any other kind of file or directory.

**Key Characteristics of Derivations:**

- **Declarative**: You describe the desired outcome and the inputs, not the
  exact sequence of imperative steps. Nix figures out the necessary steps based
  on the builder and args.

- **Reproducible**: Given the same inputs and build instructions, a derivation
  will always produce the same output. This is a cornerstone of Nix's
  reproducibility.

- **Tracked by Nix**: Nix keeps track of all derivations and their outputs in
  the Nix store. This allows for efficient management of dependencies and
  ensures that different packages don't interfere with each other.

- **Content-Addressed**: The output of a derivation is stored in the Nix store
  under a unique path that is derived from the hash of all its inputs and build
  instructions. This means that if anything changes in the derivation, the
  output will have a different path.

Here's a simple Nix derivation that creates a file named hello in the Nix store
containing the text "Hello, World!":

<details>
<summary> ✔️ Hello World Derivation Example (Click to expand):</summary>

```nix
{pkgs ? import <nixpkgs> {}}:
pkgs.stdenv.mkDerivation {
  name = "hello-world";

  dontUnpack = true;

  # No need for src = null; when dontUnpack = true;
  # src = null;

  buildPhase = ''
     # Create a shell script that prints "Hello, World!"
    echo '#!${pkgs.bash}/bin/bash' > hello-output-file # Shebang line
    echo 'echo "Hello, World!"' >> hello-output-file # The command to execute
    chmod +x hello-output-file # Make it executable
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp hello-output-file $out/bin/hello # Copy the file from build directory to $out/bin
  '';

  meta = {
    description = "A simple Hello World program built with Nix";
    homepage = null;
    license = pkgs.lib.licenses.unfree; # licenses.mit is often used as well
    maintainers = [];
  };
}
```

And a `default.nix` with the following contents:

```nix
{ pkgs ? import <nixpkgs> {} }:

import ./hello.nix { pkgs = pkgs; }
```

- `{ pkgs ? import <nixpkgs> {} }`: This is a function that takes an optional
  argument `pkgs`. We need Nixpkgs to access standard build environments like
  `stdenv`.

- `pkgs.stdenv.mkDerivation { ... }:` This calls the mkDerivation function from
  the standard environment (stdenv). mkDerivation is the most common way to
  define software packages in Nix.

- `name = "hello-world";`: Human-readable name of the derivation

- The rest are the build phases and package metadata.

To use the above derivation, save it as a `.nix` file (e.g. `hello.nix`). Then
build the derivation using,:

```bash
nix-build
this derivation will be built:
  /nix/store/9mc855ijjdy3r6rdvrbs90cg2gf2q160-hello-world.drv
building '/nix/store/9mc855ijjdy3r6rdvrbs90cg2gf2q160-hello-world.drv'...
Running phase: patchPhase
Running phase: updateAutotoolsGnuConfigScriptsPhase
Running phase: configurePhase
no configure script, doing nothing
Running phase: buildPhase
Running phase: installPhase
Running phase: fixupPhase
shrinking RPATHs of ELF executables and libraries in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
checking for references to /build/ in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world...
patching script interpreter paths in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
stripping (with command strip and flags -S -p) in  /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world/bin
/nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
```

- Nix will execute the `buildPhase` and `installPhase`

- After a successful build, the output will be in the Nix store. You can find
  the exact path by looking at the output of the nix build command (it will be
  something like `/nix/store/your-hash-hello-world`).

Run the "installed" program:

```bash
./result/bin/hello
```

- This will execute the `hello` file from the Nix store and print
  `"Hello, World!"`.

</details>
