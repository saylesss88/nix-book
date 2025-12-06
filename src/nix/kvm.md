---
title: KVM
date: 2025-12-06
author: saylesss88
---

# Running NixOS in a VM with Maximum Isolation (Beginner Guide)

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![sp5](images/steampunk5.cleaned.png)

## Why This Setup?

- **Host** `secureblue` = Fedora Atomic with **SELinux enforcing**, **sVirt**,
  **Secure Boot**, and hardened defaults.

- **Guest**: NixOS in a VM → full declarative power, near zero risk to host.

- **Isolation**: Mandatory Access Control (MAC) via SELinux + KVM + no direct
  hardware access.

### My Experience with Secureblue

<details>
<summary> ✔️ My Experience with Secureblue </summary>

I have been using secureblue as the host OS while running NixOS in a VM, with no
noticeable performance issues for my workload. When problems do occur, they have
all been fixable via rollbacks or small configuration changes, so there has been
no need to “nuke and reinstall.”

**Software Installation and Management**

Flatpak takes some adjustment, especially if you are used to running editors or
other tools with full root access on a traditional Linux setup. Tools like
Flatseal make it easy to inspect exactly which permissions each application has
by default, and then gradually tighten them as you learn what is actually
required for the app to keep working. On secureblue, you can also apply a very
strict default with ujust flatpak-permissions-lockdown, which revokes almost
everything so that you must explicitly grant each permission you want an
application to have.

To be honest, I have one editor that's setup with flatpak and one that is
installed with `rpm-ostree` for the tight integration and default full root
behavior that is much less secure. I ended up moving from flatpak yazi to
`rpm-ostree` as well because in order to get it to function the way I wanted, I
had to bypass much of the sandboxing anyways.

If you use toolbx, it can be helpful to also install brew and flatpak within the
toolbx so you can share more config files. Silverblue recommends using flatpak
for most GUI apps and creating a toolbox for all of your cli apps, secureblue
includes homebrew which makes installing cli apps outside of a container easy.

On secureblue `/home` is just a symlink to `/var/home`, and that leaks into some
development workflows in non‑obvious ways. Most tools work without issue, but
some get confused by the symlink. If this happens, pointing the tool at
`/var/home/username/` instead of `/home/username/` often fixes the issue.

Most people will not need to add any extra GPU or display drivers inside the
NixOS VM when running on secureblue. The host already provides the hardware
stack, and adding additional drivers in the guest can actually make things less
stable instead of better.

For example, when extra drivers or compositor tweaks are added in the VM, it can
cause flashing, freezes, or generally janky graphics because you are effectively
fighting the host’s configuration. Keeping the VM’s graphics setup simple and
close to the defaults avoids a whole class of hard‑to‑debug issues and usually
gives smoother performance.

</details>

---

### 🔑 Key Terms

> NOTE: Secureblue enables the `hardened_malloc` by default which causes
> problems for many browsers and will cause screen flashing with Firefox and
> others within the VM. See:

