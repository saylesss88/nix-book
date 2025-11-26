---
title: Lanzaboote
date: 2025-11-22
author: saylesss88
---

# Secure Boot with Lanzaboote

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

⚠️ **Warning: This can easily brick your system** ⚠️

We will mainly follow the lanzaboote
[Quick Start Guide](https://github.com/nix-community/lanzaboote/blob/master/docs/QUICK_START.md)

For Windows dual-booters and BitLocker users, you should export your BitLocker
recovery keys and confirm that they are correct. Refer to this
[Microsoft support article](https://support.microsoft.com/en-us/windows/find-your-bitlocker-recovery-key-6b71ad27-0b89-ea08-f143-056f5ab347d6)

> ❗ NOTE: There are some serious limitations to this setup when used without
> encryption, I'd say it could stop the average person. But an experienced
> hacker could easily bypass this without encryption if they had access to your
> computer. For more protection look into TPM2 Hardware Requirements, and full
> disk encryption.

## Important Considerations

I found
[This Article](https://0pointer.net/blog/authenticated-boot-and-disk-encryption-on-linux.html)
fairly enlightening as far as the state of Authenticated Boot and Disk
Encryption on Linux.

- [Brave New Trusted Boot World](https://0pointer.net/blog/brave-new-trusted-boot-world.html)

Lanzaboote only secures the boot chain. The userspace remains unverified (i.e.,
the nix store, etc.), to verify userspace you need to implement additional
integrity checks. It's common to rely to disk encryption to prevent tampering
with and keep the Nix store safe but it's not always desirable. (i.e.,
unattended boot)

## Requirements

To be able to setup Secure Boot on your device, NixOS needs to be installed in
UEFI mode and systemd-boot must be used as a boot loader. This means if you wish
to install lanzaboote on a new machine, you need to follow the install
instruction for systemd-boot and then switch to lanzaboote after the first boot.

Check these prerequisits with `bootctl status`, this is an example output:

```bash
sudo bootctl status
System:
     Firmware: UEFI 2.70 (Lenovo 0.4720)
  Secure Boot: disabled (disabled)
 TPM2 Support: yes
 Boot into FW: supported

Current Boot Loader:
      Product: systemd-boot 251.7
...
```

The firmware **must** be `UEFI` and the current bootloader needs to be
`systemd-boot`. If you check these boxes, you're good to go.

## Security Requirements

To provide any security your system needs to defend against an attacker turning
UEFI Secure Boot off or being able to sign binaries with the keys we are going
to generate.

The easiest way to achieve this is to:

1. Enable a BIOS password for your system, this will prevent someone from just
   shutting off secure boot.

2. Use full disk encryption.

## Preparation

**Finding the UEFI System Partition (ESP)**

The UEFI boot process revolves around the ESP, the (U)EFI System Partition. This
partition is conventionally mounted at `/boot` on NixOS.

Verify this with the command `sudo bootctl status`. Look for `ESP:`

**Creating Your Keys**

First you'll need to install `sbctl` which is available in `Nixpkgs`:

```nix
# configuration.nix or equivalent
environment.systemPackages = [ pkgs.sbctl ];
```

Create the keys:

```bash
$ sudo sbctl create-keys
[sudo] password for julian:
Created Owner UUID 8ec4b2c3-dc7f-4362-b9a3-0cc17e5a34cd
Creating secure boot keys...✓
Secure boot keys created!
```

If you already have keys in `/etc/secureboot` migrate these to `/var/lib/sbctl`:

```bash
sbctl setup --migrate
```

## Configuring Lanzaboote With Flakes

Shown all in `flake.nix` for brevity. Can easily be split up into a `boot.nix`,
etc:

```nix
{
  description = "A SecureBoot-enabled NixOS configurations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    lanzaboote = {
      url = "github:nix-community/lanzaboote/v0.4.2";

      # Optional but recommended to limit the size of your system closure.
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, lanzaboote, ...}: {
    nixosConfigurations = {
      yourHost = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";

        modules = [
          # This is not a complete NixOS configuration and you need to reference
          # your normal configuration here.

          lanzaboote.nixosModules.lanzaboote

          ({ pkgs, lib, ... }: {

            environment.systemPackages = [
              # For debugging and troubleshooting Secure Boot.
              pkgs.sbctl
            ];

            # Lanzaboote currently replaces the systemd-boot module.
            # This setting is usually set to true in configuration.nix
            # generated at installation time. So we force it to false
            # for now.
            boot.loader.systemd-boot.enable = lib.mkForce false;

            boot.lanzaboote = {
              enable = true;
              pkiBundle = "/var/lib/sbctl";
            };
          })
        ];
      };
    };
  };
}
```

**Build it**

```bash
sudo nixos-rebuild switch --flake /path/to/flake
```

### Ensure Your Machine is Ready for Secure Boot enforcement

```bash
$ sudo sbctl verify
Verifying file database and EFI images in /boot...
✓ /boot/EFI/BOOT/BOOTX64.EFI is signed
✓ /boot/EFI/Linux/nixos-generation-355.efi is signed
✓ /boot/EFI/Linux/nixos-generation-356.efi is signed
✗ /boot/EFI/nixos/0n01vj3mq06pc31i2yhxndvhv4kwl2vp-linux-6.1.3-bzImage.efi is not signed
✓ /boot/EFI/systemd/systemd-bootx64.efi is signed
```

### Enabling Secure Boot and Entering Setup Mode

This is where things can get tricky because UEFI/BIOS are widely different and
use different conventions.

You can see your BIOS from the output of `bootctl status`:

```bash
sudo bootctl status
doas (jr@magic) password:
System:
      Firmware: UEFI 2.70 (American Megatrends)
```

My UEFI is an American Megatrends, find yours and look up which key you have to
hit to enter the BIOS on reboot, mine is the delete key. So I reboot and
repeatedly hit delete until it brings up the BIOS settings.

The lanzaboote guide shows a few systems and how to enter setup mode for them.

For a ThinkPad the steps are:

1. Select the "Security" tab.

2. Select the "Secure Boot" entry.

3. Set "Secure Boot" to enabled.

4. Select "Reset to Setup Mode".

---

For my system, it would allow me to do the above steps but when I saved and
exited I got a red screen then blue screen and it said No Valid Keys or
something like that and eventually brought me to the MOK Manager where you can
manually register keys, this is NOT what you want to do.

Even after this mistake I was able to re-enable secure boot and get back into
the system.

After some tinkering, I found that I was able to enter "custom mode" without
enabling secure boot, which in turn allowed me to select the "Reset to Setup
Mode"

It asks if you are sure you want to erase all of the variables to enter setup
mode? Hit "Yes". Then it asks if you want to exit without saving, we want to
save our changes so hit "No" do not exit without saving.

After this you should see all No Keys entries.

Finally, Hit the setting to save and exit, some BIOS list an F4 or F9 keybind
that saves and exits.

> ❗: For my system, choosing "save and reboot" would not work for some reason,
> I had to choose "save and exit".

After hitting "save and exit", the system boots into NixOS like normal but you
are in setup mode if everything worked correctly.

Open a terminal and type:

```bash
sudo sbctl enroll-keys --microsoft
Enrolling keys to EFI variables...
With vendor keys from microsoft...✓
Enrolled keys to the EFI variables!
```

> ⚠️ If you used `--microsoft` while enrolling the keys, you might want to check
> that the Secure Boot Forbidden Signature Database (dbx) is not empty. A quick
> and dirty way is by checking the file size of
> `/sys/firmware/efi/efivars/dbx-\*`. Keeping an up to date dbx reduces Secure
> Boot bypasses, see for example:
> <https://uefi.org/sites/default/files/resources/dbx_release_info.pdf>

I then Rebooted into BIOS and enabled secure boot, saved and exited. This loads
NixOS as if you just rebooted.

And finally check the output of `sbctl status`:

```bash
sudo sbctl status
System:
      Firmware: UEFI 2.70 (American Megatrends)
 Firmware Arch: x64
   Secure Boot: enabled (user)
  TPM2 Support: yes
  Measured UKI: yes
  Boot into FW: supported
```

We can see the `Secure Boot: enabled (user)`

## What Lanzaboote (Secure Boot) Actually Secures on NixOS and Limitations

As mentioned earlier, this provides some basic protection that may be good
enough for your desktop in your bedroom but there are some serious limitations.
I want to be clear that this may stop an average person but an advanced threat
actor with resources could still fairly easily get in.

Secure Boot (with Lanzaboote or any other tool) on NixOS primarily protects the
boot chain—the bootloader, kernel, and initrd—by ensuring only signed, trusted
binaries are executed at boot. This is a real and valuable security improvement,
especially for defending against “evil maid” attacks (where someone with
physical access tampers with your bootloader or kernel) and for preventing many
forms of persistent malware.

Here are some of the caveats:

1. Userspace Remains Unverified

   Once the kernel and initrd have booted, NixOS (by default) does not
   cryptographically verify the integrity of the rest of userspace (the programs
   and libraries in the Nix store, your configs, etc.).

   This means an attacker who can modify userspace (e.g., by gaining root
   access) can potentially install persistent malware, even if your boot chain
   is protected

   .

2. Kernel Lockdown Is Not Enabled

   The Linux kernel’s [lockdown mode]

   is designed to prevent even root from tampering with the kernel at runtime
   (e.g., by loading unsigned modules, using kexec, or accessing /dev/mem).

   NixOS does not enable kernel lockdown by default, and enabling it is
   non-trivial, especially given how the Nix store works (modules and kernels
   are built dynamically and not always signed at install time).

   Without lockdown, a root user (or malware with root) can still compromise the
   kernel after boot.

3. Stage 2 Verification Is Lacking

   Some distributions (like Fedora Silverblue or systems using dm-verity)
   cryptographically verify the entire userspace at boot, making it immutable
   and much harder to tamper with. This is not the default on NixOS, though
   there are experimental or appliance-focused solutions

   .

4. Disk Encryption Complements Secure Boot

   Full disk encryption (e.g., LUKS) is strongly recommended alongside Secure
   Boot. Encryption protects your data at rest and ensures that even if someone
   bypasses Secure Boot, they cannot read or modify your files without your
   passphrase
