# Hardening README

📌 **How to Use This Guide**

**Read warnings**: Advanced hardening can break compatibility or cause data
loss! Pause and research before enabling anything not listed above unless you
understand the consequences.

The guide is broken up into 2 chapters:

- [Hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)

- [Hardening Networking](https://saylesss88.github.io/nix/hardening_networking.html)

## Getting Started

There is a lot covered in this guide which can get overwhelming when trying to
decide what is worth implementing. Here, I will list some common recommendations
that most users should follow to harden their stance.

### Baseline Hardening

Before diving into advanced or specialized hardening, apply these baseline
security measures suitable for all NixOS users. These settings help protect your
system with minimal risk of breaking workflows or causing admin headaches.

There is something to be said about the window manager you use. GNOME, KDE
Plasma, and Sway secure privileged Wayland protocols like screencopy. This means
that on environments outside of GNOME, KDE, and Sway, applications can access
screen content of the entire desktop. This implicitly includes the content of
other applications. It's primarily for this reason that Silverblue, Kinoite, and
Sericea images are recommended. COSMIC has plans to fix this.
--[secureblue Images](https://secureblue.dev/images)

- Use Disk Encryption (LUKS) to protect your data at rest.

- Keep your system up to date (update regularly).

- Use strong, unique passwords.

- Avoid reusing passwords, use a password manager.

- Only enable what you use, and actively disable what's no longer in use.

- Enable at least a basic firewall, a more complex firewall example that
  utilizes nftables is shared in the
  [Hardening Networking Chapter](https://saylesss88.github.io/nix/hardening_networking.html)

The firewall is enabled by default on NixOS. To explicitly ensure it's enabled,
add the following to your `configuration.nix` or equivalent:

```nix
# configuration.nix
# this denies incoming connections but allows outgoing and established connections
networking.firewall.enable = true;
```

Many services provide an option to open the required firewall ports
automatically. For example:

```nix
services.tor.openFirewall = true;
```

This prevents you from having to manually open ports

**Audit and remove local user accounts that are no longer needed**: Regularly
review and remove unused or outdated accounts to reduce your system’s attack
surface, improve compliance, and ensure only authorized users have access. The
following setting ensures that user (and group) management is fully declarative:

```nix
# configuration.nix
# All users must be declared
users.mutableUsers = false;
```

With `users.mutableUsers = false;`, all non-declaratively managed (imperative)
user management including creation, modification, or password changes will fail
or be reset on rebuild. User and group definitions become entirely controlled by
your system configuration for maximum reproducibility and security. If you need
to add, remove, or modify users, you must do so in your `configuration.nix` and
rebuild the system.

Create an admin user for administrative tasks and remove your daily user from
the `wheel` group:

```users.nix
{ config, pkgs, lib }:
{
users.users.admin = {
    isNormalUser = true;
    description  = "System administrator";
    extraGroups  = [ "wheel" "libvirtd" ];   # wheel = sudo, libvirtd for VMs
    initialPassword = "changeme";           # change with `passwd admin` later
    openssh.authorizedKeys.keys = [
      # (optional) paste your SSH public key here
      # "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
    ];
  };

  # --------------------------------------------------------------------
  # 2. Existing daily user – remove from wheel, keep everything else
  # --------------------------------------------------------------------
  users.users.daily = {
    isNormalUser = true;
    description  = "Daily driver account";
    extraGroups  = lib.mkForce [ "networkmanager" "audio" "video" ]; # keep useful groups
    # Remove `wheel` by *not* listing it (mkForce overrides any default)
  };
}
```

---

> NOTE: There is mention of making
> [userborn](https://github.com/nikstur/userborn) the default for NixOS in the
> future. It can be more secure by prohibiting UID/GID re-use and giving
> warnings about insecure password hashing schemes.

To enable `userborn`, just add the following to your `configuration.nix` or
equivalent:

```nix
# users.nix
{pkgs,...}:{
services.userborn = {
    enable = true;
    # Only needed if `/etc` is immutable
    # passwordFilesLocation = "/var/lib/nixos/userborn"
};
    users.users = {
       "newuser" = {
         homeMode = "755";
         uid = 1000;
         isNormalUser = true;
         description = "New user account";
         extraGroups = [ "networkmanager" "wheel" "libvirtd" ];
         shell = pkgs.bash;
         ignoreShellProgramCheck = true;
         packages = with pkgs; [];
       };
    };
    }
```

With `userborn`, you configure your users as you normally would declaratively
with NixOS with `users.users`, change `"newuser"` to your desired username.

Explicitly setting `uid = 1000;` is a best practice for compatibility and
predictability.

You can also specify which users or groups are allowed to do anything with the
Nix daemon and Nix package manager. The following setting will only allow
members of the `wheel` group access to commands that require elevated
privileges, such as installing or modifying system-wide packages:

```nix
# configuration.nix
{ ... }:
{
  nix.settings.allowed-users = [ "@wheel" ];
}
```

**OR** Only allow the `root` user:

```nix
# configuration.nix
{ ... }:
{
  nix.settings.allowed-users = [ "root" ];
}
```

This is more restrictive and much less convenient, think twice before going this
restrictive.

**Only install, enable, and run what is needed**: Disable or uninstall
unnecessary software and services to minimize potential vulnerabilities. Take
advantage of NixOS’s easy package management and minimalism to keep your system
lean and secure.

**Avoid permanently installing temporary tools**: Use tools like `nix-shell`,
`comma`, `devShells` and `nix-direnv` to test or run software temporarily. This
prevents clutter and reduces potential risks from unused software lingering on
the system.

**Update regularly**: Keep your system and software up to date to receive the
latest security patches. Delaying updates leaves known vulnerabilities open to
exploitation.

**Apply the Principle of Least Privilege**: Never run tools or services as root
unless absolutely necessary. Create dedicated users and groups with the minimum
required permissions to limit potential damage if compromised.

**Use strong passwords and passphrases**: Aim for at least 14–16 characters by
combining several unrelated words, symbols, and numbers. For example:
`sunset-CoffeeHorse$guitar!`. Strong passphrases are both memorable and secure.

**Use a password manager and enable multi-factor authentication (MFA)**: Manage
unique, strong passwords effectively with a trusted manager and protect accounts
with MFA wherever possible for a second layer of defense.

**Check logs regularly**: Reviewing your system logs helps you spot unusual
activity, errors, or failed login attempts that could indicate a security
problem. NixOS uses `journald` by default, which makes this easy. For example,
to see the logs for your current boot session:

```bash
journalctl -b
# for the previous session
journalctl -b -1
```

After establishing some standard best practices and a hardened base, it’s time
to dive deeper into system hardening, the process of adding layered safeguards
throughout your NixOS setup. This next section guides you through concrete steps
and options for hardening critical areas of your system: from encryption and
secure boot to managing secrets, tightening kernel security, and leveraging
platform-specific tools.
[Hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)
