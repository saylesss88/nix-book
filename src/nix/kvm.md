# Running NixOS in a VM with Maximum Isolation (Beginner Guide)

## Why This Setup?

- **Host**: `secureblue` = Fedora Atomic with **SELinux enforcing**, **sVirt**,
  **Secure Boot**, and hardened defaults.

- **Guest**: NixOS in a VM → full declarative power, zero risk to host.

- **Isolation**: MAC via SELinux + KVM + no direct hardware access.

---

## Step 1: Install secureblue (Hardened Host)

> NOTE: Secureblue enables the hardened_malloc by default which causes problems
> for many browsers and will cause screen flashing with Firefox and others
> within the VM. See:
> [secureblue standard_malloc](https://secureblue.dev/faq#standard-malloc)

1. Download a secureblue image:  
   <https://secureblue.dev/install>

2. Use **Fedora Media Writer** (Flatpak):

```bash
flatpak install flathub org.fedoraproject.MediaWriter
```

3. Flash the secureblue image & enable Secure Boot in UEFI **before** install.

4. On first boot:

```bash
ujust enroll-secureblue-secure-boot-key
```

- Reboot -> Enroll key in MOK manager with password: `secureblue`

5. Post-install hardening: <https://secureblue.dev/post-install>

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

1. Download: NixOS Graphical ISO (GNOME): <https://nixos.org/download/>

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

2. KVM and Host Hardening KVM: KVM provides the low-level, hardware-assisted
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

Hardening the NixOS guest creates an additional, independent layer of defense.

- Helps mitigate VM Breakout: If a zero-day allows an attacker to acheive a VM
  breakout, the security of your system depends entirely on the host's controls.
  Hardening the guest makes it much harder to compromise the initial VM,
  reducing the chance of even attempting to breakout.

- Containment within the VM: Hardening prevents an attacker from gaining full
  control or moving laterally within the VM.
