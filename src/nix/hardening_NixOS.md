# Hardening NixOS

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![guy fawks hacker](../images/guy_fawks.png) -->

Securing your NixOS system begins with a philosophy of minimalism, explicit
configuration, and proactive control.

> ⚠️ Warning: I am not a security expert. This guide presents various options
> for hardening NixOS, but it is your responsibility to evaluate whether each
> adjustment suits your specific needs and environment. Security hardening and
> process isolation can introduce stability challenges, compatibility issues, or
> unexpected behavior. Additionally, these protections often come with
> performance tradeoffs. Always conduct thorough research, there are no plug and
> play one size fits all security solutions.

> That said, I typically write about what I'm implementing myself to deepen
> understanding and share what works for me. `--Source` means the proceeding
> paragraph came from `--Source`, you can often click to check for yourself. If
> you use some common sense with a bit of caution you could end up with a more
> secure NixOS system that fits your needs.

> Much of this guide draws inspiration or recommendations from the well-known
> [Linux Hardening Guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)
> by Madaidan's Insecurities. Madaidan’s work is widely regarded in technical
> and security circles as one of the most comprehensive and rigorously
> researched sources on practical Linux security, frequently cited for its depth
> and actionable advice. For example, much of the original basis for hardening
> for [nix-mineral](https://github.com/cynicsketch/nix-mineral) came from this
> guide as well. This can be a starting point but shouldn't be blindly followed
> either, always do your own research, things change frequently.

For an article with apposing perspectives, see
[debunking-madaidans-insecurities](https://chyrp.cgps.ch/en/debunking-madaidans-insecurities/).
We can learn from both and hopefully find something in between that is closer to
the truth.

> ❗ **Note on SELinux and AppArmor**: While NixOS can provide a high degree of
> security through its immutable and declarative nature, it's important to
> understand the limitations regarding Mandatory Access Control (MAC)
> frameworks. Neither SELinux nor AppArmor are fully supported or widely used in
> the NixOS ecosystem. You can do a lot to secure NixOS but if anonymity and
> isolation are paramount, I recommend booting into a
> [Tails USB stick](https://tails.net/). Or using
> [Whonix](https://www.whonix.org/).

☝️ The unique file structure of NixOS, particularly the immutable `/nix/store`,
makes it difficult to implement and manage the file-labeling mechanisms that
these frameworks rely on. There are ongoing community efforts to improve
support, but as of now, they are considered experimental and not a standard part
of a typical NixOS configuration. For an immutable distro that implements
SELinux by default at a system level as well as many other hardening techniques,
see [Fedora secureblue](https://secureblue.dev/).

Containers and VMs are beyond the scope of this chapter but can also enhance
security and sandboxing if configured correctly.

It's crucial to **document every change** you make. By creating smaller,
feature-complete commits, each with a descriptive message, you're building a
clear history. This approach makes it far simpler to revert a breaking change
and quickly identify what went wrong. Over time, this discipline allows you to
create security-focused checklists and ensure all angles are covered, building a
more robust and secure system.

Check out the
[Hardening NixOS Baseline Hardening README](https://saylesss88.github.io/nix/index.html)
for baseline hardening recommendations and best practices.

There is something to be said about the window manager you use. GNOME, KDE
Plasma, and Sway secure privileged Wayland protocols like screencopy. This means
that on environments outside of GNOME, KDE, and Sway, applications can access
screen content of the entire desktop. This implicitly includes the content of
other applications. It's primarily for this reason that Silverblue, Kinoite, and
Sericea images are recommended. COSMIC has plans to fix this.
--[secureblue Images](https://secureblue.dev/images)

## Minimal Installation with LUKS

Begin with NixOS’s minimal installation image. This gives you a base system with
only essential tools and no extras that could introduce vulnerabilities.

## Manual Encrypted Install Following the Manual

Encryption is the process of using an algorithm to scramble plaintext data into
ciphertext, making it unreadable except to a person who has the key to decrypt
it.

**Data at rest** is data in storage, such as a computer's or a servers hard
disk.

**Data at rest encryption** (typically hard disk encryption), secures the
documents, directories, and files behind an encryption key. Encrypting your data
at rest prevents data leakage, physical theft, unauthorized access, and more as
long as the key management scheme isn't compromised.

- [Minimal ISO Download (64-bit Intel/AMD)](https://channels.nixos.org/nixos-25.05/latest-nixos-minimal-x86_64-linux.iso)

- [NixOS Manual Installation](https://nixos.org/manual/nixos/stable/#sec-installation)

- [NixOS Wiki Full Disk Encryption](https://wiki.nixos.org/wiki/Full_Disk_Encryption)

- The
  [NSA, CISA, and NIST warn](https://www.nsa.gov/Press-Room/Press-Releases-Statements/Press-Release-View/Article/3498776/post-quantum-cryptography-cisa-nist-and-nsa-recommend-how-to-prepare-now/)
  that nation-state actors are likely stockpiling encrypted data now, preparing
  for a future when quantum computers could break today’s most widely used
  encryption algorithms. Sensitive data with long-term secrecy needs is
  especially at risk.

- This is a wake-up call to use the strongest encryption available today and to
  plan early for post-quantum security.

- [NIST First 3 Post-Quantum Encryption Standards](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)
  Organizations and individuals should prepare to migrate cryptographic systems
  to these new standards as soon as practical.

- They chose
  [Four Quantum-Resistant Cryptographic Algorithms](https://www.nist.gov/news-events/news/2022/07/nist-announces-first-four-quantum-resistant-cryptographic-algorithms)
  warning that public-key cryptography is especially vulnerable and widely used
  to protect digital information.

## Guided Encrypted BTRFS Subvol install using disko

Use LUKS encryption to protect your data at rest, the following guide is a
minimal disko encrypted installation:
[Encrypted Install](https://saylesss88.github.io/installation/enc/enc_install.html)

## Impermanence

Impermanence, especially when using a `tmpfs` as the root filesystem, provides
several significant security benefits. The core principle is that impermanence
defeats persistence, a fundamental goal for any attacker.

When you use a root-as-tmpfs setup on NixOS, the boot process loads the entire
operating system from the read-only Nix store into a `tmpfs` in RAM. The mutable
directories, such as `/etc` and `/var`, are then created on this RAM disk. When
the system is shut down, the `tmpfs` is wiped, leaving the on-disk storage
untouched and secure.

This means you get a fresh, secure boot every time, making it much harder for an
attacker to maintain a presence on your system.

- [Erase your Darlings (ZFS)](https://grahamc.com/blog/erase-your-darlings/)

- [Encrypted BTRFS Impermanence Guide](https://saylesss88.github.io/installation/enc/encrypted_impermanence.html)
  Only follow this guide if you also followed the encrypted disko install,
  impermanence is designed to be destructive and needs to match your config
  exactly.

## Secure Boot

<!-- ![Virus](../images/virus1.png) -->

Enable a UEFI password or Administrator password where it requires
authentication in order to access the UEFI/BIOS.

Secure Boot helps ensure only signed, trusted kernels and bootloaders are
executed at startup.

Useful Resources:

<details>
<summary> ✔️ Click to Expand Secure Boot Resources </summary>

- [The Strange State of Authenticated Boot and Encryption](https://0pointer.net/blog/authenticated-boot-and-disk-encryption-on-linux.html)

- [NixOS Wiki Secure Boot](https://wiki.nixos.org/wiki/Secure_Boot)

- [lanzaboote repo](https://github.com/nix-community/lanzaboote)

</details>

Practical Lanzaboote Secure Boot setup for NixOS:
[Guide:Secure Boot on NixOS with Lanzaboote](https://saylesss88.github.io/installation/enc/lanzaboote.html)

### The Kernel

Given the kernel's central role, it's a frequent target for malicious actors,
making robust hardening essential.

NixOS provides a `hardened` profile that applies a set of security-focused
kernel and system configurations.

For flakes, you could do something like the following in your
`configuration.nix` or equivalent to import `hardened.nix` and enable
`profiles.hardened`:

```nix
# configuration.nix
{ pkgs, inputs, ... }: let
   modulesPath = "${inputs.nixpkgs}/nixos/modules";

in {
  imports = [ "${modulesPath}/profiles/hardened.nix" ];

}
```

- There is a proposal to remove it completely that has gained ground, the
  following thread discusses why:
  [Discourse Thread](https://discourse.nixos.org/t/proposal-to-deprecate-the-hardened-profile/63081)

- [PR #383438](https://github.com/NixOS/nixpkgs/pull/383438) Proposed removal
  PR.

- Check
  [hardened.nix](https://github.com/NixOS/nixpkgs/blob/master/nixos/modules/profiles/hardened.nix)
  to see exactly what adding it enables to avoid duplicates and conflicts moving
  on. I included this for completeness, the choice is yours if you want to use
  it or not.

## Choosing your Kernel

See which kernel you're currently using with:

```bash
# show the kernel release
uname -r
# show kernel version, hostname, and architecture
uname -a
```

Show the configuration of your current kernel:

```bash
zcat /proc/config.gz
# ...snip...
#
# Compression
#
CONFIG_CRYPTO_DEFLATE=m
CONFIG_CRYPTO_LZO=y
CONFIG_CRYPTO_842=m
CONFIG_CRYPTO_LZ4=m
CONFIG_CRYPTO_LZ4HC=m
CONFIG_CRYPTO_ZSTD=y
# end of Compression
# ...snip...
```

The [NixOS Manual](https://nixos.org/manual/nixos/stable/#sec-kernel-config)
states that the default Linux kernel configuration should be fine for most
users.

The Linux kernel is typically released under two forms: stable and long-term
support (LTS). Choosing either has consequences, do your research.
[Stable vs. LTS kernels](https://madaidans-insecurities.github.io/guides/linux-hardening.html#stable-vs-lts)

- [The Linux Kernel Archives Active kernel releases](https://www.kernel.org/category/releases.html)

**OR**, you can choose the hardened kernel for a kernel that prioritizes
security over everything else.

### The Hardened Kernel

The `linuxPackages_latest_hardened` attribute has been deprecated. If you want
to use a hardened kernel, you must specify a versioned package that is currently
supported.

You can find the latest available hardened kernel packages by searching
[pkgs/top-level/linux-kernels.nix](https://github.com/NixOS/nixpkgs/blob/master/pkgs/top-level/linux-kernels.nix)

For example, to use the latest available `6.15`, you would configure it like
this:

```nix
boot.kernelPackages = pkgs.linux_6_15_hardened;
```

Note that this not only replaces the kernel, but also packages that are specific
to the kernel version, such as NVIDIA video drivers. This also removes your
ability to use the `.extend` kernel attribute, they are only available to
_kernel package sets_ (e.g., `linuxPackages_hardened`)

- If you decide to use this, read further before rebuilding.

You can inspect
[nixpkgs/pkgs/os-specific/linux/kernel/hardened/patches.json](https://github.com/NixOS/nixpkgs/blob/master/pkgs/os-specific/linux/kernel/hardened/patches.json)
to see the metadata of the patches that are applied. You can then follow the
links in the `.json` file to see the patch diffs.

> ❗ NOTE: Always check the `linux-kernels.nix` file for the latest available
> versions, as older kernels are regularly removed from Nixpkgs.

### sysctl

A tool for checking the security hardening options of the Linux kernel:

```nix
environment.systemPackages = [ pkgs.kernel-hardening-checker ];
```

`sysctl` is a tool that allows you to view or modify kernel settings and
enable/disable different features.

Check all `sysctl` parameters against the `kernel-hardening-checker`
recommendations:

```bash
sudo sysctl -a > params.txt
kernel-hardening-checker -l /proc/cmdline -c /proc/config.gz -s ./params.txt
```

Check the value of a specific parameter:

```bash
sudo sysctl -a | grep "kernel.kptr_restrict"
# Output:
kernel.kptr_restrict = 2
```

Check Active Linux Security Modules:

```bash
cat /sys/kernel/security/lsm
# Output:
File: /sys/kernel/security/lsm
capability,landlock,yama,bpf,apparmor
```

Check Kernel Configuration Options:

```bash
zcat /proc/config.gz | grep CONFIG_SECURITY_SELINUX
zcat /proc/config.gz | grep CONFIG_HARDENED_USERCOPY
zcat /proc/config.gz | grep CONFIG_STACKPROTECTOR
```

Since it is difficult to see exactly what enabling the hardened_kernel does.
Before rebuilding, you could do something like this to see exactly what is
added:

```bash
sudo sysctl -a > before.txt
```

And after the rebuild:

```bash
sudo sysctl -a > after.txt
```

And finally run a `diff` on them:

```bash
diff before.txt after.txt
```

You can also diff against `after.txt` for future changes to avoid duplicates,
this seems easier to me than trying to parse through the patches.

## Kernel Security Settings

```nix
security = {
      protectKernelImage = true;
      lockKernelModules = false; # this breaks iptables, wireguard, and virtd

      # force-enable the Page Table Isolation (PTI) Linux kernel feature
      forcePageTableIsolation = true;

      # User namespaces are required for sandboxing.
      # this means you cannot set `"user.max_user_namespaces" = 0;` in sysctl
      allowUserNamespaces = true;

      # Disable unprivileged user namespaces, unless containers are enabled
      unprivilegedUsernsClone = config.virtualisation.containers.enable;
      allowSimultaneousMultithreading = true;
}
```

## Further Hardening with sysctl

`sysctl` hardening settings further reinforce kernel-level protections. The
hardened kernel includes security patches and stricter defaults, but it doesn't
cover all runtime tunables. Refer to the above commands to get a diff of the
changes.

[boot.kernel.sysctl](https://nixos.org/manual/nixos/stable/options#opt-boot.kernel.sysctl):
Runtime parameters of the Linux kernel, as set by sysctl(8). Note that the
sysctl parameters names must be enclosed in quotes. Values may be a string,
integer, boolean, or null.

Check what each setting does [sysctl-explorer](https://sysctl-explorer.net/)

Refer to
[madadaidans-insecurities#sysctl-kernel](https://madaidans-insecurities.github.io/guides/linux-hardening.html#sysctl-kernel)
for the following settings and their explainations.

```nix
  boot.kernel.sysctl = {
    "fs.suid_dumpable" = 0;
    # prevent pointer leaks
    "kernel.kptr_restrict" = 2;
    # restrict kernel log to CAP_SYSLOG capability
    "kernel.dmesg_restrict" = 1;
    # Note: certian container runtimes or browser sandboxes might rely on the following
    # restrict eBPF to the CAP_BPF capability
    "kernel.unprivileged_bpf_disabled" = 1;
    # should be enabled along with bpf above
    # "net.core.bpf_jit_harden" = 2;
    # restrict loading TTY line disciplines to the CAP_SYS_MODULE
    "dev.tty.ldisk_autoload" = 0;
    # prevent exploit of use-after-free flaws
    "vm.unprivileged_userfaultfd" = 0;
    # kexec is used to boot another kernel during runtime and can be abused
    "kernel.kexec_load_disabled" = 1;
    # Kernel self-protection
    # SysRq exposes a lot of potentially dangerous debugging functionality to unprivileged users
    # 4 makes it so a user can only use the secure attention key. A value of 0 would disable completely
    "kernel.sysrq" = 4;
    # disable unprivileged user namespaces, Note: Docker, NH, and other apps may need this
    # "kernel.unprivileged_userns_clone" = 0; # commented out because it makes NH and other programs fail
    # restrict all usage of performance events to the CAP_PERFMON capability
    "kernel.perf_event_paranoid" = 3;

    # Network
    # protect against SYN flood attacks (denial of service attack)
    "net.ipv4.tcp_syncookies" = 1;
    # protection against TIME-WAIT assassination
    "net.ipv4.tcp_rfc1337" = 1;
    # enable source validation of packets received (prevents IP spoofing)
    "net.ipv4.conf.default.rp_filter" = 1;
    "net.ipv4.conf.all.rp_filter" = 1;

    "net.ipv4.conf.all.accept_redirects" = 0;
    "net.ipv4.conf.default.accept_redirects" = 0;
    "net.ipv4.conf.all.secure_redirects" = 0;
    "net.ipv4.conf.default.secure_redirects" = 0;
    # Protect against IP spoofing
    "net.ipv6.conf.all.accept_redirects" = 0;
    "net.ipv6.conf.default.accept_redirects" = 0;
    "net.ipv4.conf.all.send_redirects" = 0;
    "net.ipv4.conf.default.send_redirects" = 0;

    # prevent man-in-the-middle attacks
    "net.ipv4.icmp_echo_ignore_all" = 1;

    # ignore ICMP request, helps avoid Smurf attacks
    "net.ipv4.conf.all.forwarding" = 0;
    "net.ipv4.conf.default.accept_source_route" = 0;
    "net.ipv4.conf.all.accept_source_route" = 0;
    "net.ipv6.conf.all.accept_source_route" = 0;
    "net.ipv6.conf.default.accept_source_route" = 0;
    # Reverse path filtering causes the kernel to do source validation of
    "net.ipv6.conf.all.forwarding" = 0;
    "net.ipv6.conf.all.accept_ra" = 0;
    "net.ipv6.conf.default.accept_ra" = 0;

    ## TCP hardening
    # Prevent bogus ICMP errors from filling up logs.
    "net.ipv4.icmp_ignore_bogus_error_responses" = 1;

    # Disable TCP SACK
    "net.ipv4.tcp_sack" = 0;
    "net.ipv4.tcp_dsack" = 0;
    "net.ipv4.tcp_fack" = 0;

    # Userspace
    # restrict usage of ptrace
    "kernel.yama.ptrace_scope" = 2;

    # ASLR memory protection (64-bit systems)
    "vm.mmap_rnd_bits" = 32;
    "vm.mmap_rnd_compat_bits" = 16;

    # only permit symlinks to be followed when outside of a world-writable sticky directory
    "fs.protected_symlinks" = 1;
    "fs.protected_hardlinks" = 1;
    # Prevent creating files in potentially attacker-controlled environments
    "fs.protected_fifos" = 2;
    "fs.protected_regular" = 2;

    # Randomize memory
    "kernel.randomize_va_space" = 2;
    # Exec Shield (Stack protection)
    "kernel.exec-shield" = 1;

    ## TCP optimization
    # TCP Fast Open is a TCP extension that reduces network latency by packing
    # data in the sender’s initial TCP SYN. Setting 3 = enable TCP Fast Open for
    # both incoming and outgoing connections:
    "net.ipv4.tcp_fastopen" = 3;
    # Bufferbloat mitigations + slight improvement in throughput & latency
    "net.ipv4.tcp_congestion_control" = "bbr";
    "net.core.default_qdisc" = "cake";
  };
```

> ❗ Note: The above settings are fairly aggressive and can break common
> programs, read the comment warnings.

## Hardening Boot Parameters

`boot.kernelParams` can be used to set additional kernel command line arguments
at boot time. It can only be used for built-in modules.

You can find the following settings in the above guide in the
[Boot parameters section](https://madaidans-insecurities.github.io/guides/linux-hardening.html#boot-parameters)

```nix
# boot.nix
      boot.kernelParams = [
        # make it harder to influence slab cache layout
        "slab_nomerge"
        # enables zeroing of memory during allocation and free time
        # helps mitigate use-after-free vulnerabilaties
        "init_on_alloc=1"
        "init_on_free=1"
        # randomizes page allocator freelist, improving security by
        # making page allocations less predictable
        "page_alloc.shuffel=1"
        # enables Kernel Page Table Isolation, which mitigates Meltdown and
        # prevents some KASLR bypasses
        "pti=on"
        # randomizes the kernel stack offset on each syscall
        # making attacks that rely on a deterministic stack layout difficult
        "randomize_kstack_offset=on"
        # disables vsyscalls, they've been replaced with vDSO
        "vsyscall=none"
        # disables debugfs, which exposes sensitive info about the kernel
        "debugfs=off"
        # certain exploits cause an "oops", this makes the kernel panic if an "oops" occurs
        "oops=panic"
        # only alows kernel modules that have been signed with a valid key to be loaded
        # making it harder to load malicious kernel modules
        # can make VirtualBox or Nvidia drivers unusable
        "module.sig_enforce=1"
        # prevents user space code excalation
        "lockdown=confidentiality"
        # "rd.udev.log_level=3"
        # "udev.log_priority=3"
      ];
```

This is a thoughtful start to hardening boot parameters, there are more
recommendations in the guide.

Kernel modules for hardware devices are generally loaded automatically by
`udev`. You can force a module to be loaded via `boot.kernelModules`.

[boot.blacklistedKernelModules](https://nixos.org/manual/nixos/stable/options#opt-boot.blacklistedKernelModules):
List of names of kernel modules that should not be loaded automatically by the
hardware probing code.

You can find the following settings in the
[Blacklisting Kernel Modules Section](https://madaidans-insecurities.github.io/guides/linux-hardening.html#kasr-kernel-modules)

```nix
      boot.blacklistedKernelModules = [
        # Obscure networking protocols
        "dccp"   # Datagram Congestion Control Protocol
        "sctp"  # Stream Control Transmission Protocol
        "rds"  # Reliable Datagram Sockets
        "tipc"  # Transparent Inter-Process Communication
        "n-hdlc" # High-level Data Link Control
        "ax25"  # Amateur X.25
        "netrom"  # NetRom
        "x25"     # X.25
        "rose"
        "decnet"
        "econet"
        "af_802154"  # IEEE 802.15.4
        "ipx"  # Internetwork Packet Exchange
        "appletalk"
        "psnap"  # SubnetworkAccess Protocol
        "p8023"  # Novell raw IEE 802.3
        "p8022"  # IEE 802.3
        "can"   # Controller Area Network
        "atm"
        # Various rare filesystems
        "cramfs"
        "freevxfs"
        "jffs2"
        "hfs"
        "hfsplus"
        "udf"

        # "squashfs"  # compressed read-only file system used for Live CDs
        # "cifs"  # cmb (Common Internet File System)
        # "nfs"  # Network File System
        # "nfsv3"
        # "nfsv4"
        # "ksmbd"  # SMB3 Kernel Server
        # "gfs2"  # Global File System 2
        # vivid driver is only useful for testing purposes and has been the
        # cause of privilege escalation vulnerabilities
        # "vivid"
      ];
```

As with the `kernelParameters` above, there are more suggestions in the guide, I
have used the above parameters along with the commented out ones and had no
issues.

## Hardening Systemd

<!-- ![Hacker](../images/hacker.png) -->

`systemd` is the core "init system" and service manager that controls how
services, daemons, and basic system processes are started, stopped and
supervised on modern Linux distributions, including NixOS. It provides a suite
of basic building blocks for a Linux system as well as a system and service
manager that runs as `PID 1` and starts the rest of the system.

Because it launches and supervises almost all system services, hardening systemd
means raising the baseline security of your entire system.

`dbus-broker` is generally considered more secure and robust but isn't the
default as of yet. To set `dbus-broker` as the default:

```nix
  users.groups.netdev = {};
  services = {
    dbus.implementation = "broker";
    logrotate.enable = true;
    journald = {
      storage = "volatile"; # Store logs in memory
      upload.enable = false; # Disable remote log upload (the default)
      extraConfig = ''
        SystemMaxUse=500M
        SystemMaxFileSize=50M
      '';
    };
  };
```

- `dbus-broker` is more resilient to resource exhaustion attacks and integrates
  better with Linux security features.

- [Rethinking-the-dbus-message-bus](https://dvdhrm.github.io/rethinking-the-dbus-message-bus/)

- Setting `storage = "volatile"` tells journald to keep log data only in memory.
  There is a tradeoff though, If you need long-term auditing or troubleshooting
  after a reboot, this will not preserve system logs.

- `upload.enable` is for forwarding log messages to remote servers, setting this
  to false prevents accidental leaks of potentially sensitive or internal system
  information.

- Enabling `logrotate` prevents your disk from filling with excessive
  **legacy/service** log files. These are the classic plain-text logs.

- Systemd uses `journald` which stores logs in a binary format

You can check the security status with:

```bash
systemd-analyze security
# or for a detailed view of individual services security posture
systemd-analyze security NetworkManager
```

Further reading on systemd:

<details>
<summary> ✔️ Click to Expand Systemd Resources </summary>

- [systemd.io](https://systemd.io/)

- [Rethinking PID 1](https://0pointer.de/blog/projects/systemd.html)

- [Biggest Myths about Systemd](https://0pointer.de/blog/projects/the-biggest-myths.html)

</details>

The following is a repo containing many of the Systemd hardening settings in
NixOS format:

[nix-system-services-hardened](https://github.com/wallago/nix-system-services-hardened)

For example, to harden bluetooth you could add the following to your
`configuration.nix` or equivalent:

```nix
systemd.services = {
      bluetooth.serviceConfig = {
      ProtectKernelTunables = lib.mkDefault true;
      ProtectKernelModules = lib.mkDefault true;
      ProtectKernelLogs = lib.mkDefault true;
      ProtectHostname = true;
      ProtectControlGroups = true;
      ProtectProc = "invisible";
      SystemCallFilter = [
        "~@obsolete"
        "~@cpu-emulation"
        "~@swap"
        "~@reboot"
        "~@mount"
      ];
      SystemCallArchitectures = "native";
    };
}
```

As you can see from above, you typically use the `serviceConfig` attribute to
harden settings for systemd services.

```bash
systemd-analyze security bluetooth
→ Overall exposure level for bluetooth.service: 3.3 OK 🙂
```

## Lynis and other tools

Lynis is a security auditing tool for systems based on UNIX like Linux, macOS,
BSD, and others.--[lynis repo](https://github.com/CISOfy/lynis)

`chkrootkit` was removed as it is unmaintained and archived upstream.

Installation:

```nix
environment.systemPackages = [
pkgs.lynis
pkgs.clamav
pkgs.aide
 ];
```

<details>
<summary> ✔️ Click to Expand AIDE Example </summary>

AIDE is an intrusion detection system (IDS) that will notify us whenever it
detects that a potential intrusion has occurred. When a system is compromised,
attackers typically will try to change file permissions and escalate to the root
user account and start to modify system files, AIDE can detect this.

To set up AIDE on your system follow these steps:

1. Create the `aide.conf`:

```bash
sudo mkdir -p /var/lib/aide && cd /var/lib/aide/
sudo hx aide.conf
```

Add the following content to `/var/lib/aide/aide.conf`:

```conf
# aide.conf
# Example configuration file for AIDE.

@@define DBDIR /var/lib/aide

# The location of the database to be read.
database_in=file:@@{DBDIR}/aide.db.gz

# The location of the database to be written.
#database_out=sql:host:port:database:login_name:passwd:table
#database_out=file:aide.db.new
database_out=file:@@{DBDIR}/aide.db.new.gz

# Whether to gzip the output to database
gzip_dbout=yes

log_level=info

report_url=file:/var/log/aide/aide.log
report_url=stdout
#report_url=stderr
#NOT IMPLEMENTED report_url=mailto:root@foo.com
#NOT IMPLEMENTED report_url=syslog:LOG_AUTH

# These are the default rules.
#
#p:      permissions
#i:      inode:
#n:      number of links
#u:      user
#g:      group
#s:      size
#b:      block count
#m:      mtime
#a:      atime
#c:      ctime
#S:      check for growing size
#md5:    md5 checksum
#sha1:   sha1 checksum
#rmd160: rmd160 checksum
#tiger:  tiger checksum
#haval:  haval checksum
#gost:   gost checksum
#crc32:  crc32 checksum
#R:      p+i+n+u+g+s+m+c+md5
#L:      p+i+n+u+g
#E:      Empty group
#>:      Growing logfile p+u+g+i+n+S

# You can create custom rules like this.

NORMAL = R+b+sha512

DIR = p+i+n+u+g

# Next decide what directories/files you want in the database.

/boot   NORMAL
/bin    NORMAL
/sbin   NORMAL
/lib    NORMAL
/opt    NORMAL
/usr    NORMAL
/root   NORMAL

# Check only permissions, inode, user and group for /etc, but
# cover some important files closely.
/etc    p+i+u+g
!/etc/mtab
/etc/exports  NORMAL
/etc/fstab    NORMAL
/etc/passwd   NORMAL
/etc/group    NORMAL
/etc/gshadow  NORMAL
/etc/shadow   NORMAL

/var/log   p+n+u+g

# With AIDE's default verbosity level of 5, these would give lots of
# warnings upon tree traversal. It might change with future version.
#
#=/lost\+found    DIR
#=/home           DIR
```

Create the logfile:

```bash
sudo mkdir -p /var/log/aide
sudo touch /var/log/aide/aide.log
```

2. Generate the initial database, this will store the checksums of all files
   that it's configured to monitor. Take note of the location of the new
   database, mine was `/etc/aide.db.new`

```bash
sudo aide --config /var/lib/aide/aide.conf --init
```

3. Move the new database and remove the `.new`:

```bash
sudo mv /var/lib/aide/aide.db.new.gz /var/lib/aide/aide.db.gz
```

```bash
ls /var/lib/aide/
aide.conf   aide.db.gz
```

4. Check with AIDE:

```bash
sudo aide --check --config /var/lib/aide/aide.conf
Start timestamp: 2025-09-05 09:50:07 -0400 (AIDE 0.19.2)
AIDE found NO differences between database and filesystem. Looks okay!!
```

5. Whenever you make changes to system files, or especially after running a
   system update or installing new tools, you have to rescan all files to update
   their checksums in the AIDE database:

```bash
sudo aide --update --config /var/lib/aide/aide.conf
```

Unfortunately, AIDE doesn't automatically replace the old database so you have
to rename the new one again:

```bash
sudo mv /var/lib/aide/aide.db.new.gz /var/lib/aide/aide.db.gz
```

And finally check again:

```bash
sudo aide --check --config /var/lib/aide/aide.conf
```

- [aide(1) man page](https://linux.die.net/man/1/aide)

</details>

<details>
<summary> ✔️ Click to Expand clamav.nix Example </summary>

```nix
{pkgs, ...}: {
  environment.systemPackages = with pkgs; [
    clamav
  ];
  services.clamav = {
    # Enable clamd daemon
    daemon.enable = true;
    updater.enable = true;
    updater.frequency = 12; # Number of database checks per day
    scanner = {
      enable = true;
      # 4:00 AM
      interval = "*-*-* 04:00:00";
      scanDirectories = [
        "/home"
        "/var/lib"
        "/tmp"
        "/etc"
        "/var/tmp"
      ];
    };
  };
}
```

</details>

Lynis Usage:

```bash
sudo lynis show commands
# Output:
Commands:
lynis audit
lynis configure
lynis generate
lynis show
lynis update
lynis upload-only

sudo lynis audit system
# Output:
  Lynis security scan details:

  Hardening index : 79 [###############     ]
  Tests performed : 234
  Plugins enabled : 0

  Components:
  - Firewall               [V]
  - Malware scanner        [V]

  Scan mode:
  Normal [V]  Forensics [ ]  Integration [ ]  Pentest [ ]

  Lynis modules:
  - Compliance status      [?]
  - Security audit         [V]
  - Vulnerability scan     [V]
```

- The "Lynis hardening index" is an overall impression on how well a system is
  hardened. However, this is just an indicator on measures taken - not a
  percentage of how safe a system might be. A score over 75 typically indicates
  a system with more than average safety measures implemented.

- Lynis will give you more recommendations for securing your system as well.

If you use `clamscan`, create the following log file:

```bash
sudo touch /var/log/clamscan.log
```

Example cron job for `clamav` & `aide`:

```nix
{pkgs, ...}: {
  services.cron = {
    enable = true;
    # messages.enable = true;
    systemCronJobs = [
      # Every day at 2:00 AM, run clamscan as root and append output to a log file
      "0 2 * * * root ${pkgs.clamav}/bin/clamscan -r /home >> /var/log/clamscan.log"
      "0 11 * * * ${pkgs.aide}/bin/aide --check --config /var/lib/aide/aide.conf"
    ];
  };
}
```

ClamAV usage:

You can run `clamav` manually with:

```bash
# Recursive Scan:
sudo clamscan -r ~/home
```

> ❗ NOTE: You only need either the individual `pkgs.clamav` with the cron job
> **OR** the `clamd-daemon` module. `clamdscan` is for software integration and
> uses a different user that doesn't have permission to scan your files. You can
> use `clamdscan --fdpass /path/to/scan` to pass the necessary file permissions.
> NOTE: `clamdscan` runs in the background, you can watch it with `top`.

## Securing SSH

> **Security information**: Changing SSH configuration settings can
> significantly impact the security of your system(s). It is crucial to have a
> solid understanding of what you are doing before making any adjustments. Avoid
> blindly copying and pasting examples, including those from this Wiki page,
> without conducting a thorough analysis. Failure to do so may compromise the
> security of your system(s) and lead to potential vulnerabilities. Take the
> time to comprehend the implications of your actions and ensure that any
> changes made are done thoughtfully and with care. --NixOS Wiki

> ❗ NOTE: Choose one, either `ssh-agent` or `gpg-agent`

1. Use normal SSH keys generated with `ssh-keygen`, this is recommended unless
   you have a good reason for not using it.

**OR**

2. Use a GPG key with `gpg-agent` (which acts as your SSH agent). Complex, and
   harder to understand in my opinion.

My setup caused conflicts when enabling `programs.ssh.startAgent` so I chose
`gpg-agent` personally.

There are situations where you are required to use one or the other like for
headless CI/CD environments, `ssh-keygen` is required.

- [Click Here for GnuPG and gpg-agent chapter](https://saylesss88.github.io/nix/gpg-agent.html)

Further reading:

<details>
<summary> ✔️ Click to Expand Resourses on OpenSSH </summary>

- [Arch Wiki OpenSSH](https://wiki.archlinux.org/title/OpenSSH)

- [Gentoo GnuPG](https://wiki.gentoo.org/wiki/GnuPG)

- [A Visual Explanation of GPG Subkeys](https://rgoulter.com/blog/posts/programming/2022-06-10-a-visual-explanation-of-gpg-subkeys.html)

- [Secure Secure Shell](https://blog.stribik.technology/2015/01/04/secure-secure-shell.html)

</details>

## Key generation

### ssh-keygen

The `ed25519` algorithm is significantly faster and more secure when compared to
`RSA`. You can also specify the key derivation function (KDF) rounds to
strengthen protection even more.

For example, to generate a strong key for MdBook:

```bash
ssh-keygen -t ed25519 -a 32 -f ~/.ssh/id_ed25519_github_$(date +%Y-%m-%d) -C "SSH Key for MdBook"
```

- `-t` is for type

- `-a 32` sets the number of KDF rounds. The standard is usually good enough,
  adding extra rounds can make it harder to brute-force.

- `-f` is for filename

### OpenSSH Server

First of all, if you don't use SSH don't enable it in the first place. If you do
use SSH, it's important to understand what that opens you up to.

The following are some recommendations from Mozilla on OpenSSH:

- [Mozilla OpenSSH guidelines](https://infosec.mozilla.org/guidelines/openssh.html)

The following OpenSSH setup is based on the above guidelines with strong
algorithms, and best practices:

```nix
{config, ...}: {
  config = {
    services = {
      fail2ban = {
        enable = true;
        maxretry = 5;
        bantime = "1h";
        # ignoreIP = [
        # "172.16.0.0/12"
        # "192.168.0.0/16"
        # "2601:881:8100:8de0:31e6:ac52:b5be:462a"
        # "matrix.org"
        # "app.element.io" # don't ratelimit matrix users
        # ];

        bantime-increment = {
          enable = true; # Enable increment of bantime after each violation
          multipliers = "1 2 4 8 16 32 64 128 256";
          maxtime = "168h"; # Do not ban for more than 1 week
          overalljails = true; # Calculate the bantime based on all the violations
        };
      };
      openssh = {
        enable = true;
        settings = {
          PasswordAuthentication = false;
          PermitEmptyPasswords = false;
          PermitTunnel = false;
          UseDns = false;
          KbdInteractiveAuthentication = false;
          X11Forwarding = config.services.xserver.enable;
          MaxAuthTries = 3;
          MaxSessions = 2;
          ClientAliveInterval = 300;
          ClientAliveCountMax = 0;
          AllowUsers = ["your-user"];
          TCPKeepAlive = false;
          AllowTcpForwarding = false;
          AllowAgentForwarding = false;
          LogLevel = "VERBOSE";
          PermitRootLogin = "no";
          KexAlgorithms = [
            # Key Exchange Algorithms in priority order
            "curve25519-sha256@libssh.org"
            "ecdh-sha2-nistp521"
            "ecdh-sha2-nistp384"
            "ecdh-sha2-nistp256"
            "diffie-hellman-group-exchange-sha256"
          ];
          Ciphers = [
            # stream cipher alternative to aes256, proven to be resilient
            # Very fast on basically anything
            "chacha20-poly1305@openssh.com"
            # industry standard, fast if you have AES-NI hardware
            "aes256-gcm@openssh.com"
            "aes128-gcm@openssh.com"
            "aes256-ctr"
            "aes192-ctr"
            "aes128-ctr"
          ];
          Macs = [
            # Combines the SHA-512 hash func with a secret key to create a MAC
            "hmac-sha2-512-etm@openssh.com"
            "hmac-sha2-256-etm@openssh.com"
            "umac-128-etm@openssh.com"
            "hmac-sha2-512"
            "hmac-sha2-256"
            "umac-128@openssh.com"
          ];
        };
        # These keys will be generated for you
        hostKeys = [
          {
            path = "/etc/ssh/ssh_host_ed25519_key";
            type = "ed25519";
          }
        ];
      };
    };
  };
}
```

TCP port 22 (ssh) is opened automatically if the SSH daemon is enabled
(`services.openssh.enable = true;`)

Much of the SSH hardening settings came from
[ryanseipp's secure-ssh Guide](https://ryanseipp.com/post/nixos-secure-ssh/)
with some additions of my own.

Fail2Ban is an intrusion prevention software framework. It's designed to prevent
brute-force attacks by scanning log files for suspicious activity, such as
repeated failed login attempts.

OpenSSH is the primary tool for secure remote access for NixOS. Enabling it
activates the OpenSSH server on the system, allowing incoming SSH connections.

The above configuration is a robust setup for securing an SSH server by:

- Preventing brute-force attacks with Fail2Ban

- Eliminating password authentication in favor of more secure SSH keys

- Restricting user access and preventing root login

- Disabling potentially risky forwarding features (tunnel, TCP, agent)

- Enforce the use of strong, modern cryptographic algorithms for all SSH
  communications.

- Enhanced logging for better auditing.

Further Reading:

- [OpenSSH](https://www.openssh.com/)

- [DigitalOcean how fail2ban works](https://www.digitalocean.com/community/tutorials/how-fail2ban-works-to-protect-services-on-a-linux-server)

## Encrypted Secrets

Never store secrets in plain text in repositories. Use something like
[sops-nix](https://github.com/Mic92/sops-nix), which lets you keep encrypted
secrets under version control declaratively.

Another option is [agenix](https://github.com/ryantm/agenix)

- [NixOS Wiki Agenix](https://wiki.nixos.org/wiki/Agenix)

### Sops-nix Guide

Protect your secrets, the following guide is on setting up Sops on NixOS:
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)

## Auditd

To enable the Linux Audit Daemon (`auditd`) and define a very basic rule set,
you can use the following NixOS configuration. This example demonstrates how to
log every program execution (`execve`) on a 64-bit architecture.

```nix
# modules/security/auditd-minimal.nix (or directly in configuration.nix)
{
  # start as early in the boot process as possible
  boot.kernelParams = ["audit=1"];
  security.auditd.enable = true;
  security.audit.enable = true;
  security.audit.rules = [
    # Log all program executions on 64-bit architecture
    "-a exit,always -F arch=b64 -S execve"
  ];
}
```

- `audit=1` Enables auditing at the kernel level very early in the boot process.
  Without this, some events could be missed.

- `security.auditd.enable = true;` Ensures the `auditd` userspace daemon is
  started.

- While often enabled together, `security.audit.enable` specifically refers to
  enabling the NixOS module for audit rules generation.

- `execve` (program executions)

- This is just a basic configuration, there is much more that can be tracked.

## USB Port Protection

It's important to protect your USB ports to prevent BadUSB attacks, data
exfiltration, unauthorized device access, malware injection, etc.

To get a list of your connected USB devices you can use `lsusb` from the
`usbutils` package.

```bash
lsusb
```

To list the devices recognized by USBGuard, run:

```bash
sudo usbguard list-devices
```

- [MyNixOS services.usbguard](https://mynixos.com/options/services.usbguard)

Change `your-user` to your username:

```nix
# usbguard.nix
{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) mkIf;
  cfg = config.custom.security.usbguard;
in {
  options.custom.security.usbguard = {
    enable = lib.mkEnableOption "usbguard";
  };

  config = mkIf cfg.enable {
    services.usbguard = {
      enable = true;
      IPCAllowedUsers = ["root" "your-user"];
    # presentDevicePolicy refers to how to treat USB devices that are already connected when the daemon starts
      presentDevicePolicy = "allow";
      rules = ''
        # allow `only` devices with mass storage interfaces (USB Mass Storage)
        allow with-interface equals { 08:*:* }
        # allow mice and keyboards
        # allow with-interface equals { 03:*:* }

        # Reject devices with suspicious combination of interfaces
        reject with-interface all-of { 08:*:* 03:00:* }
        reject with-interface all-of { 08:*:* 03:01:* }
        reject with-interface all-of { 08:*:* e0:*:* }
        reject with-interface all-of { 08:*:* 02:*:* }
      '';
    };

    environment.systemPackages = [pkgs.usbguard];
  };
}
```

The above settings can be found in
[RedHat UsbGuard](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/7/html/security_guide/sec-using-usbguard)

The only `allow` rule is for devices with **only** mass storage interfaces
(`08:*:*`) i.e., USB Mass storage devices, devices like keyboards and mice
(which use interface class `03:*:*`) implicitly **not allowed**.

The `reject` rules reject devices with a suspicious combination of interfaces. A
USB drive that implements a keyboard or a network interface is very suspicious,
these `reject` rules prevent that.

The `presentDevicePolicy = "allow";` allows any device that is present at daemon
start up even if they're not explicitly allowed. However, newly plugged in
devices must match an `allow` rule or get denied implicitly.

The `presentDevicePolicy` should be one of: # one of `"apply-policy"`(default,
evaluate the rule set for every present device), `"block"`, `"reject"`, `"keep"`
(keep whatever state the device is currently in), or `"allow"`, which is used in
the example.

There is also the
[usbguard-notifier](https://github.com/Cropi/usbguard-notifier)

And enable it with the following in your `configuration.nix` or equivalent:

```nix
# configuration.nix
imports = [
    ./usbguard.nix
];
custom.security.usbguard.enable = true;
```

> ❗ If you are ever unsure about a setting that you want to harden and think
> that it could possibly break your system you can always use a specialisation
> reversing the action and choose it's generation at boot up. For example, to
> force-reverse the above settings you could:
>
> ```nix
> # configuration.nix
> specialisation.no-usbguard.configuration = {
>     services.usbguard.enable = lib.mkForce false;
> };
> ```
>
> - This is a situation where I recommend this, it's easy to lock yourself out
>   of your keyboard, mouse, etc. when trying to configure this.

Further Reading:

- [NinjaOne BadUSB](https://www.ninjaone.com/it-hub/endpoint-security/what-is-badusb/)

- [USBGuard](https://usbguard.github.io/)

- [NixCraft USBGuard](https://www.cyberciti.biz/security/how-to-protect-linux-against-rogue-usb-devices-using-usbguard/)

## Doas over sudo

For a more minimalist version of `sudo` with a smaller codebase and attack
surface, consider `doas`. Replace `userName` with your username:

```nix
# doas.nix
{
  lib,
  config,
  pkgs, # Add pkgs if you need to access user information
  ...
}: let
  cfg = config.custom.security.doas;
in {
  options.custom.security.doas = {
    enable = lib.mkEnableOption "doas";
  };

  config = lib.mkIf cfg.enable {
    # Disable sudo
    security.sudo.enable = false;

    # Enable and configure `doas`.
    security.doas = {
      enable = true;
      extraRules = [
        {
          # Grant doas access specifically to your user
          users = ["userName"]; # <--- Only give access to your user
          # persist = true; # Convenient but less secure
          # noPass = true;    # Convenient but even less secure
          keepEnv = true; # Often necessary
          # Optional: You can also specify which commands they can run, e.g.:
          # cmd = "ALL"; # Allows running all commands (default if not specified)
          # cmd = "/run/current-system/sw/bin/nixos-rebuild"; # Only allow specific command
        }
      ];
    };

    # Add an alias to the shell for backward-compat and convenience.
    environment.shellAliases = {
      sudo = "doas";
    };
  };
}
```

You would then import this into your `configuration.nix` and enable/disable it
with the following:

```nix
# configuration.nix

imports = [
    ./doas.nix
];

custom.security.doas.enable = true;
```

> ❗ NOTE: Many people opt for the less secure `groups = ["wheel"];` in the
> above configuration instead of `users = ["userName"];` to give wider access,
> the choice is yours.

## Firejail

> ❗️ Critics such as madaidan say that Firejail worsens security by acting as a
> privilege escalation hole. Firejail requires the executable to be setuid,
> meaning it runs with root privileges.This is risky because any vulnerability
> in Firejail can lead to privilege escalation. This combined with many
> convenience features and complicated command line flags leads to a large
> attack surface.

- I haven't personally tried
  [nix-bwrapper](https://github.com/Naxdy/nix-bwrapper) myself yet, but it's
  another sandboxing option that looks interesting. Bubblewrap is known for
  having a more minimal design and smaller attack surface.
  - Also see: [Flatpak section](#flatpak) for another option for sandboxing.

- [nix-bubblewrap](https://sr.ht/~fgaz/nix-bubblewrap/) is another option.

- [NixOS Wiki Firejail](https://wiki.nixos.org/wiki/Firejail)

- [Arch Wiki Firejail](https://wiki.archlinux.org/title/Firejail)

> ❗ WARNING: Running untrusted code is never safe, sandboxing cannot change
> this. --Arch Wiki

```nix
# firejail.nix
{
  pkgs,
  lib,
  ...
}: {
  programs.firejail = {
    enable = true;
    wrappedBinaries = {
      # Sandbox a web browser
      librewolf = {
        executable = "${lib.getBin pkgs.librewolf}/bin/librewolf";
        profile = "${pkgs.firejail}/etc/firejail/librewolf.profile";
      };
      # Sandbox a file manager
      thunar = {
        executable = "${lib.getBin pkgs.xfce.thunar}/bin/thunar";
        profile = "${pkgs.firejail}/etc/firejail/thunar.profile";
      };
      # Sandbox a document viewer
      zathura = {
        executable = "${lib.getBin pkgs.zathura}/bin/zathura";
        profile = "${pkgs.firejail}/etc/firejail/zathura.profile";
      };
    };
  };
}
```

`wrappedBinaries` is a list of applications you want to run inside a sandbox.
Only the apps in the `wrappedBinaries` attribute set will be automatically
firejailed when launched the usual way.

Other apps may be started manually using `firejail <app>`, or added to
`wrappedBinaries` if you want automatic sandboxing, just make sure the profile
exists.

To inspect which profiles are available, after rebuilding go to `/nix/store/`, I
used Yazi to search for `/firejail` and followed it to `firejail/etc`, where the
profiles are.

There are many flags and options available with firejail, I suggest checking out
`man firejail`.

There are comments explaining what's going on in:
[firejail/package.nix](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/fi/firejail/package.nix)

Firejail is a SUID program that reduces the risk of security breaches by
restricting the running environment of untrusted applications using
[Linux namespaces](https://lwn.net/Articles/531114/) and
[seccomp-bpf](https://l3net.wordpress.com/2015/04/13/firejail-seccomp-guide/)--[Firejail Security Sandbox](https://firejail.wordpress.com/)

It provides sandboxing and access restriction per application, much like what
AppArmor/SELinux does at a kernel level. However, it's not as secure or
comprehensive as kernel-enforced MAC systems (AppArmor/SELinux), since it's a
userspace tool and can potentially be bypassed by privilege escalation exploits.

---

## Flatpak

> ❗️NOTE: You cannot effectively use Firejail with Flatpak apps because of how
> their sandboxing technologies operate.

Apps that don't have a flatpak equivalent can be further hardened with
bubblewrap independently but bubblewrap is not needed on Flatpak apps.

Because of this limited native MAC (Mandatory Access Control) support on NixOS,
using Flatpak is often a good approach to get sandboxing and isolation for many
GUI apps.

- Flatpak bundles runtimes and sandbox mechanisms that provide app isolation
  independently of the host system's AppArmor or SELinux infrastructure. This
  can improve security and containment for GUI applications running on NixOS
  despite the system lacking full native MAC coverage.

- Flatpak apps benefit from sandboxing through bubblewrap, which isolate apps
  and restrict access to user/home and system resources.

Add Flatpak with the FlatHub repository for all users:

```nix
services.flatpak.enable = true;
  systemd.services.flatpak-repo = {
    wantedBy = [ "multi-user.target" ];
    path = [ pkgs.flatpak ];
    script = ''
      flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
      # Only apps that are verified
      # flatpak remote-add --if-not-exists --subset=verified flathub-verified https://flathub.org/repo/flathub.flatpakrepo
    '';
  };
```

- [Flathub Verified Apps](https://docs.flathub.org/docs/for-users/verification)

- [Flatpak the good the bad the ugly](https://secureblue.dev/articles/flatpak)

Then you can either find apps through [FlatHub](https://flathub.org/en) or on
the cmdline with `flatpak search <app>`. Flatpak is best used for GUI apps, some
cli apps can be installed through it but not all.

- There is also [nix-flatpak](https://github.com/gmodena/nix-flatpak), which
  enables you to manage your flatpaks declaratively.

- [Flatseal](https://flathub.org/en/apps/com.github.tchx84.Flatseal) is GUI
  utility that enables you to review and modify permissions from your Flatpak
  apps.

- [Warehouse](https://flathub.org/en/apps/io.github.flattool.Warehouse) provides
  a simple UI to control complex Flatpak options, no cmdline required.

Considering that your browser is likely the most vulnerable piece of your whole
setup, it can be beneficial to install it with Flatpak, effectively sandboxing
it. You may have to adjust some of the "portals", flatpaks way of accessing
system resources.

---

## SeLinux/AppArmor MAC (Mandatory Access Control)

**AppArmor** is available on NixOS, but is still in a somewhat experimental and
evolving state. There are only a few profiles that have been adapted to NixOS,
see here
[Discourse on default-profiles](https://discourse.nixos.org/t/apparmor-default-profiles/16780)
Which guides you here
[apparmor/includes.nix](https://github.com/NixOS/nixpkgs/blob/2acaef7a85356329f750819a0e7c3bb4a98c13fe/nixos/modules/security/apparmor/includes.nix)
where you can see some of the abstractions and tunables to follow progress.

**SELinux**: Experimental, not fully integrated, recent progress for
advanced/curious users; expect rough edges and manual intervention if you want
to try it. Most find SELinux more complex to configure and maintain than
AppArmor.

This isn't meant to be a comprehensive guide, more to get people thinking about
security on NixOS.

See the following guide on hardening networking:

- [Hardening Networking](https://saylesss88.github.io/nix/hardening_networking.html)

## Resources

### Advanced Hardening with `nix-mineral` (Community Project)

<details>
<summary> ✔️ Click to Expand section on `nix-mineral` </summary>

For users seeking a more comprehensive and opinionated approach to system
hardening beyond the built-in `hardened` profile, the community project
[`nix-mineral`](https://github.com/cynicsketch/nix-mineral) offers a declarative
NixOS module.

`nix-mineral` aims to apply a wide array of security configurations, focusing on
tweaking kernel parameters, system settings, and file permissions to reduce the
attack surface.

- **Community Project Status:** `nix-mineral` is a community-maintained project
  and is not officially part of the Nixpkgs repository or NixOS documentation.
  Its development status is explicitly stated as "Alpha software," meaning it
  may introduce stability issues or unexpected behavior.

For detailed information on `nix-mineral`'s capabilities and current status,
refer directly to its
[GitHub repository](https://github.com/cynicsketch/nix-mineral).

</details>

- [AppArmor and apparmor.d on NixOS](https://hedgedoc.grimmauld.de/s/hWcvJEniW#)

- [SELinux on NixOS](https://tristanxr.com/post/selinux-on-nixos/)

- [Paranoid NixOS](https://xeiaso.net/blog/paranoid-nixos-2021-07-18/)

- [NixOS Wiki Security](https://wiki.nixos.org/wiki/Security)

- [Luks Encrypted File Systems](https://nixos.org/manual/nixos/unstable/index.html#sec-luks-file-systems)

- [Discourse A Modern and Secure Desktop](https://discourse.nixos.org/t/a-modern-and-secure-desktop-setup/41154)

- [notashelf NixOS Security 1 Systemd](https://notashelf.dev/posts/insecurities-remedies-i)

- [ryanseipp hardening-nixos](https://ryanseipp.com/post/hardening-nixos/)

- [madaidans Linux Hardening Guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)

- [Hardening-Linux-Servers](https://cybersecuritynews.com/hardening-linux-servers)

- [linux-audit Linux Server hardening best practices](https://linux-audit.com/linux-server-hardening-most-important-steps-to-secure-systems/)

- [linux-audit Linux security guide extended](https://linux-audit.com/linux-security-guide-extended-version/)

- [Arch Wiki Security](https://wiki.archlinux.org/title/Security)

- [Gentoo Security_Handbook Concepts](https://wiki.gentoo.org/wiki/Security_Handbook/Concepts)

- STIGs are configuration standards developed by the Defense Information Systems
  Agency (DISA) to secure systems and software for the U.S. Department of
  Defense (DoD). They are considered a highly authoritative source for system
  hardening.There are recommendations for hardening all kinds of software in the
  [Stig Viewer](https://stigviewer.com/stigs)

- [CIS Benchmarks](https://www.cisecurity.org/cis-benchmarks)

- [NSA Cybersecurity Directorate](https://github.com/nsacyber)
