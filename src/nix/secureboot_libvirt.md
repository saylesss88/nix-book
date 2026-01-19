---
title: Secure Boot on Libvirt stack
date: 2026-01-19
author: saylesss88
collection: "blog"
tags: ["Virtual Machines", "KVM", "Impermanence", "Secure Boot"]
draft: false
---

# Secure Boot in a Libvirt (KVM) VM with ZFS on LUKS Impermanence

## Initial VM Setup

When creating the VM in virt-manager:

1. Before clicking "Finish", check the "Customize configuration before install"
   box

2. In the Overview section, change Firmware from BIOS to UEFI x86_64:
   `/usr/share/edk2/ovmf/OVMF_CODE_4M.secboot.qcow2`

3. Proceed with the NixOS installation as normal. For lanzaboote to build
   successfully, I had to pin it to nixpkgs `25.05`.

**Known Issue**: After running `nixos-install` and rebooting, the SATA CDROM
source path may be cleared. If the VM fails to boot, manually reselect the NixOS
ISO in the SATA settings and reboot. ​

---

## Configure Firmware for Custom Secure Boot Keys

The default configuration uses Microsoft's pre-enrolled keys, which won't trust
your custom-signed kernel. To enable custom key enrollment, you need to modify
the VM's XML configuration. ​

On your host, find your VM name:

```bash
virsh -c qemu:///system list --all
```

Example Output:

```bash
 Id   Name             State
---------------------------------
 -    nixos-unstable   shut off
 -    nixos            shut off
```

Edit the VM configuration:

```bash
virsh edit nixos-unstable
```

Make the following changes to the `<os>` section:

1. Change the `enrolled-keys` feature from `yes` to `no`:

```xml
<feature enabled='no' name='enrolled-keys'/>
```

2. Delete the explicit `<loader>` and `<nvram>` lines. These conflict with
   libvirt's firmware autoselection when using `enrolled-keys='no'`:

```xml
<!-- DELETE THESE TWO LINES -->
<loader readonly='yes' secure='yes' type='pflash' format='qcow2'>/usr/share/edk2/ovmf/OVMF_CODE_4M.secboot.qcow2</loader>
<nvram template='/usr/share/edk2/ovmf/OVMF_VARS_4M.secboot.qcow2' templateFormat='qcow2' format='qcow2'>/var/lib/libvirt/qemu/nvram/nixos-unstable_VARS.qcow2</nvram>
```

Your final `<os>` section should look like this:

```xml
<os firmware='efi'>
  <type arch='x86_64' machine='pc-q35-10.1'>hvm</type>
  <firmware>
    <feature enabled='no' name='enrolled-keys'/>
    <feature enabled='yes' name='secure-boot'/>
  </firmware>
  <boot dev='hd'/>
</os>
```

3. Add the `<serial>` tag as a best practice

```xml
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2' discard='unmap'/>
      <source file='/var/lib/libvirt/images/nixos-unstable-1.qcow2'/>
      <serial>disk01</serial>
      <target dev='vda' bus='virtio'/>
      <address type='pci' domain='0x0000' bus='0x04' slot='0x00' function='0x0'/>
    </disk>
```

- This enables commands like: `zpool import -d /dev/disk/by-id rpool`

Save and exit the editor. Libvirt will now automatically create a new NVRAM file
in Setup Mode (no keys enrolled).

---

## NixOS Installation with ZFS on root with LUKS

- [ZFS on root with LUKS & Impermanence](https://saylesss88.github.io/nix/encrypted_zfs.html)

Add the impermanence flake input. I also had to pin lanzaboote to nixpkgs
`25.05` for lanzaboote to build successfully:

```nix
inputs = {
   impermanence.url = "github:nix-community/impermanence";
   nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
   nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-25.05";
};
```

Add lanzaboote, if you don't use `unstable`, you can obviously avoid the
overlay:

```nix
{ pkgs, lib, inputs, ... }: {
# configuration.nix
nixpkgs.overlays = [
   (final: prev: {
      lanzaboote = (inputs.nixpkgs-stable.legacyPackages.${pkgs.system}.lanzaboote or prev.lanzaboote)
   })
];

environment.systemPackages = [ pkgs.sbctl ];

boot.loader.systemd-boot.enable = lib.mkForce false;

boot.lanzaboote = {
  enable = true;
  pkiBundle = "/var/lib/sbctl";
};
}
```

And `impermanence.nix`:

```nix
{ inputs, lib, ... }: {
   imports = [
      inputs.impermanence.nixosModules.impermanence
   ];
   boot.initrd.postMountCommands = lib.mkAfter ''
     zfs rollback -r rpool/local/root@blank
   '';
   environment.persistence."/persist" = {
      directories = [ "/var/lib/sbctl" "/var/lib/nixos" ];
   };
   fileSystems."/persist" = {
      device = "rpool/safe/persist";
      fsType = "zfs";
      neededForBoot = true;
   };
}
```

---

## Lanzaboote Installation

- [Secure Boot with Lanzaboote](https://saylesss88.github.io/installation/enc/lanzaboote.html)

---

## Enroll the Secure Boot Keys

After the XML changes, boot the VM and enter the firmware setup (press ESC
during boot).

1. Navigate to Device Manager → Secure Boot Configuration

2. Switch to "Custom mode", uncheck Attempt Secure Boot [ ], select "Reset
   Secure Boot Keys", save with F10, and reboot.

3. Enroll the keys: `sudo sbctl enroll-keys -m`

4. Reboot: After enrolling the keys and rebooting, the system will automatically
   be placed in "Standard mode", and Attempt Secure Boot [x] selected. You do
   not need to re-enter firmware setup mode.

5. Verify Secure Boot Status: `bootctl status`
