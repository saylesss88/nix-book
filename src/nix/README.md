---
title: Readme1
date: 2025-11-22
author: saylesss88
---

# Hardening README

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

📌 **How to Use This Guide**

**Read warnings**: Advanced hardening can break compatibility or cause data
loss! Pause and research before enabling anything not listed above unless you
understand the consequences.

**Hardening NixOS**:

🔷 Start Here:

- [Hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)

- [Hardening Networking](https://saylesss88.github.io/nix/hardening_networking.html)

---

**Additional security/hardening topics**

- [Browser/Browsing Security/Privacy](https://saylesss88.github.io/nix/browsing_security.html)

- [GnuPG gpg-agent](https://saylesss88.github.io/nix/gpg-agent.html)

- [Whonix KVM on NixOS](https://saylesss88.github.io/nix/whonix_kvm.html)

- [Running NixOS in a secureblue VM](https://saylesss88.github.io/nix/kvm.html)

## Getting Started

There is a lot covered in this guide which can get overwhelming when trying to
decide what is worth implementing. Here, I will list some common recommendations
that most users should follow to harden their stance.

> "The major problem with current systems is their inability to provide
> effective isolation between various programs running on one machine. E.g. if
> the user's Web browser gets compromised (due to a bug exploited by a malicious
> web site), the OS is usually unable to protect other user's applications and
> data from also being compromised."--Qubes arch-spec

## Threat Modeling

You should always start by conducting a personal threat assesment to identify
potential threats and vulnerabilities that you need to develop strategies to
defend against.

Threat modeling in computing involves evaluating the security risks to your
computer or network. It helps uncover possible threats and weaknesses so you can
create plans to safeguard your systems and data effectively. By examining
various attack scenarios, you can anticipate potential cyber threats and better
protect your digital resources.

It's not possible to protect yourself against every attack(er), focus on the
most probable threats to your specific situation.

- [EFF Security Starter Pack](https://ssd.eff.org/playlist/want-security-starter-pack)

- [EFF Your Security Plan](https://ssd.eff.org/module/your-security-plan)

- [Kicksecure Computer Security Threat Modeling](https://www.kicksecure.com/wiki/Threat_Modeling)

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

Secureblue recommends disabling Xwayland and finding alternatives for those apps
as well as disabling `xdg-desktop-portal-wlr`, this is because the wlroots
desktop portal reintroduces the screencopy vulnerability.

- Use Disk Encryption (LUKS) to protect your data at rest.

- Keep your system up to date (update regularly).

- Use strong, unique passwords. To generate one from the command-line, there is
  `pkgs.diceware`. Generate a password with: `diceware -n 12 -w en_eff`, add
  spaces between the words for higher entropy.
  - [Kicksecure Password_Generation](https://www.kicksecure.com/wiki/Passwords#Password_Generation)

- Avoid reusing passwords, use a password manager.

- Avoid storing files directly in the root home folder (i.e., `/home/user`),
  create sub-folders instead.(i.e., Instead of creating `~/notes.txt`, create
  `~/my-notes/notes.txt` or `~/Documents/notes.txt`).
  - If you are able to implement a Mandatory Access Control framework, there are
    more sub-folders that should be avoided such as `~/Downloads`. Another
    reason to use non-default sub-dirs is to avoid typos deleting important
    files.

  - Home-Manager has an option `xdg.userDirs.enable`

```nix
# home.nix or equivalent
{
  xdg.userDirs.enable = true;
  xdg.userDirs.createDirectories = true;
  # Optionally create non-default sub-dirs
  # xdg.userDirs.documents = "/home/jr/my-documents";
  # xdg.userDirs.download = "/home/jr/my-downloads";
}
```

- The XDG Base Directory Specification defines a consistent way for apps and
  desktops to store and find files. It helps prevent "dotfile clutter" by
  directing application files into clear, organized locations.

- Only enable what you use, and actively disable what's no longer in use.

- Enable at least a basic firewall, a more complex firewall example that
  utilizes nftables is shared in the
  [Hardening Networking Chapter](https://saylesss88.github.io/nix/hardening_networking.html)

Although the firewall is enabled by default on NixOS, let's be explicit about
it, add the following to your `configuration.nix` or equivalent:

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

Don't log in as `root`, it's unnecessary.

Commands that require `root` permissions should be run individually using `sudo`
in all cases. Avoid logging in as `root` & using `sudo su`.

Never run GUI applications as `root`. If there is a legitimate reason for doing
this, use `lxsudo` instead.

---

> NOTE: There is mention of making
> [userborn](https://github.com/nikstur/userborn) the default for NixOS in the
> future. It can be more secure by prohibiting UID/GID re-use and giving
> warnings about insecure password hashing schemes.

I have personally had nothing but problems with `userborn` and find the docs
extremely lacking, you need to read the source code to figure anything out which
is ridiculous. I don't personally use this but if you figure it out, more power
to ya.

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

---

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
