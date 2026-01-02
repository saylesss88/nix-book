---
title: KVM
date: 2025-12-06
author: saylesss88
collection: "blog"
tags: ["KVM", "Virtual Machines", "secureblue"]
draft: false
---

# Running NixOS in a VM with Maximum Isolation (Beginner Guide)

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![sp5](/images/steampunk5.cleaned.png)

## Why This Setup?

- **Host** `secureblue` = Fedora Atomic with **SELinux enforcing**, **sVirt**,
  **Secure Boot**, and hardened defaults.

- **Guest**: NixOS in a VM → full declarative power, near zero risk to host.

- **Isolation**: Mandatory Access Control (MAC) via SELinux + KVM + no direct
  hardware access.

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
and apps that require more system access to be layered with `rpm-ostree`. It
takes some getting used to but is very stable.

- [secureblue how to install software](https://secureblue.dev/faq#software)

---

## Create NixOS VM (via virt-manager)

Easiest way to get a working configuration IMO, I haven't personally had luck
with custom NixOS disk layouts within a VM.

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

The host uses a classic defense-in-depth model: the hardened outer layer (the
host) is treated as the real security boundary, and it is designed to remain
safe even if the inner layer (the NixOS guest) is fully compromised.

1. **MAC confinement with SELinux and sVirt**

On the secureblue host, sVirt automatically applies SELinux labels to all
VM-related processes and resources.

- **QEMU process confinement**: The QEMU process that runs the NixOS VM runs
  under a dedicated SELinux type, typically `svirt_t`. The host’s MAC policy
  tightly restricts what this process can access, so even a successful VM escape
  is still trapped inside a very limited sandbox rather than gaining normal host
  privileges.

- **Disk image protection**: VM disk images are labeled (for example,
  `virt_image_t`), which prevents unrelated host processes from reading or
  modifying them and keeps the VM’s storage isolated from the rest of the
  system.

2. **KVM and host hardening**

KVM provides the hardware-assisted virtualization layer and forms a strong
barrier between the guest and the host kernel. On top of that, the secureblue
host is hardened with SELinux in enforcing mode, Secure Boot, a hardened kernel,
and hardened_malloc by default. Together, these measures reduce the attack
surface and help ensure the integrity of the platform that is actually running
the VM.

3. **Isolation and “zero host compromise”**

The host and guest are deliberately decoupled from a security perspective. The
assumption is that the NixOS VM can be misconfigured, vulnerable, or even fully
compromised. If that happens, KVM plus the host MAC policy (SELinux + sVirt) are
responsible for containing the damage. In other words, the security boundary is
not the NixOS configuration inside the VM, but the hypervisor and the host’s
mandatory access control rules that enforce strict isolation of the guest from
the host.

## Hardening the NixOS Guest is Still Worth It

Even with a hardened host and MAC confinement, treating the NixOS VM as
“untrusted but hardened” adds another independent safety layer. The goal is to
minimize what an attacker can do inside the guest, even if they never manage a
breakout.

**Minimize VM device exposure**

- **Use snapshots aggressively**: Take a snapshot right after a fresh install
  and initial configuration. That snapshot becomes your “known-good” state for
  testing risky software or malware, so you can revert and wipe out any changes
  afterward.

- **Avoid unnecessary passthrough**: Only pass through hardware (USB, GPU,
  network interfaces, etc.) if the VM genuinely needs it. Every extra device is
  another potential attack surface.

- **Prefer simple virtual devices**: Use virtio and other paravirtualized
  devices where possible, and avoid legacy or fully emulated devices unless
  there is a specific need.

**Network isolation for guests**

- **Keep networks virtual and segmented**: Favor isolated virtual networks,
  VLANs, or internal-only networks over bridged physical interfaces, so VMs
  cannot talk to the host or each other unless you explicitly design for it.

- **Filter traffic tightly**: Use libvirt nwfilter, firewall rules
  (nftables/iptables/firewalld), and similar tools to restrict VM-to-VM and
  VM-to-external traffic, especially for services exposed on multiple guests.

- **Be cautious with IPv6**: Full IPv6 inside the VM usually implies bridged
  networking, which connects the VM more directly to the host’s LAN. That
  improves connectivity but reduces isolation, so enable it only if you truly
  need it.

**Guest-side hardening measures**

- **Harden the allocator**: Enabling `graphene-hardened` or
  `graphene-hardened-light` inside the guest improves memory safety for many
  applications:

```nix
# configuration.nix
environment.memoryAllocator.provider = "graphene-hardened";
# OR for a more permissive and better performing allocator:
# environment.memoryAllocator.provider = "graphene-hardened-light";
```

Some software (notably certain browsers) can be finicky with hardened mallocs
and may require rebuilding or alternatives; be prepared to switch to another
browser or allocator profile when you hit incompatibilities.

- **Disable nonessential features**: Turn off USB redirection/debugging, audio
  devices, and other “extras” you do not need in the VM. These often pull in
  complex subsystems that are rarely worth the extra attack surface for a
  security-focused guest.

For deeper NixOS-specific hardening, see:
[hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)

---

### Nix Toolbox

> ⚠️ Warning: toolbx containers are integrated with the host system, don't do
> things you wouldn't do on your host. Toolbx containers are not fully isolated
> environments like VMs.

That said, they are a fast and convenient way to spin up a Nix development
environment. Know the limitations and benefits, and when you need more
isolation, just spin up a VM instead.

Secureblue enforces restrictive container image policies by default, blocking
unsigned or unverified images from registries like GitHub Container Registry.
This requires explicit trust configuration for each container source.

```bash
ujust set-container-userns on
```

Without this setting, containers will fail with `OCI permission denied` errors.

**Allow the Nix-Toolbox Image**

```bash
# For system-wide configuration (affects all users)
run0 podman image trust set -t accept ghcr.io/thrix/nix-toolbox

# For user-specific configuration (recommended for development)
podman image trust set -t accept ghcr.io/thrix/nix-toolbox
```

The `-t accept` flag allows images from this registry without requiring
signature verification.

```bash
# Check that the policy has been updated correctly:
podman image trust show
```

Create the `nix-toolbox` container:

```bash
toolbox create --image ghcr.io/thrix/nix-toolbox:42
```

You will be prompted whether you want Home Manager installed or not as well.

Enter the toolbox:

```bash
toolbox enter nix-toolbox-42
```

You can then use nix and home-manager to setup a fully declarative
dev-environment.

---

### Real-world recovery example

Secureblue’s design and the underlying firmware safeguards also make certain
failures recoverable. On a mini PC, running a firmware update command resulted
in a boot error (“Something went seriously wrong, MOK is full”) and a forced
shutdown. Resetting NVRAM by moving the jumper on the motherboard briefly, then
restoring it to the original position, allowed the system to retrain and boot
again, after which the Secure Boot key could be re-enrolled and the system
returned to a known-good, secure state.

<details>
<summary> ✔️ My Experience with Secureblue </summary>

Using secureblue as the host OS with NixOS in a VM has been surprisingly smooth
for day‑to‑day work. Performance has been more than adequate for editing,
development, and browsing, and every issue so far has been fixable with
rollbacks or small config changes—no “nuke and reinstall” moments required.

**Software installation and workflows**

Flatpak takes a bit of relearning if you are used to installing everything with
full root on a mutable distro. Tools like Flatseal help a lot: you can see
exactly which permissions an app has, then selectively tighten them instead of
blindly trusting defaults. On secureblue, running the
`ujust flatpak-permissions-lockdown` helper gives you a very strict baseline,
then you add back only what each app truly needs.

In practice, a hybrid approach has worked best. One editor runs as a Flatpak,
and another is installed via `rpm-ostree` for tighter system integration and the
“traditional” root behavior when needed. The same thing happened with `yazi`: to
get the exact workflow wanted, it was easier to install it via `rpm-ostree`
rather than poking so many holes in the Flatpak sandbox that most isolation
benefits disappeared.

Toolbx also fits into this nicely. Putting Homebrew and Flatpak inside a toolbox
lets more config be shared while keeping the host image clean. The general
pattern on Silverblue/secureblue is: Flatpak for most GUI apps, toolbox (plus
brew or distro packages) for CLI tooling, and only a small number of host‑layer
`rpm-ostree` installs when deep integration is really warranted.

One quirk worth knowing about: on secureblue, `/home` is a symlink to
`/var/home`. Most tools don’t care, but a few development workflows get confused
by the indirection. In those cases, pointing the tool directly at
`/var/home/username` instead of `/home/username` usually clears things up.

**Graphics and drivers in the NixOS VM**

For GPU and display, the safest approach has been to let secureblue own the
hardware stack and keep the NixOS guest as simple as possible. Extra GPU drivers
or compositor tweaks inside the VM tended to make things less stable, showing up
as flicker, random freezes, or generally janky graphics because the guest was
effectively fighting the host’s configuration. Sticking close to the defaults in
the VM has consistently produced smoother and more predictable graphics
behavior.

The main issue I've had is with the dns-selector occasionally causing networking
problems. I configure global DNS with their `ujust` command and I'm assuming
that updates were incompatible with my DNS setup. Running `ujust dns-selector`
and pressing `1` (Reset to defaults), and a reboot typically fix the connection
and within a few days the global DNS will work again.

</details>

---

### Resources

- [RedHat What is virtualization?](https://www.redhat.com/en/topics/virtualization/what-is-virtualization)

- [virtualization & hypervisors](https://sumit-ghosh.com/posts/virtualization-hypervisors-explaining-qemu-kvm-libvirt/)

- [Virtualization on Linux using the KVM/QEMU/Libvirt stack](https://bitgrounds.tech/posts/kvm-qemu-libvirt-virtualization/)
