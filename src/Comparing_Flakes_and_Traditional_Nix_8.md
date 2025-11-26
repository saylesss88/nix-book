---
title: Comparing Flakes and Traditional Nix
date: 2025-11-22
author: saylesss88
---

# Chapter 8

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![nixWinter](images/nixWinter.png) -->

## Comparing Flakes and Traditional Nix

- This post is based on notes from Nix-Hour #4, comparing Traditional Nix and
  Flakes, focusing on achieving pure build results. See the
  [YouTube video](https://www.youtube.com/watch?v=atmoYyBAhF4) for the original
  content. This guide adapts the information for clarity and ease of
  understanding.

<details>
<summary> What is Purity in Nix? (click here) </summary>

- A key benefit of Nix Flakes is their _default_ enforcement of **pure
  evaluation**.

- In Nix, an **impure operation** depends on something _outside_ its explicit
  inputs. Examples include:
  - User's system configuration
  - Environment variables
  - Current time

- Impurity leads to unpredictable builds that may differ across systems or time.

</details>

## Building a Simple "hello" Package: Flakes vs. Traditional Nix

- We'll demonstrate building a basic "hello" package using both Flakes and
  Traditional Nix to highlight the differences in handling purity.

## Using Nix Flakes

<details>
<summary> Building Hello with Flakes (click here) </summary>

1.  **Setup:**

    ```bash
    mkdir hello && cd hello/
    ```

2.  **Create `flake.nix` (Initial Impure Example):**

    ```nix
    # flake.nix
    {
      outputs = { self, nixpkgs }: {
        myHello = (import nixpkgs {}).hello;
      };
    }
    ```

    - Note: Flakes don't have access to `builtins.currentSystem` directly.

3.  **Impure Build (Fails):**

    ```bash
    nix build .#myHello
    ```

    - This fails because Flakes enforce purity by default.

4.  **Force Impure Build:**

    ```bash
    nix build .#myHello --impure
    ```

5.  **Making the Flake Pure:**

    ```nix
    # flake.nix
    {
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
      };

      outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in {
            packages.myHello = pkgs.hello;
          }
        );
    }
    ```

    - `flake-utils` simplifies making flakes system-agnostic and provides the
      `system` attribute.

6.  **Pure Build (Success):**
    ```bash
    nix build .#myHello
    ```

  </details>

## Using Traditional Nix

<details>
<summary> Building hello with Traditional Nix </summary>

1.  **Setup:**

    ```bash
    mkdir hello2 && cd hello2/
    ```

2.  **Create `default.nix` (Initial Impure Example):**

    ```nix
    # default.nix
    { myHello = (import <nixpkgs> { }).hello; }
    ```

3.  **Build (Impure):**

    ```bash
    nix-build -A myHello
    ```

4.  **Impurity Explained:**

    ```bash
    nix repl
    nix-repl> <nixpkgs>
    /nix/var/nix/profiles/per-user/root/channels/nixos
    ```

    - `<nixpkgs>` depends on the user's environment (Nixpkgs channel), making it
      impure. Even with channels disabled, it relies on a specific Nixpkgs
      version in the store.

5.  **Achieving Purity: Using `fetchTarball`**
    - GitHub allows downloading repository snapshots at specific commits,
      crucial for reproducibility.

    - **Get Nixpkgs Revision from `flake.lock` (from the Flake example):**

    ```nix
    # flake.lock
    "nixpkgs": {
      "locked": {
        "lastModified": 1746372124,
        "narHash": "sha256-n7W8Y6bL7mgHYW1vkXKi9zi/sV4UZqcBovICQu0rdNU=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "f5cbfa4dbbe026c155cf5a9204f3e9121d3a5fe0",
        "type": "github"
      },
    ```

6.  **Modify `default.nix` for Purity:**

    ```nix
    # default.nix
    let
      nixpkgs = fetchTarball {
        url = "[https://github.com/NixOS/nixpkgs/archive/f5cbfa4dbbe026c155cf5a9204f3e9121d3a5fe0.tar.gz](https://github.com/NixOS/nixpkgs/archive/f5cbfa4dbbe026c155cf5a9204f3e9121d3a5fe0.tar.gz)";
        sha256 = "0000000000000000000000000000000000000000000000000000"; # Placeholder
      };
    in {
      myHello = (import nixpkgs {}).hello;
    }
    ```

    - Replace `<nixpkgs>` with `fetchTarball` and a specific revision. A
      placeholder `sha256` is used initially.

7.  **Build (Nix provides the correct `sha256`):**

    ```bash
    nix-build -A myHello
    ```

8.  **Verification:** Both Flake and Traditional Nix builds now produce the same
    output path.

9.  **Remaining Impurities in Traditional Nix:**
    - Default arguments to `import <nixpkgs> {}` can introduce impurity:
      - `overlays`: `~/.config/nixpkgs/overlays` (user-specific)
      - `config`: `~/.config/nixpkgs/config.nix` (user-specific)
      - `system`: `builtins.currentSystem` (machine-specific)

10. **Making Traditional Nix Fully Pure:**

    ```nix
    # default.nix
    {system ? builtins.currentSystem}:
    let
      nixpkgs = fetchTarball {
        url =
          "[https://github.com/NixOS/nixpkgs/archive/0243fb86a6f43e506b24b4c0533bd0b0de211c19.tar.gz](https://github.com/NixOS/nixpkgs/archive/0243fb86a6f43e506b24b4c0533bd0b0de211c19.tar.gz)";
        sha256 = "1qvdbvdza7hsqhra0yg7xs252pr1q70nyrsdj6570qv66vq0fjnh";
      };
    in {
      myHello = (import nixpkgs {
        overlays = [];
        config = {};
        inherit system;
      }).hello;
    }
    ```

    - Override impure defaults for `overlays`, `config`, and make `system` an
      argument.

11. **Building with a Specific System:**

    ```bash
    nix-build -A myHello --argstr system x86_64-linux
    ```

12. **Pure Evaluation Mode in Traditional Nix:**

    ```bash
    nix-instantiate --eval --pure-eval --expr 'fetchGit { url = ./.; rev = "b4fe677e255c6f89c9a6fdd3ddd9319b0982b1ad"; }'
    ```

    - Example of using `--pure-eval`.

    ```bash
    nix-build --pure-eval --expr '(import (fetchGit { url = ./.; rev = "b4fe677e255c6f89c9a6fdd3ddd9319b0982b1ad"; }) { system = "x86_64-linux"; }).myHello'
    ```

    - Building with a specific revision and system.

  </details>

### Updating Nixpkgs

<details>
<summary> Updating Nixpkgs with Flakes </summary>

```bash
nix flake update
```

```nix
nix build .#myHello --override-input nixpkgs github:NixOS/nixpkgs/nixos-24.11
```

</details>

### Updating Traditional Nix (using `niv`)

<details>
<summary> Updating with niv </summary>

```nix
nix-shell -p niv
niv init
```

```nix
# default.nix
{ system ? builtins.currentSystem,
  sources ? import nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  pkgs ? import nixpkgs {
    overlays = [ ];
    config = { };
    inherit system;
  }, }: {
  myHello = pkgs.hello;
}
```

And build it with:

```bash
nix-build -A myHello
```

```bash
niv update nixpkgs --branch=nixos-unstable
nix-build -A myHello
```

</details>

<details>
<summary> Adding Home-Manager with Flakes (click here) </summary>

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    home-manager.url = "github:nix-community/home-manager";
  };

  outputs = { self, nixpkgs, flake-utils, home-manager, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.myHello = pkgs.hello;
        packages.x86_64-linux.homeManagerDocs =
          home-manager.packages.x86_64-linux.docs-html;
      });
}
```

```bash
nix flake update
nix flake show github:nix-community/home-manager
```

```nix
home-manager.inputs.follows = "nixpkgs";
```

</details>

#### Adding Home-Manager with Traditional Nix

<details>
<summary> Adding Home-Manager with Traditional Nix (click here) </summary>
```nix
niv add nix-community/home-manager
```

```nix
nix repl
nix-repl> s = import ./nix/sources.nix
nix-repl> s.home-manager
```

```nix
{ system ? builtins.currentSystem, sources ? import nix/sources.nix
  , nixpkgs ? sources.nixpkgs, pkgs ? import nixpkgs {
    overlays = [ ];
    config = { };
    inherit system;
  }, }: {
  homeManagerDocs = (import sources.home-manager { pkgs = pkgs; }).docs;

  myHello = pkgs.hello;
}
```

```bash
nix-build -A homeManagerDocs
```

</details>

#### Conclusion

In this chapter, we've explored the key differences between traditional Nix and
Nix Flakes, particularly focusing on how each approach handles purity,
dependency management, and project structure. We've seen that while traditional
Nix can achieve purity with careful configuration, Flakes enforce it by default,
offering a more robust and standardized way to build reproducible environments.
Flakes also streamline dependency management and provide a more structured
project layout compared to the often ad-hoc nature of traditional Nix projects.

However, regardless of whether you're working with Flakes or traditional Nix,
understanding how to debug and trace issues within your Nix code is crucial.
When things go wrong, you'll need tools and techniques to inspect the evaluation
process, identify the source of errors, and understand how your modules and
derivations are being constructed.

In our next chapter,
[Debugging and Tracing Modules](https://saylesss88.github.io/Debugging_and_Tracing_NixOS_Modules_9.html),
we will delve into the world of Nix debugging. We'll explore various techniques
and tools that can help you understand the evaluation process, inspect the
values of expressions, and trace the execution of your Nix code, enabling you to
effectively troubleshoot and resolve issues in both Flake-based and traditional
Nix projects.