- [secureblue standard_malloc](https://secureblue.dev/faq#standard-malloc)

## Step 1: Install secureblue (Hardened Host)

1. Download a [secureblue image](https://secureblue.dev/install)

2. Use **Fedora Media Writer** (Flatpak):

```bash
flatpak install flathub org.fedoraproject.MediaWriter
```

3. Flash the secureblue image & enable Secure Boot in UEFI **before** install.
   This is now possible with Fedora, when you boot into Fedora Media Writer (not
   Ventoy or Rufus), you will be allowed to enroll the secure boot key with
   secure boot pre-enabled.

4. On first boot:

```bash
ujust enroll-secureblue-secure-boot-key
```

- Reboot -> Enroll key in MOK manager with password: `secureblue`

5. Post-install hardening See:
   [post-install](https://secureblue.dev/post-install)

6. Install virtualization stack:

```bash
ujust install-libvirt-packages
```

- The above command enables `qemu`, `libvirt`, & `virt-manager` with SELinux
  labels.

- Read the [secureblue FAQ](https://secureblue.dev/faq) to learn the quirks of
  an atomic fedora image.

Secureblue recommends installing GUI apps with Flatpak, CLI apps with homebrew,
and apps that require more system access to be layered with rpm-ostree. It takes
some getting used to but is very stable.

- [secureblue how to install software](https://secureblue.dev/faq#software)

---

## Create NixOS VM (via virt-manager)

1. Download: [NixOS Graphical ISO](https://nixos.org/download/)

2. Open `virt-manager` -> File -> New Virtual Machine

- Select ISO

- CPU: `host-passthrough` (optional, for performance)

- Do some research to find the ideal Memory and Storage for your system.

3. Ensure SELinux is enabled (the default for secureblue) with: `getenforce`

4. Ensure sVirt is enabled (the default) with `run0 ps -eZ | grep qemu`.

```bash
run0 ps -eZ | grep qemu
# Output
system_u:system_r:svirt_t:s0:c383,c416 14793 ?   00:01:37 qemu-system-x86
```

5. Boot -> Follow graphical installer:

- Enable LUKS

- Create an admin user

- Optionally skip desktop -> install your own after first boot.

The attack surface is reduced significantly when running NixOS within a hardened
hosts VM. The VM operates on virtualized hardware, which is a powerful form of
attack surface reduction.

Devices like your host's Bluetooth adapter, Wi-Fi card, microphone, webcam, and
USB ports are not directly exposed to the guest operating system. The VM only
sees virtual versions of these devices. If an exploit targets a vulnerability in
the Bluetooth stack within the VM, it compromises the VM environment, but it
cannot typically reach and exploit the physical Bluetooth hardware on the host.

You can also choose not to pass through certain devices, like Bluetooth or
webcam to the VM at all, effectively disabling that attack vector. Since your
host likely already has these hardened features you may not need the additional
functionality within the VM.

If something breaks, you have an option to rollback to the previous generation
with `rpm-ostree rollback`. The previous generation will be applied on next
reboot. You can also just reboot and choose the previous generation through the
grub menu, this way it is temporary and will revert back on next reboot.

---

## 🔒 How Host MAC Secures the NixOS VM

The core security principle here is defense-in-depth, where the outer, hardened
layer (the host) compensates for potential weaknesses in the inner layer (the
guest).

1. MAC Confinement via SELinux and sVirt sVirt (Secure Virtualization): This is
   a critical component running on the secureblue host. It automatically assigns
   unique SELinux labels to all virtualization components.

**QEMU Process Confinement**: The entire QEMU process that runs the NixOS VM is
confined by a specific SELinux type, typically `svirt_t`. This means:

The host's MAC policy strictly controls what the QEMU process can access and do
on the host system.

If an attacker were to achieve a "VM breakout" (a worst-case scenario where they
escape the VM and try to interact with the host OS), their activity would still
be confined by the extremely strict rules of the `svirt_t` label. They would not
be able to arbitrarily read host files or compromise the host kernel.

**Disk Image Confinement**: The VM's disk images are also labeled, typically as
virt_image_t, preventing other processes on the host from accessing or tampering
with them.

2. **KVM and Host Hardening KVM**: KVM provides the low-level, hardware-assisted
   virtualization. It is an extremely secure and audited hypervisor that creates
   a strong barrier between the guest and the host kernel.

**Secureblue Hardening**: The secureblue host is designed with SELinux
enforcing, Secure Boot, a hardened kernel, and hardened_malloc by default, which
minimizes the attack surface and ensures the integrity of the base operating
system that's running the VM.

3. **Isolation and Zero Host Risk Decoupling Security**: The security of the
   host is completely decoupled from the security of the NixOS guest.

Any compromise within the NixOS VM (e.g., a service vulnerability,
misconfiguration, or user error) will be contained by the host's isolation
mechanisms (KVM + SELinux + sVirt). This containment means the host remains
secure ("Zero host compromise"), regardless of the NixOS VM's internal security
settings, including its lack of default MAC.

In short, the security boundary isn't the guest OS's (NixOS) configuration, but
the hypervisor and the host's MAC policy that enforces the complete isolation of
the VM

## It's still recommended to harden the Guest VM (NixOS)

Hardening the NixOS guest VM adds an extra, independent layer of defense,
helping to protect the system beyond what the host provides.

**Best Practices for Minimizing VM Device Exposure**

Take a VM snapshot right after a fresh install. This snapshot acts as a clean
restore point. Many people safely test malware or potentially dangerous software
by running it within the VM, then reverting to the snapshot afterward to wipe
out any changes or infections caused by the malware.

Avoid unnecessary device passthrough: Only pass through hardware devices (like
USB, GPU, or network interfaces) that are essential for your VM's operation. For
example, if a device isn't needed within the VM, do not passthrough the device
to reduce attack surface.​

Use virtual network segmentation: Instead of bridging physical network devices,
opt for virtual network configurations like isolated networks, VLANs, or
internal networks that prevent VM-to-VM or VM-to-host communication unless
explicitly allowed.​

Implement network filtering and firewall rules: Use libvirt nwfilter, iptables,
or firewalld rules to restrict communications between VMs and external networks,
or between guest VMs on the same host.​

- [libvirt Firewall and network filtering](https://libvirt.org/firewall.html)

Use virtual device models with minimal capabilities: Prefer virtio or similar
paravirtualized devices that have a smaller attack surface. Avoid emulated
devices when not necessary.​

Disable features like USB debugging, audio, or PnP devices: These can
potentially be exploited or leak information if enabled unnecessarily.

- It's still recommended to enable either the `graphene-hardened` or
  `graphene-hardened-light` memory allocators on the NixOS guest machine as
  well.

```nix
# configuration.nix
environment.memoryAllocator.provider = "graphene-hardened";
# OR for a more permissive and better performing allocator:
# environment.memoryAllocator.provider = "graphene-hardened-light";
```

- Remember that certain programs won't run with the `hardened_malloc`. I have
  read that you need to recompile Firefox for it to respect and work with the
  `hardened_malloc`. I haven't attempted this as of yet and use Brave for now.

Continue
[hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)

> ❗️ NOTE: It’s generally recommended not to enable GPU drivers inside the VM
> unless you are specifically doing GPU passthrough, as this often causes
> stability and compatibility issues. GPU passthrough itself requires careful
> configuration and dedicated hardware, and introduces additional attack
> surfaces.

> Regarding IPv6 networking, enabling it typically requires using a bridged
> network setup rather than NAT, which connects the VM more directly to the
> host's network. While bridged networking enables full IPv6 functionality, it
> also reduces the network isolation between the VM and host, potentially
> increasing security risks. For maximum isolation, consider carefully whether
> you need IPv6 connectivity inside the VM and weigh that against your security
> goals.

I have been able to recover from quite a few missteps with Secureblue. I run a
mini PC and attempted running `ujust update-firmware`, some systems allow you to
update the firmware of a booted system. On reboot I got a message "Something
went seriously wrong MOK is full", it then forced a shutdown. I was familiar
with resetting the NVRAM by disassembling the PC and moving the red jumper from
prongs 1 & 2 to prongs 2 & 3 with the power off for 10 seconds. I then moved the
jumper back to the default position and rebooted. The PC sounds like it's
revving up a few times and does a few reboots and allowed me to sign right back
in and re-enroll the secure boot key.

### Resources

- [RedHat What is virtualization?](https://www.redhat.com/en/topics/virtualization/what-is-virtualization)

- [virtualization & hypervisors](https://sumit-ghosh.com/posts/virtualization-hypervisors-explaining-qemu-kvm-libvirt/)

- [Virtualization on Linux using the KVM/QEMU/Libvirt stack](https://bitgrounds.tech/posts/kvm-qemu-libvirt-virtualization/)
