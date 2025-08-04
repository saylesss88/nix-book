# Hardening NixOS

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![guy fawks hacker](../images/guy_fawks.png)

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
> understanding and share what works for me. Accuracy is important to me so if
> you catch a mistake, please let me know so I can fix it. `--Source` means the
> proceeding paragraph came from `--Source`, you can often click to check for
> yourself. Much of the information comes directly from the wiki or other
> respected sources that are also linked in multiple places. If you use some
> common sense with a bit of caution you could end up with a more secure NixOS
> system that fits your needs.

Containers and VMs are beyond the scope of this chapter but can also enhance
security if configured correctly.

## Best Practices

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

> NOTE: There is mention of making
> [userborn](https://github.com/nikstur/userborn) the default for NixOS in the
> future. It can be more secure by prohibiting UID/GID re-use and giving
> warnings about insecure password hashing schemes. From the userborn docs all
> that is clear is how to install it, I see no mention on how to use it or how
> to convert from declarative NixOS users to userborn.

You can also specify which users or groups are allowed to do anything with the
Nix daemon and Nix package manager. The following setting will only allow
members of the `wheel` group access to commands that require elevated
privileges, such as installing or modifying system-wide packages:

```nix
# configuration.nix
{ ... }:
{
  nix.settings.allowed-users = [ "@wheel" ];
}
```

**OR** Only allow the `root` user:

```nix
# configuration.nix
{ ... }:
{
  nix.allowedUsers = [ "root" ];
}
```

This is more restrictive and much less convenient, think twice before going this
restrictive.

**Only install, enable, and run what is needed**: Disable or uninstall
unnecessary software and services to minimize potential vulnerabilities. Take
advantage of NixOS’s easy package management and minimalism to keep your system
lean and secure.

**Avoid permanently installing temporary tools**: Use tools like `nix-shell`,
`comma`, and `nix-direnv` to test or run software temporarily. This prevents
clutter and reduces potential risks from unused software lingering on the
system.

**Update regularly**: Keep your system and software up to date to receive the
latest security patches. Delaying updates leaves known vulnerabilities open to
exploitation.

**Apply the Principle of Least Privilege**: Never run tools or services as root
unless absolutely necessary. Create dedicated users and groups with the minimum
required permissions to limit potential damage if compromised. See the doas
example. [Check the doas example here](#doas-over-sudo)

**Use strong passwords and passphrases**: Aim for at least 14–16 characters by
combining several unrelated words, symbols, and numbers. For example:
`sunset-CoffeeHorse$guitar!`. Strong passphrases are both memorable and secure.

**Use a password manager and enable multi-factor authentication (MFA)**: Manage
unique, strong passwords effectively with a trusted manager and protect accounts
with MFA wherever possible for a second layer of defense.

After establishing some standard best practices, it’s time to dive deeper into
system hardening, the process of adding layered safeguards throughout your NixOS
setup. This next section guides you through concrete steps and options for
hardening critical areas of your system: from encryption and secure boot to
managing secrets, tightening kernel security, and leveraging platform-specific
tools.

## Minimal Installation with LUKS

Begin with NixOS’s minimal installation image. This gives you a base system with
only essential tools and no extras that could introduce vulnerabilities.

## Manual Encrypted Install Following the Manual

- [Minimal ISO Download (64-bit Intel/AMD)](https://channels.nixos.org/nixos-25.05/latest-nixos-minimal-x86_64-linux.iso)

- [NixOS Manual Installation](https://nixos.org/manual/nixos/stable/#sec-installation)

- [NixOS Wiki Full Disk Encryption](https://wiki.nixos.org/wiki/Full_Disk_Encryption)

## Guided Encrypted install using disko

Use LUKS encryption to protect your data at rest, the following guide is a
minimal disko encrypted installation:
[Encrypted Install](https://saylesss88.github.io/installation/enc/enc_install.html)

## Secure Boot

![Virus](../images/virus1.png)

Secure Boot helps ensure only signed, trusted kernels and bootloaders are
executed at startup.

Useful Resources:

- [The Strange State of Authenticated Boot and Encryption](https://0pointer.net/blog/authenticated-boot-and-disk-encryption-on-linux.html)

- [NixOS Wiki Secure Boot](https://wiki.nixos.org/wiki/Secure_Boot)

- [lanzaboote repo](https://github.com/nix-community/lanzaboote)

Practical Lanzaboote Secure Boot setup for NixOS:
[Guide:Secure Boot on NixOS with Lanzaboote](https://saylesss88.github.io/installation/enc/lanzaboote.html)

## Hardening the Kernel

Given the kernel's central role, it's a frequent target for malicious actors,
making robust hardening essential.

- [NixOS Wiki Linux Kernel](https://wiki.nixos.org/wiki/Linux_kernel)

NixOS provides a `hardened` profile that applies a set of security-focused
kernel and system configurations.

The following discourse thread explains the use of `profiles.hardened`:

- [Discourse Thread Enabling hardened profile](https://discourse.nixos.org/t/enabling-hardened-profile/63107)

I misunderstood the above thread, it means that **if** the file is imported that
it's enabled by default. If you look at `profiles/hardened.nix` you'll see that
it defaults to true:

```nix
# nixpkgs/nixos/modules/profiles/hardened.nix
# ... snip ...
{
  options.profiles.hardened = mkEnableOption "hardened" // {
    default = true;
    example = false;
  };
# ... snip ...
```

For flakes, you could do something like the following in your
`configuration.nix` or equivalent:

```nix
# configuration.nix
{ pkgs, inputs, ... }: let
   modulesPath = "${inputs.nixpkgs}/nixos/modules";

in {
  imports = [ "${modulesPath}/profiles/hardened.nix" ];

}
```

- Now after importing the above module into your configuration,
  `profiles.hardened` is enabled by default.

- There is a proposal to remove it completely, so you may want to think twice
  before enabling it and read the following
  [Discourse Thread](https://discourse.nixos.org/t/proposal-to-deprecate-the-hardened-profile/63081)
  Talking about removing it completely because of reasons listed in the above
  thread.

- [PR #383438](https://github.com/NixOS/nixpkgs/pull/383438)

- If you do decide to use the hardened profile, if you decide to continue
  hardening your system you need to know exactly what `profiles.hardened`
  enables/disables so you avoid duplicate entries and conflicts. Check
  [hardened.nix](https://github.com/NixOS/nixpkgs/blob/master/nixos/modules/profiles/hardened.nix).

## Choosing the Hardened Kernel

You can also use the hardened kernel:

```nix
boot.kernelPackages = pkgs.linuxPackages_latest_hardened;
```

`sysctl` is a tool that allows you to view or modify kernel settings and
enable/disable different features.

Check all `sysctl` parameters (long output):

```bash
sysctl -a
```

Or a specific parameter:

```bash
sysctl -a | grep "kernel.kptr_restrict"
```

Check Active Linux Security Modules:

```bash
cat /sys/kernel/security/lsm
```

Check Kernel Configuration Options:

```bash
zcat /proc/config.gz | grep CONFIG_SECURITY_SELINUX
zcat /proc/config.gz | grep CONFIG_HARDENED_USERCOPY
zcat /proc/config.gz | grep CONFIG_STACKPROTECTOR
```

## OR Harden your existing Kernel

If you chose the hardened kernel don't follow this section.

**Or** you can harden the kernel you're using `sysctl`, the following parameters
come from the madaidans-insecurities guide with a few optimizations:

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

Note: The above settings are fairly aggressive and can break common programs, I
left comment warnings. The following guide explains kernel hardening and many of
the parameters above:
[Linux Hardening Guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)

## Hardening Boot Parameters

`boot.kernelParams` can be used to set additional kernel command line arguments
at boot time. It can only be used for built-in modules.

You can find the following settings in the above guide in the Kernel
self-protection section:

```nix
# boot.nix
      kernelParams = [
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

There are many more recommendations in the
[Linux Hardening Guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)

In the above guide, the following are in the Blacklisting kernel modules
section:

```nix
      blacklistedKernelModules = [
        # Obscure networking protocols
        "dccp"
        "sctp"
        "rds"
        "tipc"
        "n-hdlc"
        "ax25"
        "netrom"
        "x25"
        "rose"
        "decnet"
        "econet"
        "af_802154"
        "ipx"
        "appletalk"
        "psnap"
        "p8023"
        "p8022"
        "can"
        "atm"
        # Various rare filesystems
        "cramfs"
        "freevxfs"
        "jffs2"
        "hfs"
        "hfsplus"
        "udf"

        # Not so rare filesystems
        # "squashfs"
        # "cifs"
        # "nfs"
        # "nfsv3"
        # "nfsv4"
        # "ksmbd"
        # "gfs2"
        # vivid driver is only useful for testing purposes and has been the
        # cause of privilege escalation vulnerabilities
        # "vivid"
      ];
```

As with the `kernelParameters` above, there are more suggestions in the guide, I
have used the above parameters and had no issues.

## Hardening Systemd

![Hacker](../images/hacker.png)

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

- Setting `storage = "volatile"` tells journald to keep log data only in memory.
  There is a tradeoff though, If you need long-term auditing or troubleshooting
  after a reboot, this will not preserve system logs.

- `upload.enable` is for forwarding log messages to remote servers, setting this
  to false prevents accidental leaks of potentially sensitive or internal system
  information.

- Enabling `logrotate` prevents your disk from filling with excessive
  **legacy/service** log files. These are the classic plain-text logs.

- Systemd uses `journald` which stores logs in a binary format which we take
  care of with the `extraConfig` settings.

You can check the security status with:

```bash
systemd-analyze security
# or for a detailed view of individual services security posture
systemd-analyze security NetworkManager
```

Further reading on systemd:

- [systemd.io](https://systemd.io/)

- [Rethinking PID 1](https://0pointer.de/blog/projects/systemd.html)

- [Biggest Myths about Systemd](https://0pointer.de/blog/projects/the-biggest-myths.html)

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

Installation:

```nix
environment.systemPackages = [
pkgs.lynis
pkgs.chkrootkit
pkgs.clamav
pkgs.aide
 ];
```

Usage:

```bash
sudo lynis show commands
sudo lynis audit system
 Lynis security scan details:

  Hardening index : 78 [###############     ]
  Tests performed : 231
  Plugins enabled : 0

  Components:
  - Firewall               [V]
  - Malware scanner        [V]
```

- Lynis will give you more recommendations for securing your system as well.

Example cron job for `chkrootkit`:

```nix
{pkgs, ...}: {
  services.cron = {
    enable = true;
    # messages.enable = true;
    systemCronJobs = [
      # Every Sunday at 2:10 AM, run chkrootkit as root, log output for review
      "10 2 * * 0 root ${pkgs.chkrootkit}/bin/chkrootkit | logger -t chkrootkit"
    ];
  };
}
```

The above cron job will use `chkrootkit` to automatically scan for known rootkit
signatures. It can detect hidden processes and network connections.

I got the recommendation for `clamav` from the Paranoid NixOS blog post and the
others help with compliance for `lynis`.

## Securing SSH

> **Security information**: Changing SSH configuration settings can
> significantly impact the security of your system(s). It is crucial to have a
> solid understanding of what you are doing before making any adjustments. Avoid
> blindly copying and pasting examples, including those from this Wiki page,
> without conducting a thorough analysis. Failure to do so may compromise the
> security of your system(s) and lead to potential vulnerabilities. Take the
> time to comprehend the implications of your actions and ensure that any
> changes made are done thoughtfully and with care. --NixOS Wiki

> ❗ NOTE: I am going to show two approaches for SSH authentication. You should
> typically choose **one**:

1. Use normal SSH keys generated with `ssh-keygen`, this is recommended unless
   you have a good reason for not using it.

**OR**

2. Use a GPG key with `gpg-agent` (which acts as your SSH agent). Complex, and
   harder to understand in my opinion.

My setup caused conflicts when enabling `programs.ssh.startAgent` so I chose
`gpg-agent` personally.

There are situations where you are required to use one or the other like for
headless CI/CD environments, `ssh-keygen` is required.

Further reading:

- [Arch Wiki OpenSSH](https://wiki.archlinux.org/title/OpenSSH)

- [Gentoo GnuPG](https://wiki.gentoo.org/wiki/GnuPG)

- [A Visual Explanation of GPG Subkeys](https://rgoulter.com/blog/posts/programming/2022-06-10-a-visual-explanation-of-gpg-subkeys.html)

- [Secure Secure Shell](https://blog.stribik.technology/2015/01/04/secure-secure-shell.html)

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

**OR**

## Install GNUPG and gpg-agent for Home Manager

<details>
<summary> ✔️ Click to expand PGP installation and key generation example </summary>

**PGP (Pretty Good Privacy)** and **GPG (GNU Privacy Guard)**. While distinct,
they are deeply interconnected and, for the rest of this section, I'll use the
terms interchangeably.

**PGP** was the original, groundbreaking software that brought robust public-key
cryptography to the masses. It set the standard for secure email communication.
However, PGP later became a commercial product.

To provide a free and open-source alternative that anyone could use and inspect,
**GPG** was created. Crucially, **GPG** is a complete implementation of the
OpenPGP standard. This open standard acts as a universal language for encryption
and digital signatures.

**What’s safe to share?**

- Your public key (used to encrypt files and verify signatures)

- Your key ID (identifies your key, useful for sharing public keys or configs)

**What must never be shared?**

- Your private (secret) key, usually in your `~/.gnupg/private-keys-v1.d/`
  directory.

- Your passphrase for your private key.

Home Manager module with `gpg-agent`, `gnupg`, and `pinentry-gnome3`:

```nix
# gpg-agent.nix
{
  config,
  lib,
  pkgs,
  ...
}: {
  options = {
    custom.pgp = {
      enable = lib.mkEnableOption {
        description = "Enable PGP Gnupgp";
        default = false;
      };
    };
  };

  config = lib.mkIf config.custom.pgp.enable {
    services = {
      ## Enable gpg-agent with ssh support
      gpg-agent = {
        enable = true;
        enableSshSupport = true;
        enableZshIntegration = true;
        pinentryPackage = pkgs.pinentry-gnome3;
      };

      ## We will put our keygrip here
      gpg-agent.sshKeys = [];
    };
    home.packages = [pkgs.gnupg];
    programs = {
      gpg = {
        ## Enable GnuPG
        enable = true;

        # homedir = "/home/userName/.config/gnupg";
      };
    };
  };
}
```

- The default path is `~/.gnupg`, if you prefer placing it in the `~/.config`
  directory uncomment the `homedir` line and change `userName` to your username.

- I use hyprland so `pinentry-gnome3` works for me, there is also the following
  options for this attribute:

- `pinentry-tty`

- `pinentry-qt`

- `pinentry-gtk2`

And more, research what you need and use the correct one.

Enable in your `home.nix` or equivalent:

```nix
# home.nix
# ... snip ...
imports = [
    ./gpg-agent.nix
];
custom.pgp.enable = true;
# ... snip ...
```

`gpg --full-generate-key` can be used to generate a basic keypair, adding
`--expert` gives more options and capabilities needed for `gpg-agent`.

To generate a pgp key you can do the following:

```bash
gpg --expert --full-generate-key
```

- Choose the default (11) ECC (set your own capabilities)

- Choose `A` for Authenticate capabilities, which is the only setting required
  for this. Additional subkeys may be created for encryption, sign, and/or
  authentication capabilities.

- Choose (1) Default Curve 25519

- Give it a name and description

- Give it an expiration date, 1y is common

- Use a strong passphrase or password

If you see a warning about incorrect permissions, you can run the following:

```bash
chmod 700 ~/.gnupg
chmod 600 ~/.gnupg/*
```

Verify:

```bash
ls -ld ~/.gnupg
# Should show: drwx------

ls -l ~/.gnupg
# Files should show: -rw-------
```

After fixing, run:

```bash
gpg --list-keys
```

The warning should be gone.

**List your key and copy the key ID**

```bash
gpg --list-secret-keys --keyid-format LONG
```

Output example:

```bash
sec   ed25519/ABCDEF1234567890 2025-07-31 [SC]
      ABC123ABC123ABC123ABC123ABC123ABC123ABC1
uid           [ultimate] Your Name <you@example.com>
ssb   ed25519/1234567890ABCDEF 2025-07-31 [S]
```

- Take the part after `/` on the `sec` line (e.g., `ABCDEF1234567890`)

**Export the public key for GitHub**

```bash
gpg --armor --export ABCDEF1234567890
```

- Copy everything from
  `-----BEGIN PGP PUBLIC KEY BLOCK----- to -----END PGP PUBLIC KEY BLOCK-----`.

**Add to GitHub**

1. Go to Settings, SSH and GPG keys, New GPG key

2. Paste the exported block.

**Add Keygrip to `sshcontrol` for gpg-agent**

```bash
gpg --list-secret-keys --with-keygrip --with-colons
```

- Copy the `grp` line - that's your keygrip

Add the keygrip number to your `gpg-agent.sshKeys` and rebuild:

```nix
gpg-agent.sshKeys = ["6BD11826F3845BC222127FE3D22C92C91BB3FB32"];
```

- By itself, a keygrip cannot be used to reconstruct your private key. It's
  derived from the public key material, not from the secret key itself so it's
  safe to version control. Don't put your keygrip in a public repo if you don't
  want people to know you use that key for signing/authentication. It's not a
  security risk, but it leaks a tiny bit of metadata.

The following article mentions the keygrip being computed from public elements
of the key:

- [gnupg-users what-is-a-keygrip](https://gnupg-users.gnupg.narkive.com/q5JtahdV/gpg-agent-what-is-a-keygrip)

- Never version-control your private key files or `.gnupg` contents.

Add the following to your shell config:

```bash
# zsh.nix
# ... snip ...
initContent = ''
      # GPG Agent
        export GPG_TTY=$(tty) # which terminal to use for passphrase prompts
        export SSH_AUTH_SOCK="$(gpgconf --list-dirs agent-ssh-socket)" # points SSH_AUTH_SOCK to the socket created by gpg-agent
        gpg-connect-agent updatestartuptty /bye # refresh gpg-agent so it knows which terminal is active

        # Optional: confirm it's set correctly
        echo "SSH_AUTH_SOCK is set to: $SSH_AUTH_SOCK"
'';
# ... snip ...
```

Rebuild and then restart gpg-agent:

```bash
gpgconf --kill gpg-agent
gpgconf --launch gpg-agent
```

Test, these should match:

```bash
echo "$SSH_AUTH_SOCK"
# output
/run/user/1000/gnupg/d.wft5hcsny4qqq3g31c76534j/S.gpg-agent.ssh

gpgconf --list-dirs agent-ssh-socket
# output
/run/user/1000/gnupg/d.wft5hcsny4qqq3g31c76834j/S.gpg-agent.ssh
```

```bash
ssh-add -L
# output
ssh-ed25519 AABBC3NzaC1lZDI1NTE5AAAAIGXwhVokJ6cKgodYT+0+0ZrU0sBqMPPRDPJqFxqRtM+I (none)
```

- Mine shows `(none)` because I left the comment field blank when creating the
  key and doesn't affect functionality.

## Encrypt a File with PGP

### List your keys and get the key ID

```bash
gpg --list-keys --keyid-format LONG
```

Example output:

```bash
pub   rsa4096/ABCDEF1234567890 2024-01-01 [SC]
uid           [ultimate] Your Name <you@example.com>
sub   rsa4096/1234567890ABCDEF 2024-01-01 [E]
```

- The part after the slash on the `pub` line is your key ID (`ABCDEF1234567890`
  in the example)

- You can also use your email or name to refer to the key in most commands.

### Encrypt a file for yourself

```bash
echo "This file will be encrypted" > file.txt
```

```bash
gpg --encrypt --recipient ABCDEF1234567890 file.txt
```

```bash
ls
│  7 │ file.txt            │ file │     28 B │ now           │
│  8 │ file.txt.gpg        │ file │    191 B │ now           │
```

`gpg --encrypt` doesn't modify the original file. It creates a new encrypted
file by default with `gpg` amended to the filename.

```bash
gpg --decrypt file.txt.gpg
gpg: encrypted with cv25519 key, ID 0x4AC131B80CEC833E, created 2025-07-31
      "GPG Key <sayls8@proton.me>"
This file will be encrypted
```

- You will be asked for the passphrase you used when creating the key in order
  to decrypt the file.

There is much more you can do with PGP beyond simple file encryption:

- Sign files and commits: Prove that content really came from you.

- Encrypt for multiple recipients: Share encrypted data with teammates using
  their public keys.

- Use smartcards or YubiKeys: Store your private key on hardware for extra
  security.

- Verify software releases: Check that downloaded files are genuine using the
  developer’s signature.

- Integrate with Git: Sign tags and commits so others can trust your repository
  history.

This guide only scratches the surface — once your PGP key and `gpg-agent` are
set up, these capabilities become easy to add to your workflow.

</details>

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
            "curve25519-sha256@libssh.org"
            "ecdh-sha2-nistp521"
            "ecdh-sha2-nistp384"
            "ecdh-sha2-nistp256"
            "diffie-hellman-group-exchange-sha256"
          ];
          Ciphers = [
            "chacha20-poly1305@openssh.com"
            "aes256-gcm@openssh.com"
            "aes128-gcm@openssh.com"
            "aes256-ctr"
            "aes192-ctr"
            "aes128-ctr"
          ];
          Macs = [
            "hmac-sha2-512-etm@openssh.com"
            "hmac-sha2-256-etm@openssh.com"
            "umac-128-etm@openssh.com"
            "hmac-sha2-512"
            "hmac-sha2-256"
            "umac-128@openssh.com"
          ];
        };
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

- Much of the OpenSSH hardening settings adapted to NixOS came from:
  [ryanseipp hardening-nixos](https://ryanseipp.com/post/hardening-nixos/)

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

## Sops-nix Guide

Protect your secrets, the following guide is on setting up Sops on NixOS:
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)

<details>
<summary> ✔️ Click to expand `auditd` example </summary>

To enable the Linux Audit Daemon (`auditd`) and define a very basic rule set,
you can use the following NixOS configuration. This example demonstrates how to
log every program execution (`execve`) on a 64-bit architecture.

```nix
# modules/security/auditd-minimal.nix (or directly in configuration.nix)
{
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

</details>

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
devices must match an `allow` rule or get denied implicitly. I have a keyboard,
and mouse usb device connected and it works without uncommenting the line,
`allow with-interface equals { 03:*:* }` since they were both plugged in at
daemon start up.

The `presentDevicePolicy` should be one of: # one of `"apply-policy"`(default,
evaluate the rule set for every present device), `"block"`, `"reject"`, `"keep"`
(keep whatever state the device is currently in), or `"allow"`

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

- [NixOS Wiki Firejail](https://wiki.nixos.org/wiki/Firejail)

- [Arch Wiki Firejail](https://wiki.archlinux.org/title/Firejail)

> ❗ WARNING: Running untrusted code is never safe, sandboxing cannot change
> this. --Arch Wiki

Firejail is a SUID program that reduces the risk of security breaches by
restricting the running environment of untrusted applications using
[Linux namespaces](https://lwn.net/Articles/531114/) and
[seccomp-bpf](https://l3net.wordpress.com/2015/04/13/firejail-seccomp-guide/)--[Firejail Security Sandbox](https://firejail.wordpress.com/)

It provides sandboxing and access restriction per application, much like what
AppArmor/SELinux does at a kernel level. However, it's not as secure or
comprehensive as kernel-enforced MAC systems (AppArmor/SELinux), since it's a
userspace tool and can potentially be bypassed by privilege escalation exploits.

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
