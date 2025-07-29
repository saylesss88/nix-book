# Hardening NixOS

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![guy fawks hacker](../images/guy_fawks.png)

Securing your NixOS system begins with a philosophy of minimalism, explicit
configuration, and proactive control.

> ⚠️ Warning: I am not a security expert, this is meant to show some of your
> options when hardening NixOS. You will have to judge for yourself if something
> fits your needs or is unnecessary for your setup. Always do your own research,
> hardening and isolating processes can naturally cause some issues. There are
> also performance tradeoffs with added protection. Take what you find useful
> and leave the rest, there is a lot to cover so it's easy for it to get
> convoluted.

Containers and VMs are beyond the scope of this chapter but can also enhance
security if configured correctly.

## Minimal Installation with LUKS

Begin with NixOS’s minimal installation image. This gives you a base system with
only essential tools and no extras that could introduce vulnerabilities.

- [Minimal ISO Download (64-bit Intel/AMD)](https://channels.nixos.org/nixos-25.05/latest-nixos-minimal-x86_64-linux.iso)

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

## Encrypted Secrets

Never store secrets in plain text in repositories. Use something like
[sops-nix](https://github.com/Mic92/sops-nix), which lets you keep encrypted
secrets under version control declaratively.

Protect your sectets, the following guide is on setting up Sops on NixOS:
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)

## Hardening the Kernel

Given the kernel's central role, it's a frequent target for malicious actors,
making robust hardening essential.

NixOS provides a `hardened` profile that applies a set of security-focused
kernel and system configurations. This profile is defined in
[nixpkgs/nixos/modules/profiles/hardened.nix](https://github.com/NixOS/nixpkgs/blob/master/nixos/modules/profiles/hardened.nix)

For users of the NixOS unstable channel, the following is applied by default:

```nix
profiles.hardened.enable = true;
```

**Note on Future Changes**:

- It's important to be aware that the status of the hardened profile is under
  active discussion within the NixOS community. There is a proposal to deprecate
  or remove it in future releases, as discussed in this:
  [Discourse thread](https://discourse.nixos.org/t/proposal-to-deprecate-the-hardened-profile/63081)

- There is an open Pull Request regarding the above thread:
  [PR#383438](https://github.com/NixOS/nixpkgs/pull/383438)

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

## Using sysctl on an existing kernel

**Or** you can harden the kernel you're using `sysctl`, the following parameters
come from the madaidans-insecurities guide with a few optimizations:

```nix
  boot.kernel.sysctl = {
    "fs.suid_dumpable" = false;
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
    # The Magic SysRq key is a key combo that allows users connected to the
    # system console of a Linux kernel to perform some low-level commands.
    # Disable it, since we don't need it, and is a potential security concern.
    "kernel.sysrq" = 4;
    # disable unprivileged user namespaces, Note: Docker, and other apps may need this
    "kernel.unprivileged_userns_clone" = 0;
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

## Hardening Systemd

![Hacker](../images/hacker.png)

`systemd` is the core "init system" and service manager that controls how
services, daemons, and basic system processes are started, stopped and
supervised on modern Linux distributions, including NixOS.

`systemd` is a suite of basic building blocks for a Linux system. It provides a
system and service manager that runs as `PID 1` and starts the rest of the
system.

Because it launches and supervises almost all system services, hardening systemd
means raising the baseline security of your entire system.

`dbus-broker` is generally considered more secure and robust but isn't the
default as of yet. To set `dbus-broker` as the default:

```nix
  users.groups.netdev = {};
  services = {
    usbguard.enable = false;
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

## Firejail

- [NixOS Wiki Firejail](https://wiki.nixos.org/wiki/Firejail)

Firejail is a SUID program that reduces the risk of security breaches by
restricting the running environment of untrusted applications using
[Linux namespaces](https://lwn.net/Articles/531114/) and
[seccomp-bpf](https://l3net.wordpress.com/2015/04/13/firejail-seccomp-guide/)--[Firejail Security Sandbox](https://firejail.wordpress.com/)

It provides sandboxing and access restriction per application, much like what
AppArmor/SELinux does at a kernel level. However, it's not as secure or
comprehensive as kernel-enforced MAC systems (AppArmor/SELinux), since it's a
userspace tool and can potentially be bypassed by privilege escalation exploits.

## Securing SSH

> **Security information**: Changing SSH configuration settings can
> significantly impact the security of your system(s). It is crucial to have a
> solid understanding of what you are doing before making any adjustments. Avoid
> blindly copying and pasting examples, including those from this Wiki page,
> without conducting a thorough analysis. Failure to do so may compromise the
> security of your system(s) and lead to potential vulnerabilities. Take the
> time to comprehend the implications of your actions and ensure that any
> changes made are done thoughtfully and with care. --NixOS Wiki

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

- [Wikipedia Fail2Ban](https://en.wikipedia.org/wiki/Fail2ban)

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

- [USBGuard](https://usbguard.github.io)

You can safely use the following USBGuard configuration in your NixOS
`configuration.nix` to block high-risk composite USB devices (such as those
combining mass storage and keyboard functions), while still allowing standard
USB thumb drives. This setup significantly reduces the likelihood of BadUSB
attacks. If you use legitimate devices that combine multiple interfaces, you may
need to tailor the rules to your hardware.

```nix
{ ... }:
{
    services.usbguard = {
        enable = true;
        dbus.enable = true;
        IPCAllowedGroups = [ "usbguard" "wheel" ];
        rules = ''
        allow with-interface equals { 08:*:* }
        # Reject devices with suspicious combination of interfaces
        reject with-interface all-of { 08:*:* 03:00:* }
        reject with-interface all-of { 08:*:* 03:01:* }
        reject with-interface all-of { 08:*:* e0:*:* }
        reject with-interface all-of { 08:*:* 02:*:* }
        '';
    };
}
```

Further Reading:

- [Wikipedia BadUSB](https://en.wikipedia.org/wiki/BadUSB)

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
attack surface. Its features include, but are not limited to: hardened `sysctl`
options, boot parameter adjustments, root login restrictions, privacy
enhancements (MAC randomization, Whonix machine-id), comprehensive module
blacklisting, firewall configuration, AppArmor integration, and USBGuard
enablement.

**Important Considerations:**

- **Community Project Status:** `nix-mineral` is a community-maintained project
  and is not officially part of the Nixpkgs repository or NixOS documentation.
  Its development status is explicitly stated as "Alpha software," meaning it
  may introduce stability issues or unexpected behavior.
- **Opinionated Configuration:** It applies a broad set of hardening measures
  that might impact system functionality or compatibility with certain
  applications. Users should thoroughly review its source code and test its
  effects in a non-critical environment before deploying.
- **Complementary to Core Hardening:** While comprehensive, it's a layer on top
  of NixOS's inherent security benefits and the `profiles.hardened` option.

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
