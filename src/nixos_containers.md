# NixOS Containers

NixOS containers are lightweight `systemd-nspawn` containers managed
declaratively through your NixOS configuration. They allow you to run separate,
minimal NixOS instances on the same machine, each with its own services,
packages, and (optionally) network stack.

**Common Use Cases**

- **Isolating services**: Run a web server, database, or any service in its own
  container, so it can’t interfere with the main system or other services

- **Testing and development**: Try out new configurations, packages, or services
  in a sandboxed environment.

- **Reproducible deployments**: Because containers are defined declaratively,
  you can reproduce the exact same environment anywhere.

- **Running multiple versions of a service**: For example, testing different
  versions of Git or HTTP servers side by side.

## Hosting an mdBook or Offline Git Server

Let’s say you want to host your mdBook or run a Git server for offline use. You
can define a NixOS container that runs only the necessary service, isolated from
your main system:

```nix
containers.mdbook = {
  ephemeral = true;         # Container resets on restart (optional)
  autoStart = true;         # Starts automatically at boot
  config = { config, pkgs, ... }: {
    # Example: Serve static files with httpd (Apache)
    services.httpd.enable = true;
    services.httpd.adminAddr = "you@example.org";
    networking.firewall.allowedTCPPorts = [ 80 ];
    # You could also use nginx, or run the mdbook server directly
    # Or, for a git server:
    # services.gitDaemon.enable = true;
  };
};
```

- `ephemeral`: if true, the container resets on each restart.

- `autoStart`: Ensures the container starts automatically at boot.

- `config`: Defines the containers NixOS configuration, just like a regular
  NixOS system.
