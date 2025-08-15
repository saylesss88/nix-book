# NixOS Containers

NixOS containers are lightweight `systemd-nspawn` containers managed
declaratively through your NixOS configuration. They allow you to run separate,
minimal NixOS instances on the same machine, each with its own services,
packages, and (optionally) network stack.

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

## Hosting mdbook

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

See if your container is active:

```bash
systemctl is-active container@mdbook-host.service
active
```

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
