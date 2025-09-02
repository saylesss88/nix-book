# Whonix KVM on NixOS

![Whonix Logo](../images/swappy-20250901-101339.cleaned.png)

> ⚠️ WARNING: There is no general software that can guarantee absolute anonymity
> or security; perfect security is a myth. Security is a continuous process, not
> a one-time product. It also depends on time and resources: if an adversary has
> enough of either, eventual compromise is possible. However, by layering
> defenses and following best practices, we can make attacks costly and
> time-consuming, deterring all but highly targeted adversaries.

Whonix can be useful for many things, including not having to shut down your
main OS to boot into a Tails USB stick and getting mostly the same protection.
Their docs are honest about the limitations of the software making it easier to
trust.

- [Whonix Compared to Tails](https://www.whonix.org/wiki/Comparison_with_Others)

## 🔑 Key Terms

Whonix is an operating system based on Debian base (Kicksecure Hardened) and the
Tor network, which is designed for maximum anonymity and security. Whonix
consists of two Debian based VMs, the `Whonix-Gateway` and `Whonix-Workstation`.

In this case NixOS is the **Host Operating System**, NixOS runs the KVM kernel
module, libvirtd service, and QEMU virtualization service which together enable
hosting VMs.

**Guests** are the virtualized operating systems running inside the host's
virtual machines. In this case the Whonix VMs are the **Guest Machines**.

`Whonix-Gateway` the first of 2 VMs runs Tor processes and forces all traffic
through the Tor network using iptables.

`Whonix-Workstation` the second VM, is responsible for running user applications
such as the Tor Browser. The Whonix-Workstation is isolated from both the
Whonix-Gateway and the Host OS, if an app misbehaves, it is contained within the
isolated Whonix-Workstation. It is largely unaware of sensitive info and won't
leak unless an advanced adversary is able to break out of the VM.

The primary goal of Whonix is to be safer than Tor alone and that no one can
find out the user's IP, location, or de-anonymize the user. It offers full
spectrum anti-tracking protection that is much safer than VPNs. Whonix provides
this through security by isolation, no app is trusted.

`Whonix Concept`: Whonix is an Isolating Proxy with an additional Transparent
Proxy, which can be optionally disabled. --Whonix Docs

Since Whonix is based on Kicksecure which is based on Debian stable, you can
typically look up solutions in an Ubuntu forum.

- The Whonix Team recommends KVM over VirtualBox for a number of
  reasons:[Why choose KVM over VirtualBox](https://www.whonix.org/wiki/KVM#Why_Use_KVM_Over_VirtualBox?)

If you really want to use VirtualBox, I got it working off of this config:

<details>
<summary> ✔️ Click to Expand VirtualBox Example </summary>

Change `your-user` to your username

```nix
# vbox.nix
{
  config,
  lib,
  ...
}: let
  cfg = config.custom.virtualbox;
in {
  options.custom.virtualbox = {
    enable = lib.mkEnableOption "Enable VirtualBox";
  };

  config = lib.mkIf cfg.enable {
    virtualisation.virtualbox.host = {
      enable = false;
      # enableExtensionPack = true;
    };

    user.user.your-user.extraGroups = ["vboxusers"];

    boot.kernelModules =
      if config.hardware.cpu.amd.updateMicrocode
      then ["kvm-amd"]
      else ["kvm-intel"];
  };
}
```

Enable it with `custom.virtualbox.enable = true;`.

- [Whonix VBox Download](https://www.whonix.org/wiki/VirtualBox)

After rebuilding with virtualbox enabled and downloading the virtualbox whonix,
open VirtualBox and import the Whonix file.

Fix the error:: VirtualBox can't enable the AMD-V extension. Please disable the
KVM kernel extension:

If both of these are active, they compete with each other:

```bash
sudo lsmod | grep -E 'kvm|vbox'
```

Check the currently in use modules:

```bash
modprobe -r kvm
```

Disable kvm and kvm_amd:

```bash
sudo rmmod kvm_amd
sudo rmmod kvm
# To re-enable them when necessary
# sudo modprobe kvm
# sudo modprobe kvm_amd
```

- [Quick fix](https://atetux.com/quick-fix-virtualbox-cant-enable-the-amd-v-extension)

There is an opposite viewpoint,
[Why choose VirtualBox over KVM](https://www.whonix.org/wiki/Dev/VirtualBox#Why_use_VirtualBox_over_KVM?)

</details>

## Whonix-Gateway

The whonix-gateway is software designed to run Tor.

The Gateway acts as a firewall and is what is routing all your traffic through
Tor.

You will spend minimal time in the Gateway, it's mainly used for Tor
configuration which is reserved for advanced users.

### Whonix-Workstation

All user applications should only be launched from Whonix-Workstation to ensure
they utilize the Tor network. (Never launch the Tor browser or any other user
app from Whonix-Gateway.)

Leaky applications can't breakout of the Workstation, all network connections
are forced to go through the Whonix-Gateway where they are torrified and routed
to the internet.

## Whonix KVM (Kernel Virtual Machine) on NixOS

**KVM** (Kernel-based Virtual Machine) is a Linux kernel module that provides
hardware-assisted virtualization.

It allows the Linux kernel to act as a hypervisor, enabling virtual machines
(VMs) to run with near-native speeds by using CPU virtualization extensions
(Intel VT-x or AMD-V).

KVM itself doesn’t handle the entire VM lifecycle; it provides the core
virtualization infrastructure.

**QEMU** (Quick Emulator) is an open-source user-space program that emulates
hardware for virtual machines.

When combined with KVM, QEMU uses hardware acceleration to run VMs much faster
by offloading CPU virtualization to KVM.

So, QEMU provides the device emulation and VM management interface, while KVM
provides the fast virtualization engine within the kernel.

**Install Qemu-KVM**:

```nix
{
  config,
  pkgs,
  ...
}: {
  ##  QEMU-KVM
  environment.systemPackages = with pkgs; [
    qemu
    # Optional
    virt-viewer
  ];

  # Virt-Manager GUI
  programs.virt-manager.enable = true;
  virtualisation = {
    # libvirtd daemon
    libvirtd = {
      enable = true;
      qemu = {
        # enables a TPM emulator
        swtpm.enable = true;
      };
    };
    # allow USB device to be forwarded
    spiceUSBRedirection.enable = true;
  };
  # Spice protocol improves VM display and input responsiveness
  services.spice-vdagentd.enable = true;
}
```

---

The **libvirtd** is the primary daemon (service) in the libvirt virtualization
management system. It runs on your host machine and acts as the core management
component for virtual machines (VMs).

Add `libvirtd` & `kvm` to your users `extraGroups`:

```nix
users.users = {
    your-user = {
        extraGroups = [
            "libvirtd"
            "kvm"
        ];
    };
};
```

Restart `libvirtd`:

```bash
sudo systemctl restart libvirtd
```

---

## Network Start

Ensure KVM's / QEMU's default network is enabled and has started:

```bash
sudo virsh -c qemu:///system net-autostart default
```

```bash
sudo virsh -c qemu:///system net-start default
```

---

### Download Whonix (KVM) (stable)

- [Whonix (KVM) (stable) Download](https://www.whonix.org/download/libvirt/17.4.4.6/Whonix-Xfce-17.4.4.6.Intel_AMD64.qcow2.libvirt.xz)

- Go to [whoniix.org](https://www.whonix.org/wiki/KVM) to verify the signature.

- [Decompress the Image](https://www.whonix.org/wiki/KVM#Decompress) and follow
  the rest of the Whonix KVM install instructions from there.

Nixpkgs doesn't have the `xz-utils` package but it does have the `xz` package.

```bash
tar -xvf Whonix*.libvirt.xz
```

---

### Import the Whonix VM Templates

- Follow steps 1 thru 3 in
  [Importing Whonix VM Templates](https://www.whonix.org/wiki/KVM#Importing_Whonix_VM_Templates)

After the above steps, either copy or move the `qcow2` images to
`/var/lib/libvirt/images`:

```bash
sudo mv Whonix-Gateway*.qcow2 /var/lib/libvirt/images/Whonix-Gateway.qcow2
```

```bash
sudo mv Whonix-Workstation*.qcow2 /var/lib/libvirt/images/Whonix-Workstation.qcow2
```

### Launch virt-manager and start the VMs

```bash
virt-manager
```

## Start Whonix-Gateway

![Whonix Logo](../images/swappy-20250901-101351.cleaned.png)

Always start the Whonix-Gateway first.

Click on Whonix-Gateway, press Play, and choose the default Persistent VM.

To view the gateway press `Open`.

You can use the "System Maintenance Panel" to `Check for Updates` and then
`Install Updates`. This can also be used for user and password creation, the
default user is `user` with a passwordless login.

Change the password manually:

```bash
sudo passwd
changeme
```

- [Whonix Common CLI Commands](https://www.whonix.org/wiki/Common_CLI_Commands)

## Whonix-Workstation

Whonix-Workstation is another VM, designed to provide users with a secure and
anonymous environment for running applications and performing online tasks.

When you first launch `Whonix-Workstation`, choose the second option down or
reboot, then choose "Persistent Mode Sysmaint Session".

With the workstation, a security feature disables sudo for the default user.
Instead of the `user` account, a separate `sysmaint` (system maintenance)
account is used for administrative tasks that require root privileges, such as
updates and package installations.

Once Workstation is running and both VMs are updated and upgraded, check that
your IP address is a Tor IP:

```bash
curl ip.me
```

Start Tor and check what you are fingerprinted as by typing `deviceinfo.me` into
the URL.

#### Launching Tor

Click the Xfce logo and choose Tor Browser. On the first launch, you will need
to update Tor by clicking in the top right corner.

Or you can open the terminal and type:

```bash
update-torbrowser
```

Make sure you don't forget to go to the Settings, Privacy and Security, and set
the `Security Level` to `Safest` to disable JavaScript and more before exploring
the dark web.

If you need a place to start, check out `tor.taxi` by plugging that into the
URL.

### Resources

- [Whonix Docs](https://www.whonix.org/wiki/Documentation)

- [Kicksecure Computer Security Intro](https://www.kicksecure.com/wiki/Computer_Security_Introduction)

- [Kicksecure Advanced Security Guide](https://www.kicksecure.com/wiki/Computer_Security_Introduction#Advanced_Security_Guide)

- [System Hardening Checklist](https://www.kicksecure.com/wiki/System_Hardening_Checklist)
