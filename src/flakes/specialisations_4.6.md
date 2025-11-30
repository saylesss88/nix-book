---
title: Specialisations
date: 2025-11-22
author: saylesss88
collection: blog
tags: ["nixos", "specialisations"]
draft: true
---

# NixOS Specialisations For Multiple Profiles

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

**NixOS specialisations** are a powerful feature that allow you to define
alternative system configurations variations within a single NixOS setup. Each
specialisation can modify or extend the base configuration, and NixOS will
generate separate boot entries for each, letting you choose at boot time (or
switch at runtime) which environment to use. This is ideal for testing,
hardware-specific tweaks, or separating work and personal environments without
maintaining multiple configuration files

## How Specialisations Work

Specialisations are defined as attributes under the `specialisation` option in
your configuration. Each key (e.g., `niri-test`) represents a named
specialisation, and its configuration attribute contains the NixOS options to
apply on top of the base system

By default, a specialisation inherits the parent configuration and applies its
changes on top. You can also set `inheritParentConfig = false;` to create a
completely separate configuration.

After running `nixos-rebuild boot`, your bootloader will present extra entries
for each specialisation. Selecting one boots into the system with that
specialisation's settings applied

Runtime Switching: You can switch to a specialisation at runtime using
activation scripts, e.g.:

```bash
nixos-rebuild switch --specialisation niri-test
```

or

```bash
/run/current-system/specialisation/niri-test/bin/switch-to-configuration switch
```

> Note: Some changes (like kernel switches) require a reboot to take effect

Example: Let's create a basic specialisation to try out the Niri Window Manager:

First we have to add the `niri-flake` as an input to our `flake.nix` and add the
module to install it:

```nix
# flake.nix
inputs = {
     niri.url = "github:sodiboo/niri-flake";
};
```

```nix
# configuration.nix
{ pkgs, inputs, ... }: {
# ... snip ...
imports = [
    inputs.niri.nixosModules.niri
];

# This is the top-level overlay
  nixpkgs.overlays = [inputs.niri.overlays.niri];

# ... snip ...

  specialisation = {
    niri-test.configuration = {
      system.nixos.tags = ["niri"];

      # Add the Niri overlay for this specialisation
      nixpkgs.overlays = [inputs.niri.overlays.niri];

      # Enable Niri session
      programs.niri = {
        enable = true;
        package = pkgs.niri-unstable;
      };

      # Optionally, add a test user and greetd for login
      users.users.niri = {
        isNormalUser = true;
        extraGroups = ["networkmanager" "video" "wheel"];
        initialPassword = "test"; # for testing only!
        createHome = true;
      };

      services.greetd = {
        enable = true;
        settings = rec {
          initial_session = {
            command = lib.mkForce "${pkgs.niri}/bin/niri";
            user = lib.mkForce "niri";
          };
          default_session = initial_session;
        };
      };

      environment.etc."niri/config.kdl".text = ''
        binds {
          Mod+T { spawn "alacritty"; }
          Mod+D { spawn "fuzzel"; }
          Mod+Q { close-window; }
          Mod+Shift+Q { exit; }
        }
      '';
      environment.systemPackages = with pkgs; [
        alacritty
        waybar
        fuzzel
        mako
        firefox
      ];

      programs.firefox.enable = true;

      services.pipewire = {
        enable = true;
        alsa.enable = true;
        pulse.enable = true;
        # Optionally:
        jack.enable = true;
      };

      hardware.alsa.enablePersistence = true;

      networking.networkmanager.enable = true;
    };
  };
}
```

I chose to use the nightly version so it was required to add the overlay at the
top-level as well as inside the `specialisation` block.

On my system it sped up build times to first run:

```bash
sudo nixos-rebuild switch --flake .
# And Then Run
sudo nixos-rebuild boot --flake .
```

**What this does**:

- Creates a boot entry called `niri-test` with the Niri Wayland compositor, a
  test user, and a `greetd` login manager.

- Installs a set of packages and enables PipeWire with ALSA, PulseAudio, and
  JACK support.

- Provides a custom Niri configuration file for a few keybinds and enables
  NetworkManager.

## Using Your Specialisation After Boot

Once you have rebooted and selected your specialisation from the boot menu, you
can use your system as usual. If you want to add or remove programs, change
settings, or update your environment within a specialisation, simply:

1.  Edit your configuration: Add or remove packages (e.g., add `ghostty` to
    `environment.systemPackages`) or change any other options inside the
    relevant `specialisation` block in your NixOS configuration.

2.  Apply changes with a rebuild: Run the standard NixOS rebuild command. If you
    are currently running the specialisation you want to update, use:

```bash
sudo nixos-rebuild switch
```

This will apply your changes to the current specialisation

If you want to build and activate a different specialisation from your current
session, use:

```bash
sudo nixos-rebuild switch --specialisation name
```

Or, you can activate a specialisation directly with:

```bash
sudo /run/current-system/specialisation/<name>/bin/switch-to-configuration switch
```

