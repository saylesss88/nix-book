---
title: Local Nixpkgs
date: 2025-11-22
author: saylesss88
---

# Chapter 10

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![server_rack](images/server_rack.cleaned.png)

<!-- ![gruv18](images/gruv18.png) -->

## Working with Nixpkgs Locally: Benefits and Best Practices

- Nixpkgs, the package repository for NixOS, is a powerful resource for building
  and customizing software.

- Working with a local copy enhances development, debugging, and contribution
  workflows.

- This post covers setting up a local Nixpkgs repository, searching for
  dependencies, and leveraging its advantages, incorporating tips from the Nix
  community.

# I. Why Work with Nixpkgs Locally?

- A local Nixpkgs repository offers significant advantages for Nix developers:

  ## A. Faster Development Cycle
  - Local searches for packages and dependencies are significantly quicker than
    querying remote repositories or channels.

  - This speedup is crucial for efficient debugging and rapid prototyping of Nix
    expressions.

  ## B. Enhanced Version Control
  - By pinning your local repository to specific commits or branches (e.g.,
    `nixos-unstable`), you ensure build reproducibility.

  - This prevents unexpected issues arising from upstream changes in Nixpkgs.

  ## C. Flexible Debugging Capabilities
  - You can directly test and modify package derivations within your local copy.

  - This allows for quick fixes to issues like missing dependencies without
    waiting for upstream updates or releases.

  ## D. Streamlined Contribution Workflow
  - Developing and testing new packages or patches locally is essential before
    submitting them as pull requests to Nixpkgs.

  - A local setup provides an isolated environment for experimentation.

  ## E. Up-to-Date Documentation Source
  - The source code and comments within the Nixpkgs repository often contain the
    most current information about packages.
  - This can sometimes be more up-to-date than official, external documentation.

  ## F. Optimized Storage and Performance
  - Employing efficient cloning strategies (e.g., shallow clones) and avoiding
    unnecessary practices (like directly using Nixpkgs as a flake for local
    development) minimizes disk usage and build times.

# II. Flake vs. Non-Flake Syntax for Local Nixpkgs

- When working with Nixpkgs locally, the choice between Flake and non-Flake
  syntax has implications for performance and storage:

  ## A. Flake Syntax (`nix build .#<package>`)
  - Treats the current directory as a flake, requiring evaluation of
    `flake.nix`.

  - For local Nixpkgs, this evaluates the flake definition in the repository
    root.

  - **Performance and Storage Overhead:** Flakes copy the entire working
    directory (including Git history if present) to `/nix/store` for evaluation.
    This can be slow and storage-intensive for large repositories like Nixpkgs.

  ## B. Non-Flake Syntax (`nix-build -f . <package>` or `nix build -f . <package>`)
  - `-f .` specifies the Nix expression (e.g., `default.nix` or a specific file)
    in the current directory.

  - **Efficiency:** Evaluates the Nix expression directly _without_ copying the
    entire worktree to `/nix/store`. This is significantly faster and more
    storage-efficient for local development on large repositories.

## III. Setting Up a Local Nixpkgs Repository Efficiently

- Cloning Nixpkgs requires careful consideration due to its size.

## A.a Initial Clone: Shallow Cloning

It is common to place your local clone in the `/src` directory:

```bash
mkdir src && cd src
```

> ❗ Warning, A shallow clone (`--depth 1`) is not recommended for general
> development or contributing changes back to Nixpkgs via pull requests. It's
> primarily suitable for:
>
> - Quick checks or builds: If you only need to verify a package's current state
>   or build a specific version without needing historical context.
> - CI/CD environments: Where disk space and clone time are critical, and only
>   the latest commit is needed for automated tests or builds.

With that said, to avoid downloading the entire history, perform a shallow
clone:

```bash
git clone [https://github.com/NixOS/nixpkgs](https://github.com/NixOS/nixpkgs) --depth 1
cd nixpkgs
```

## A.b A few Examples exploring Nixpkgs

While in the `nixpkgs` directory, you can check the version of a package:

```bash
nix-instantiate --eval -A openssl.version
"3.4.1"
```

Or to directly edit the file you can use `nix edit`:

```bash
nix edit nixpkgs#openssl
```

It uses the nix registry and `openssl.meta.position` to locate the file.

```bash
man nix3 registry
```

The above command will open the `openssl/default.nix` in your `$EDITOR`.

---

## A.1 Full Fork and Clone of Nixpkgs

If you want to contribute to Nixpkgs, you need to set up a local version
following the
[Contributing guide](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md)

You'll need to, this is directly from the `Contributing.md`:

