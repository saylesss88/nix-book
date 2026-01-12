---
title: NixOS Containers
date: 2026-01-11
author: saylesss88
collection: "blog"
tags: ["nixos", "containers"]
draft: false
---

# NixOS Containers

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![boxes](images/boxes.cleaned.png)

NixOS containers are lightweight `systemd-nspawn` containers managed
declaratively through your NixOS configuration. They allow you to run separate,
minimal NixOS instances on the same machine, each with its own services,
packages, and (optionally) network stack.

- [freedesktop systemd-nspawn](https://www.freedesktop.org/software/systemd/man/latest/systemd-nspawn.html?__goaway_challenge=meta-refresh&__goaway_id=5497ebb54af7da76c7cff2e5210fe9ab&__goaway_referer=https%3A%2F%2Fsearch.brave.com%2F)

> ❗ NixOS’ containers do not provide full security out of the box (just like
> docker). They do give you a separate chroot, but a privileged user (root) in a
> container can escape the container and become root on the host system.
> --[beardhatcode Declarative-Nixos-Containers](https://blog.beardhatcode.be/2020/12/Declarative-Nixos-Containers.html)

**Common Use Cases**

- **Isolating services**: Run a web server, database, or any service in its own
  container, so it can’t interfere with the main system or other services

- **Testing and development**: Try out new configurations, packages, or services
  in a sandboxed environment.

- **Reproducible deployments**: Because containers are defined declaratively,
  you can reproduce the exact same environment anywhere.

- **Running multiple versions of a service**: For example, testing different
  versions of Git or HTTP servers side by side.

---

## Hosting mdBook

Let’s say you want to host your mdBook. You can define a NixOS container that
runs only the necessary service, isolated from your main system:

```nix
{
  config,
  lib,
  ...
}: {
  containers.mdbook-host = {
    autoStart = true;
    ephemeral = true;
    privateNetwork = false;  # Use the hosts network

    bindMounts."/var/www/mdbook" = {
      hostPath = "/home/jr/nix-book/book";
      isReadOnly = true;
    };

    config = {containerPkgs, ...}: {
      networking.useDHCP = lib.mkDefault true;

      services.httpd = {
        enable = true;
        adminAddr = "yourEmail.com";
        virtualHosts."localhost" = {
          documentRoot = "/var/www/mdbook";
          serverAliases = [];
        };
      };

      networking.firewall.allowedTCPPorts = [80];
      environment.systemPackages = with containerPkgs; [];
      system.stateVersion = "25.05";
    };
  };
}
```

- `ephemeral`: if true, the container resets on each restart.

- `autoStart`: Ensures the container starts automatically at boot.

- `config`: Defines the containers NixOS configuration, just like a regular
  NixOS system.

**Mounts**

```nix
    bindMounts."/var/www/mdbook" = {
      hostPath = "/home/jr/nix-book/book";
      isReadOnly = true;
    };
```

The `bindMount` settings above specify that `/var/www/mdbook` in the container
should be linked to `/home/jr/nix-book/book` on the host.

`hostPath` must exist, and `/var/www/mdbook` must not exist for this to work.

The above container is fairly simple because its `ReadOnly`, things get more
complicated when you need HTTPD to have write privileges.

When you create and run a NixOS container like `mdbook-host`. NixOS stores the
container's root filesystem and related container state data under:

```bash
ls /var/lib/nixos-containers/
╭────────────╮
│ empty list │  # It's empty because we set ephemeral to true
╰────────────╯
```

This directory holds the container's own filesystem image, including system
files, installed packages, configuration, and any data internal to the
container.

---

## Check Container Status

```bash
nixos-container list
mdbook-host
```

```bash
sudo systemctl status container@mdbook-host
 Main PID: 32938 (systemd-nspawn)
     Status: "Container running: Ready."
```

**Test HTTP server inside the container**

We configured Apache (`httpd`) to serve `/var/www/mdbook` at `localhost`

Let's check if Apache is running:

```bash
sudo nixos-container run mdbook-host -- systemctl status httpd
● httpd.service - Apache HTTPD
     Loaded: loaded (/etc/systemd/system/httpd.service; enabled; preset: ignored)
     Active: active (running) since Fri 2025-08-15 10:14:39 EDT; 2min 18s ago
```

Check the Bind Mount:

```bash
sudo nixos-container run mdbook-host -- ls -l /var/www/mdbook
```

- You should see an `index.html` and any other files from `~/nix-book/book`

Test the Web Server:

```bash
curl http://localhost
```

- You should see your book in HTTP format as raw HTML.

Test on the web, in your browser visit:

```text
http://localhost/
```

- You should see your book fully served

---

### Troubleshooting

Make sure your book has the correct permissions to allow `hostPath` to read it:

```bash
sudo chmod -R o+rX ~/nix-book/book
```

If needed restart the container:

```bash
sudo nixos-container stop mdbook-host
sudo nixos-container start mdbook-host
```

Ensure that `/var/www/mdbook` is being populated:

```bash
sudo nixos-container run mdbook-host -- ls -l /var/www/mdbook
```

You should see an `index.html` and more

```bash
sudo nixos-container run mdbook-host -- systemctl status httpd
```

- You should see `enabled` & `active (running)`

Check the containers status:

```bash
sudo nixos-container status mdbook-host
up
```

---

## Why Bother Serving your book to localhost?

1. Real-time updates without rebuilding the container

- Files added, changed, or removed from `~/nix-book/book` on the host are
  immediately reflected inside the container. This allows for:
  - Rapid iteration and testing of your books content without rebuilding

  - Easier debugging and fixing content or config issues on the fly.

2. Keeps container images small and immutable

- Instead of baking book files into the container image (which requires
  rebuilding every change), the container image remains clean and generic.

3. Separation of concerns

- The container focuses on running the service, while the content is managed
  independently on the host. This separation improves maintainability and more.

4. Data persistence

- Since the files live on the host, they persist independently of the containers
  lifecycle: restarting, recreating, or destroying the container won't lose your
  content.

5. Security Control

- You can carefully set permissions on the host directory, control read/write
  access, and isolate the container runtime from sensitive data.

---

## Removing the State

To remove `/var/lib/nixos-containers/mdbook-host`, you need to remove the
container configuration, rebuild, and then run the following commands to remove
the immutable sticky bits that prevent deletion.

```bash
# Forcibly remove all attributes
sudo chattr -R -i mdbook-host/
sudo rm -rf mdbook-host/
```

## OCI deployment pipeline building a Rust App

> If you want to use `mdbook-nix-repl` check out the README, the following shows
> how I tested locally before eventually adding a `flake.nix` to the repo
> streamlining this for users of the project.

- [mdbook-nix-repl README](https://github.com/saylesss88/mdbook-nix-repl)

This documents how to work with a local Rust crate repository without a
`flake.nix` for testing. The README above explains how to generate a token and
use the project, this is just for educational purposes if you wanted to
implement something similar:

1. `nix-repl-server.nix`, place this in the same dir as your
   `configuration.nix`:

```nix
{
  config,
  lib,
  pkgs,
  inputs,
  ...
}:

let
  cfg = config.custom.nix-repl-server;

  serverSource = inputs.mdbook-nix-repl + "/server";

  # 1. Build the binary using your package definition
  # serverPkg = pkgs.callPackage ./server-pkg.nix { };
  serverPkg = pkgs.callPackage ./server-pkg.nix {
    src = serverSource;
  };

  # 2. Build a minimal container image containing just the server + nix + deps
  nixReplImage = pkgs.dockerTools.buildLayeredImage {
    name = "nix-repl-server";
    tag = "latest";

    # dependencies needed at runtime inside the container
    contents = [
      serverPkg
      pkgs.nix
      pkgs.bashInteractive
      pkgs.cacert
      pkgs.tini
      pkgs.coreutils
    ];

    config = {
      Entrypoint = [
        "${pkgs.tini}/bin/tini"
        "--"
      ];
      Cmd = [ "${serverPkg}/bin/nix-repl-server" ];
      ExposedPorts = {
        "8080/tcp" = { };
      };
      # Important: Container must see 0.0.0.0 to receive traffic from host port mapping
      Env = [
        "NIX_REPL_BIND=0.0.0.0"
        "NIX_CONFIG=experimental-features = nix-command flakes"
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
      ];
    };
  };
in
{
  options.custom.nix-repl-server = {
    enable = lib.mkEnableOption "nix-repl-server container";
    port = lib.mkOption {
      type = lib.types.port;
      default = 8080;
      description = "Host port to map to the container";
    };
    tokenFile = lib.mkOption {
      type = lib.types.path;
      default = "/etc/nix-repl-server.env";
      description = "Path to file containing NIX_REPL_TOKEN=...";
    };
  };

  config = lib.mkIf cfg.enable {
    # Enable Podman backend
    virtualisation.podman.enable = true;
    virtualisation.oci-containers.backend = "podman";

    # The OCI container definition
    virtualisation.oci-containers.containers.nix-repl-server = {
      image = "nix-repl-server:latest";

      # This effectively "loads" the image into Podman on boot
      imageFile = nixReplImage;

      ports = [ "127.0.0.1:${toString cfg.port}:8080" ];

      # Inject the token safely at runtime (not in Nix store)
      environmentFiles = [ cfg.tokenFile ];

      extraOptions = [
        "--cap-drop=ALL"
        "--security-opt=no-new-privileges"
        "--pull=never" # Use the local loaded image
      ];
    };
  };
}
```

2. `server-pkg.nix`, place this in the same dir as `nix-repl-server.nix`:

```nix
{
  lib,
  rustPlatform,
  nix,
  pkg-config,
  openssl,
  makeWrapper,
  inputs,
  src,
}:

rustPlatform.buildRustPackage {
  pname = "nix-repl-server";
  version = "0.1.0";

  # Point this to your actual source root (where Cargo.toml is)
  # src = ./.;
  inherit src;

  # You must commit Cargo.lock for this to work
  # cargoLock.lockFile = ../../../mdbook-nix-repl/server/Cargo.lock;
  cargoLock.lockFile = "${src}/Cargo.lock";

  postPatch = ''
    cp Cargo.toml.inc Cargo.toml
  '';

  # Runtime dependencies (nix for evaluation)
  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];
  buildInputs = [ openssl ];

  doCheck = false;

  # Ensure 'nix' is available in the path if your binary calls Command::new("nix")
  postInstall = ''
    wrapProgram $out/bin/nix-repl-server --prefix PATH : ${lib.makeBinPath [ nix ]}
  '';

  meta = with lib; {
    description = "Secure Nix REPL server for mdbook-nix-repl";
    platforms = platforms.linux;
  };
}
```

3. `flake.nix`, this URL leads to a Rust crate repo:

```nix
inputs = {
  mdbook-nix-repl = {
    url = "path:/home/jr/mdbook-nix-repl";
    flake = false;
  };
}
```

4. `configuration.nix`:

```nix
{ pkgs, inputs, ... }:
{
  imports = [
    # Include the results of the hardware scan.
    ./hardware-configuration.nix
    ./users.nix
    ./nix-repl-server.nix
  ];

  custom.nix-repl-server = {
    enable = true;
    port = 8080; # Optional, defaults to 8080
    tokenFile = "/etc/nix-repl-server.env";
  };
# --snip--
```

---

### Resources

- [RedHat A Practical Intro to Container Technology](https://developers.redhat.com/blog/2018/02/22/container-terminology-practical-introduction#)