Replace `<name>` with your specialisation’s name.

Reboot if needed: Most changes apply immediately, but some (like kernel or
`initrd` changes) require a reboot for the specialisation to fully take effect

**Tip**:

Each specialisation can have its own set of installed programs. Only those
listed in the `environment.systemPackages` (or enabled via modules) inside the
`specialisation` block will be available when you boot into that context.

You manage and update your specialisation just like your main NixOS system no
special commands or workflow are required beyond specifying the specialisation
when rebuilding or switching.

## Use Cases for Specialisations

- **Hardware Profiles**: Enable/disable drivers or services for specific
  hardware (e.g., eGPU, WiFi, or SR-IOV setups)

- **Desktop Environments**: Quickly switch between different desktop
  environments or compositors (e.g., GNOME, Plasma, Niri)

- **Testing**: Safely try out unstable packages, new kernels, or experimental
  features without risking your main environment

- **User Separation**: Create profiles for different users, each with their own
  settings, packages, and auto-login

- **Secure Environments**: Combine with encrypted partitions for more secure,
  isolated setups

## Securely Separated Contexts with NixOS Specialisations

I will just explain the concept here for completeness, if you want to implement
this I recommend following:

[Tweag Hard User Separation with NixOS](https://www.tweag.io/blog/2022-11-01-hard-user-separation-with-nixos/)

<details>
<summary> ✔️ Click To Expand Section on Separate Contexts </summary>

If you use the same computer in different contexts such as for work and for your
private life you may worry about the risks of mixing sensitive environments. For
example, a cryptolocker received through a compromised work email could
potentially encrypt your personal files, including irreplaceable family photos.

A common solution is to install two different operating systems and dual-boot
between them, keeping work and personal data isolated. However, this approach
means you have two systems to maintain, update, and configure, which can be a
significant hassle.

NixOS offers a third alternative: With NixOS specialisations, you can manage two
(or more) securely separated contexts within a single operating system. At boot
time, you select which context you want to use work or personal. Each context
can have its own encrypted root partition, user accounts, and configuration, but
both share the same Nix store for packages. This means:

- No duplicated packages: Both contexts use the same system-wide package store,
  saving space and simplifying updates.

- Single system to maintain: You update and manage only one NixOS installation,
  not two.

- Strong security boundaries: Each context can have its own encrypted root, so a
  compromise in one context (such as malware in your work environment) cannot
  access the data in the other context.

- Flexible management: You can configure both contexts from either environment,
  making administration easier.

This approach combines the security of dual-booting with the convenience and
efficiency of a single, unified system.

**How It Works**:

- Encrypted Partitions: Each context (work and personal) has its own encrypted
  root partition. The shared /nix/store partition is also encrypted, but can be
  unlocked by either context.

- Specialisations at Boot: NixOS generates multiple boot entries, one for each
  context. You simply choose your desired environment at boot time.

- Separation of Data: Your work and personal home directories, settings, and
  documents remain isolated from each other, while still benefiting from shared
  system packages.

Benefits Over Traditional Dual-Boot

- Only one system to update and configure.

- No wasted disk space on duplicate packages.

- Seamless switching between contexts with a reboot.

- Consistent NixOS tooling and workflows in both environments.

What You Need

- A physical or virtual machine supported by NixOS.

- Willingness to erase the system disk during setup.

- LVM (Logical Volume Manager) support: This setup requires using LVM for disk
  partitioning and management. LVM allows you to create multiple logical volumes
  on a single physical disk, making it possible to securely separate your work
  and personal environments while sharing a common Nix store. You will use LVM
  commands such as `pvcreate`, `vgcreate`, and `lvcreate` to prepare your disk
  layout

In summary: With NixOS specialisations and careful disk partitioning, you can
achieve secure, convenient, and efficient context separation—no need to
compromise between security and manageability.

</details>

### Tips and Best Practices

- Overriding Values: Use `lib.mkDefault` or `lib.mkForce` to make options
  overridable or forced in specialisations. I had to do it above because I have
  greetd setup for my main configuration as well.

- Selective Configuration: If you want certain options only in the default
  (non-specialised) system, use:

```nix
config = lib.mkIf (config.specialisation != {}) { ... }
```

- This condition checks if you're in a specialisation.

- Any settings inside this block will **not** be inherited by specialisations,
  keeping them exclusive to the main system.

- Runtime Limitations: Not all changes (e.g., kernel or `initrd`) can be fully
  applied at runtime; a reboot is required for those.

- Modularity: Specialisations work well with modular NixOS configs keep
  hardware, user, and service configs in separate files for easier management

References to Official Documentation and Community Resources

- [Tweag: Introduction to NixOS specialisations](https://www.tweag.io/blog/2022-08-18-nixos-specialisations/)

- [NixOS Wiki: Specialisation](https://wiki.nixos.org/wiki/Specialisation)

- [Tweag Hard User Separation with NixOS](https://www.tweag.io/blog/2022-11-01-hard-user-separation-with-nixos/)