1. [Fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo#forking-a-repository)
   the [Nixpkgs repository](https://github.com/nixos/nixpkgs/)

2. [Clone the forked repo](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo#cloning-your-forked-repository)
   into a local `nixpkgs` directory.

```bash
git clone git@github.com:your-user/nixpkgs.git
```

3. [Configure the upstream Nixpkgs repo](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo#configuring-git-to-sync-your-fork-with-the-upstream-repository)

```bash
git remote add upstream git@github.com:NixOS/nixpkgs.git
# Check them out
git remove -v
```

### The Three Master Branches

1. **Upstream Master** (upstream/master):

- **Where it is**: On the official NixOS servers.

- **Role**: The absolute source of truth. Thousands of people are pushing to
  this daily.

2. **Local Master** (master):

- **Where it is**: On your physical computer.

- **Role**: Your working copy. This is the only one you can actually "rebase" or
  "commit" to directly.

3. **Origin Master** (origin/master):

- **Where it is**: On your GitHub fork (github.com/your-user/nixpkgs).

- **Role**: A personal backup and a place to host your code so you can open Pull
  Requests.

### Create a branch

In the nixpkgs ecosystem, the "cleanest" way to work is to **never** add your
own commits to your local master.

- Keep your `master` as a "pure mirror" of `upstream/master`.

- Whenever you want to fix a package or add something new, create a feature
  branch:

```bash
git checkout -b fix-my-package master
```

This way, syncing is as simple as a reset:

```bash
# Total reset of your local master to match the official one
git fetch upstream
git checkout master
git reset --hard upstream/master
git push origin master --force
```

### Pushing a PR

When you're ready to push changes you push to `origin/feature-branch`, visit
your fork on github.com and submit the PR.

## B. Managing Branches with Worktrees

- Use Git worktrees to manage different branches efficiently:

  ```bash
  git fetch --all --prune --depth=1
  git worktree add -b nixos-unstable nixos-unstable # For just unstable
  ```

- **Explanation of `git worktree`:** Allows multiple working directories
  attached to the same `.git` directory, sharing history and objects but with
  separate checked-out files.

- `git worktree add`: Creates a new working directory for the specified branch
  (`nixos-unstable` in this case).

# IV. Debugging Missing Dependencies: A Practical Example

<details>
<summary> Click to see icat Example </summary>

- Let's say you're trying to build `icat` locally and encounter a missing
  dependency error:

```nix
nix-build -A icat
# ... (Error log showing "fatal error: X11/Xlib.h: No such file or directory")
```

- The error `fatal error: X11/Xlib.h: No such file or directory` indicates a
  missing X11 dependency.

## A. Online Search with `search.nixos.org`

- The Nixpkgs package search website
  ([https://search.nixos.org/packages](https://search.nixos.org/packages)) is a
  valuable first step.
- However, broad terms like "x11" can yield many irrelevant results.
- **Tip:** Utilize the left sidebar to filter by package sets (e.g., "xorg").

## B. Local Source Code Search with `rg` (ripgrep)

- Familiarity with searching the Nixpkgs source code is crucial for finding
  dependencies.
- Navigate to your local `nixpkgs/` directory and use `rg`:

  ```bash
  rg "x11 =" pkgs # Case-sensitive search
  ```

  **Output:**

  ```
  pkgs/tools/X11/primus/default.nix
  21:  primus = if useNvidia then primusLib_ else primusLib_.override { nvidia_x11 = null; };
  22:  primus_i686 = if useNvidia then primusLib_i686_ else primusLib_i686_.override { nvidia_x11 = null; };

  pkgs/applications/graphics/imv/default.nix
  38:    x11 = [ libGLU xorg.libxcb xorg.libX11 ];
  ```

- Refining the search (case-insensitive):
  ```bash
  rg -i "libx11 =" pkgs
  ```
  **Output:**
  ```
  # ... (Output showing "xorg.libX11")
  ```
- The key result is `xorg.libX11`, which is likely the missing dependency.

 </details>

# V. Local Derivation Search with `nix-locate`

<details>
<summary> Click to Expand nix-locate command Example</summary>

- `nix-locate` (from the `nix-index` package) allows searching for derivations
  on the command line.

  > **Note:** Install `nix-index` and run `nix-index` to create the initial
  > index.

  ```bash
  nix-locate libx11
  # ... (Output showing paths related to libx11)
  ```

- Combining online and local search tools (`search.nixos.org`, `rg`,
  `nix-locate`) provides a comprehensive approach to finding dependencies.

</details>

# VI. Key Benefits of Working with Nixpkgs Locally (Recap)

- **Speed:** Faster searches and builds compared to remote operations.

- **Control:** Full control over the Nixpkgs version.

- **Up-to-Date Information:** Repository source often has the latest details.

# VII. Best Practices and Tips from the Community

<details>
<summary> ✔️ Click To Expand Best Practices and Tips from the community</summary>

- **Rebasing over Merging:** Never merge upstream changes into your local
  branch. Always rebase your branch onto the upstream (e.g., `master` or
  `nixos-unstable`) to avoid accidental large-scale pings in pull requests (Tip
  from `soulsssx3` on Reddit).

- **Tip from `ElvishJErrico`:** Avoid using Nixpkgs directly as a flake for
  local development due to slow copying to `/nix/store` and performance issues
  with garbage collection on large numbers of small files. Use
  `nix build -f . <package>` instead of `nix build .#<package>`.

- **Edge Cases for Flake Syntax:** Flake syntax might be necessary in specific
  scenarios, such as NixOS installer tests where copying the Git history should
  be avoided.

- **Base Changes on `nixos-unstable`:** For better binary cache hits, base your
  changes on the `nixos-unstable` branch instead of `master`. Consider the
  merge-base for staging branches as well.

- **Consider `jujutsu`:** Explore [jj-vcs](https://github.com/jj-vcs/jj), a
  Git-compatible alternative that can offer a more intuitive workflow,
  especially for large monorepos like Nixpkgs. While it has a learning curve, it
  can significantly improve parallel work and branch management.

- [Intro-To-Jujutsu](https://saylesss88.github.io/vcs/jujutsu.html)

</details>
