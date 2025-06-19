# Secure Boot with Lanzaboote

⚠️ **Warning: This can easily brick your system** ⚠️

We will mainly follow the lanzaboote
[Quick Start Guide](https://github.com/nix-community/lanzaboote/blob/master/docs/QUICK_START.md)

For Windows dual-booters and BitLocker users, you should export your BitLocker
recovery keys and confirm that they are correct. Refer to this
[Microsoft support article](https://support.microsoft.com/en-us/windows/find-your-bitlocker-recovery-key-6b71ad27-0b89-ea08-f143-056f5ab347d6)

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

## Why Use Lanzaboote (Secure Boot) on a Non-Encrypted System?

Although full disk encryption would provide the best protection it may be
unnecessary for your home desktop in your bedroom. Full disk encryption is
beyond the scope of this chapter.

Even if your disk is not encrypted, enabling Secure Boot with Lanzaboote brings
real security improvements:

1. Protects the Boot Process from Malware

Secure Boot ensures that only bootloaders and kernels signed with your trusted
keys can run at startup. This blocks bootkits and rootkits—dangerous types of
malware that try to infect your system before the operating system even loads .
Without Secure Boot, malicious software could silently replace your bootloader
or kernel and gain control every time your computer starts.

2. Prevents Unauthorized Modifications

If someone (or some software) tries to tamper with your boot files—like swapping
out your kernel or bootloader with a malicious version—Secure Boot will detect
this and refuse to start the system, alerting you that something is wrong

. This makes it much harder for attackers to hide or persist on your machine.

3. First Line of Defense

Secure Boot acts as a “gatekeeper” for your computer’s startup process. Even if
your files aren’t encrypted, it stops unauthorized code from running before
Linux loads, making it harder for malware to take hold and harder for attackers
to compromise your system at the lowest level.

4. Protects Recovery and Rescue Environments

Secure Boot also covers recovery partitions and rescue tools. Only signed,
trusted recovery environments can be loaded, preventing attackers from sneaking
in malicious tools during system repair.

5. Peace of Mind for Updates and Multi-User Systems

If you share your computer or use it in a public setting, Secure Boot ensures
that only approved system updates and kernels can be booted, reducing the risk
of accidental or intentional tampering.

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

This is where things can get tricky because BIOS are widely different and use
different conventions.

You can see your BIOS from the output of `bootctl status`:

```bash
sudo bootctl status
sudo bootctl status
doas (jr@magic) password:
System:
      Firmware: UEFI 2.70 (American Megatrends 5.19)
```

My BIOS is an American Megatrends 5.19, find yours and look up which key you
have to hit to enter the BIOS on reboot, mine is the delete key. So I reboot and
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
      Firmware: UEFI 2.70 (American Megatrends 5.19)
 Firmware Arch: x64
   Secure Boot: enabled (user)
  TPM2 Support: yes
  Measured UKI: yes
  Boot into FW: supported
```

We can see the `Secure Boot: enabled (user)`
