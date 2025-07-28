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
> also performance tradeoffs with added protection.

Containers and VMs are beyond the scope of this chapter but can also enhance
security if configured correctly.

## Minimal Installation with LUKS

Begin with NixOS’s minimal installation image. This gives you a base system with
only essential tools and no extras that could introduce vulnerabilities.

Use LUKS encryption to protect your data at rest, the following guide is a
minimal disko encrypted installation:
[Encrypted Install](https://saylesss88.github.io/installation/enc/enc_install.html)

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
`sops-nix`, which lets you keep encrypted secrets under version control
declaratively.

Protect your sectets, the following guide is on setting up Sops on NixOS:
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)

## Hardening the Kernel

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

Check all `sysctl` parameters:

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

**Or** you can harden the kernel you're using:

```nix
  boot.kernel.sysctl = {
    # The Magic SysRq key is a key combo that allows users connected to the
    # system console of a Linux kernel to perform some low-level commands.
    # Disable it, since we don't need it, and is a potential security concern.
    "kernel.sysrq" = 0;
    # prevent unintentional writes to an attacker-controlled FIFO, 2 applies to group writable sticky directories.
    "fs.protected_fifos" = 2;
    # avoids writes to an attacker-controlled regular file, where a program is expected to create one
    "fs.protected_regular" = 2;
    # prevent memory dumps of potentially sensitive info
    "fs.suid_dumpable" = false;
    # restrictions on exposing kernel addresses via /proc, with 2, pointers are replaced with 0's
    "kernel.kptr_restrict" = 2;
    # Note: certian container runtimes or browser sandboxes might rely on the following
    # Prevents BTI and BHI attacks
    "kernel.unprivileged_bpf_disabled" = true;

    ## TCP hardening
    # Prevent bogus ICMP errors from filling up logs.
    "net.ipv4.icmp_ignore_bogus_error_responses" = 1;
    # Reverse path filtering causes the kernel to do source validation of
    # packets received from all interfaces. This can mitigate IP spoofing.
    "net.ipv4.conf.all.forwarding" = 0;
    "net.ipv4.conf.default.rp_filter" = 1;
    "net.ipv4.conf.all.rp_filter" = 1;
    # Do not accept IP source route packets (we're not a router)
    "net.ipv4.conf.all.accept_source_route" = 0;
    "net.ipv6.conf.all.accept_source_route" = 0;
    "net.ipv6.conf.all.forwarding" = 0;
    # Don't send ICMP redirects (again, we're not a router)
    "net.ipv4.conf.all.send_redirects" = 0;
    "net.ipv4.conf.default.send_redirects" = 0;
    # "net.core.bpf_jit_harden" = 2;
    # Refuse ICMP redirects (MITM mitigations)
    "net.ipv4.conf.all.accept_redirects" = 0;
    "net.ipv4.conf.default.accept_redirects" = 0;
    "net.ipv4.conf.all.secure_redirects" = 0;
    "net.ipv4.conf.default.secure_redirects" = 0;
    "net.ipv6.conf.all.accept_redirects" = 0;
    "net.ipv6.conf.default.accept_redirects" = 0;
    # Protects against SYN flood attacks
    "net.ipv4.tcp_syncookies" = 1;
    # Incomplete protection again TIME-WAIT assassination
    "net.ipv4.tcp_rfc1337" = 1;
    # disable unprivileged user namespaces, Note: Docker, and other apps may need this
    "kernel.unprivileged_userns_clone" = 0;
    # memory protection (64-bit systems)
    "vm.mmap_rnd_bits" = 32;
    # Randomize memory
    "kernel.randomize_va_space" = 2;
    # Exec Shield (Stack protection)
    "kernel.exec-shield" = 1;
    "kernel.randomize_va_space" = 2;


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
attempted to leave comment warnings.

## Best Practices

**Explicitly enable each service**: In your `configuration.nix`, only enable
networking, SSH, desktop environments, and applications as needed. Remove or
avoid legacy daemons and sample services.

**Principle of Least Privilege Limit installed software**: Each program or
service added is potential attack surface. Install packages individually rather
than enabling broad module imports or convenience meta-packages.

**Run services as unprivileged users**: Wherever possible, configure system
services to run with a dedicated user and group, not as root.

**Use NixOS’s fine-grained service options**: For example, set systemd
sandboxing options (ProtectHome, PrivateTmp, NoNewPrivileges), and use NixOS
modules’ user/group settings for daemons.

**Secure the Boot & Init Process Enable Secure Boot**: Use modules like
lanzaboote to enforce EFI Secure Boot, ensuring only signed kernels are loaded.

**Encrypt your root and data partitions**: Use LUKS to encrypt your partitions,
some even encrypt their swap.

Keep the Attack Surface Small Disable unused features and daemons: Comment out
or set `enable = false;` for modules like CUPS, Samba, avahi, etc., if you don’t
need printing, filesharing, or zeroconf networking.

**Use HTTPS**: This one is simple but has big benifits, there is usually an
extension or setting for this on most browsers. It ensures that all data
exchanged between your browser and the website you're visiting is encrypted.
This means that if it's intercepted, they won't be able to read your data.

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
  legacy/service log files.

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

## Hardening Networking

## Encrypted DNS

DNS (Domain Name System) resolution is the process of translating a website's
domain name into its corresponding IP address. By default, this traffic isn't
encrypted, which means anyone on the network, from your ISP to potential
hackers, can see the websites you're trying to visit. **Encrypted DNS** uses
protocols to scramble this information, protecting your queries and responses
from being intercepted and viewed by others.

There are 3 main types of DNS protection:

- **DNS over HTTPS (DoH)**: Uses the HTTPS protocol to encrypt data between the
  client and the resolver.

- **DNS over TLS (DoT)**: Similar to (DoH), differs in the methods used for
  encryption and delivery using a separate port from HTTPS.

- **DNSCrypt**: Uses end-to-end encryption with the added benefit of being able
  to prevent DNS spoofing attacks.

Useful resources:

<details>
<summary> ✔️ Click to Expand DNS Resources </summary>

- [NixOS Wiki Encrypted DNS](https://wiki.nixos.org/wiki/Encrypted_DNS)

- [Domain Name System (DNS)](https://www.cloudflare.com/learning/dns/what-is-dns/)

- [Wikipedia DNS over HTTPS (DoH)](https://en.wikipedia.org/wiki/DNS_over_HTTPS)

- [Wikipedia DNS over TLS (DoT)](https://en.wikipedia.org/wiki/DNS_over_TLS)

- [Cloudflare Dns Encryption Explained](https://blog.cloudflare.com/dns-encryption-explained/)

- [NordVPN Encrypted Dns Traffic](https://nordvpn.com/blog/encrypted-dns-traffic/)

</details>

The following sets up dnscrypt-proxy using DoH (DNS over HTTPS) with an oisd
blocklist, they both come directly from the Wiki:

Add `oisd` to your flake inputs:

```nix
# flake.nix
inputs = {
    oisd = {
      url = "https://big.oisd.nl/domainswild";
      flake = false;
    };
};
```

And the import the following into your `configuration.nix`:

```nix
# dnscrypt-proxy.nix
{
  pkgs,
  lib,
  inputs,
  ...
}: let
  blocklist_base = builtins.readFile inputs.oisd;
  extraBlocklist = '''';
  blocklist_txt = pkgs.writeText "blocklist.txt" ''
    ${extraBlocklist}
    ${blocklist_base}
  '';
  hasIPv6Internet = true;
  StateDirectory = "dnscrypt-proxy";
in {
  networking = {
    # Set DNS nameservers to the local host addresses for iPv4 (`127.0.0.1`) & iPv6 (::1)
    nameservers = ["127.0.0.1" "::1"];
    # If using dhcpcd
    # dhcpcd.extraConfig = "nohook resolv.conf";
    # If using NetworkManager
    networkmanager.dns = "none";
  };
  services.resolved.enable = lib.mkForce false;
  # See https://wiki.nixos.org/wiki/Encrypted_DNS
  services.dnscrypt-proxy2 = {
    enable = true;
    # See https://github.com/DNSCrypt/dnscrypt-proxy/blob/master/dnscrypt-proxy/example-dnscrypt-proxy.toml
    settings = {
      sources.public-resolvers = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/public-resolvers.md"
          "https://download.dnscrypt.info/resolvers-list/v3/public-resolvers.md"
        ];
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3"; # See https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v3/public-resolvers.md
        cache_file = "/var/lib/${StateDirectory}/public-resolvers.md";
      };

      # Use servers reachable over IPv6 -- Do not enable if you don't have IPv6 connectivity
      ipv6_servers = hasIPv6Internet;
      block_ipv6 = ! hasIPv6Internet;
      blocked_names.blocked_names_file = blocklist_txt;
      require_dnssec = true;
      require_nolog = false;
      require_nofilter = true;

      # If you want, choose a specific set of servers that come from your sources.
      # Here it's from https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v3/public-resolvers.md
      # If you don't specify any, dnscrypt-proxy will automatically rank servers
      # that match your criteria and choose the best one.
      # server_names = [ ... ];
    };
  };

  systemd.services.dnscrypt-proxy2.serviceConfig.StateDirectory = StateDirectory;
}
```

```bash
sudo systemctl status dnscrypt-proxy2
# verify that dnscrypt-proxy is listening
sudo ss -lnp | grep 53
# Test a DNS query, if you get valid responses it's working
dig @127.0.0.1 example.com +short
# check the logs
sudo journalctl -u dnscrypt-proxy2
```

`dnscrypt-proxy2` acts as your local DNS resolver listening on your machine
(`127.0.0.1`) for IPv4 and `::1` for iPv6.

`inputs.oisd` refers to the flake input oisd blocklist, it prevents your device
from connecting to unwanted or harmful domains.

- [oisd.nl](https://oisd.nl/) the oisd website

`dnscrypt-proxy2` filters ads/trackers (using oisd), enforces DNSSEC, and uses
encrypted transports (DNS-over-HTTPS/DoH, DNSCrypt, optionally
DNS-over-TLS/DoT).

## Proxy Servers

<details>
<summary> ✔️ Click to Expand section on Proxy Servers </summary>

Proxy servers let you control, monitor, or anonymize network traffic between
clients and the wider internet. In NixOS, you can set up various types of
proxies (HTTP, SOCKS, DNS, etc.) declaratively in your system config.

Types of Proxy Servers: HTTP/HTTPS Forward Proxy, Controls and filters outbound
web traffic from client machines (e.g., for content filtering or caching).

SOCKS Proxy: Works for all TCP traffic, commonly used for anonymity or routing
through Tor.

Reverse Proxy: Handles incoming web traffic to one or more backend services
(usually handled by NGINX, Apache, Caddy).

Popular Proxy Packages on NixOS:

Squid (caching HTTP proxy)

Privoxy (privacy-enhancing HTTP proxy; can chain with Tor)

shadowsocks-libev (SOCKS5 proxy for privacy/bypassing censorship)

3proxy (lightweight multiprotocol proxy)

Tor (SOCKS5 proxy with strong anonymity)

TODO: Provide a Proxy Server Example

</details>

## Firewalls

[Cloudflare What is a Firewall](https://www.cloudflare.com/learning/security/what-is-a-firewall/)

NixOS includes an integrated firewall based on iptables/nftables.

[Beginners guide to nftables](https://linux-audit.com/networking/nftables/nftables-beginners-guide-to-traffic-filtering/)

[Arch Wiki nftables](https://wiki.archlinux.org/title/Nftables)

The following firewall setup is based on the dnscrypt setup above utilizing
nftables:

```nix
{...}: {
  networking.nftables = {
    enable = true;

    ruleset = ''
      table inet filter {
        chain output {
          # Allow localhost DNS for dnscrypt-proxy2
          ip daddr 127.0.0.1 udp dport 53 accept
          ip6 daddr ::1 udp dport 53 accept
          ip daddr 127.0.0.1 tcp dport 53 accept
          ip6 daddr ::1 tcp dport 53 accept
          # Allow dnscrypt-proxy2 to talk to upstream
          # ps -o uid,user,pid,cmd -C dnscrypt-proxy; Copy UID #
          meta skuid 62396 udp dport { 443, 853 } accept
          meta skuid 62396 tcp dport { 443, 853 } accept
          # Block all other outbound DNS
          udp dport { 53, 853 } drop
          tcp dport { 53, 853 } drop
        }
      }
    '';
  };

  networking.firewall = {
    enable = true;
    allowedTCPPorts = [
      53 # DNS
      22 # SSH
      80 # HTTP
      443 # HTTPS
    ];
    allowedUDPPorts = [
      53 # DNS
    ];
  };
}
```

The firewall ensures only your authorized, local encrypted DNS proxy process can
speak DNS with the outside world, and that all other DNS requests from any other
process are blocked unless they're to `127.0.0.1` (our local proxy). This is a
robust policy against both DNS leaks and local compromise.

Review listening ports: After each rebuild, use `ss -tlpn` or `netstat` to see
which services are accepting connections. Close or firewall anything
unnecessary.

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

```bash
nix-shell -p usbguard
```

```bash
sudo usbguard generate-policy > ~/usbguard-rules.conf
```

> 🚧 Make sure not to just enable this, you need to set up rules or you can end
> up with some persistent problems.

Control USB/Removable access: Use `services.usbguard` to restrict which USB
devices are accepted. Be particularly careful if your authentication keyfiles
are on USB devices.

Usbguard can whitelist wanted usb devices and block the rest. Be careful here,
don't just enable it without adding rules.

```bash
sudo usbguard generate-policy > /etc/usbguard/rules.conf
```

For example:

```nix
{pkgs, ...}: {
  environment.systemPackages = [pkgs.usbguard-notifier];
  services.usbguard = {
    enable = true;
    rules = ''
      allow id 1d6b:0002 serial "0000:05:00.3" name "xHCI Host Controller" hash "4a4NgfdUaJO43rkCzmWRSeHHR/uUh5+SNsXnhosm9qs=" parent-hash "ldMchY4Tt4GPUYo30eNGvai+Fs/EdnVY3vMyxJUq4Nk=" with-interface 09:00:00 with-connect-type ""
      allow id 1d6b:0003 serial "0000:05:00.3" name "xHCI Host Controller" hash "d+DNGWARDtv9nEK2ZvnNOCtFernuMu5/e/oZ7kCppqQ=" parent-hash "ldMchY4Tt4GPUYo30eNGvai+Fs/EdnVY3vMyxJUq4Nk=" with-interface 09:00:00 with-connect-type ""
      # Add default policy
      block unknown
    '';
    # Optional: Configure these as needed for your security posture
    presentDevicePolicy = "apply-policy"; # Or "keep"
    IPCAllowedGroups = ["usbguard" "wheel"]; # If you want wheel group to manage
  };

  # If your user needs to interact with usbguard (e.g., via usbguard-cli)
  users.users.jr.extraGroups = ["usbguard"];
}
```

Further Reading:

- [Wikipedia BadUSB](https://en.wikipedia.org/wiki/BadUSB)

- [USBGuard](https://usbguard.github.io/)

- [NixCraft USBGuard](https://www.cyberciti.biz/security/how-to-protect-linux-against-rogue-usb-devices-using-usbguard/)

## SeLinux/AppArmor MAC (Mandatory Access Control)

**AppArmor**: Stable, supported, easier for most users; enable with one line,
but profile coverage may be incomplete. From my understanding the main issue is
that there are no default profiles so you have to write your own and since
apparmor.d isn't fully supported it makes it a bit more complicated.

I was able to get it configured for `sshd` with the following:

```nix

{
  pkgs,
  lib,
  config,
  ...
}: {
  # Enable AppArmor support in D-Bus
  services.dbus.apparmor = "enabled";
  security = {
    apparmor = {
      enable = true;
      enableCache = true;
      killUnconfinedConfinables = true;

      # Only need packages that provide real, used profiles and tools
      packages = with pkgs; [apparmor-utils apparmor-profiles];

      includes = {
        "abstractions/base" = ''
          /nix/store/*/bin/** mr,
          /nix/store/*/lib/** mr,
          /nix/store/** r,
          ${pkgs.coreutils}/bin/* rix,
          ${pkgs.coreutils-full}/bin/* rix,
        '';
      };

      # Example starter policies
      policies = {
        sshd = {
          profile = ''
            #include <tunables/global>
            /run/current-system/sw/bin/sshd {
              /nix/store/** rix,
              # ...
            }
          '';
          # Optionally, you may be able to add (if supported):
          # enforce = true;
          # enable = true;
        };

      };
    };
  };

  environment.systemPackages = with pkgs; [
    apparmor-utils
    apparmor-parser
    apparmor-profiles
    # Optional: community/contrib profiles you intend to use
    # roddhjav-apparmor-rules # incomplete apparmor.d
  ];

  # If you want PAM integration (useful)
  security.pam = {
    services.sshd.enableAppArmor = true;
  };
}
```

```bash
sudo aa-status
apparmor module is loaded.
1 profiles are loaded.
1 profiles are in enforce mode.
   /run/current-system/sw/bin/sshd
0 profiles are in complain mode.
0 profiles are in prompt mode.
0 profiles are in kill mode.
0 profiles are in unconfined mode.
0 processes have profiles defined.
0 processes are in enforce mode.
0 processes are in complain mode.
0 processes are in prompt mode.
0 processes are in kill mode.
0 processes are unconfined but have a profile defined.
0 processes are in mixed mode.
```

**SELinux**: Experimental, not fully integrated, recent progress for
advanced/curious users; expect rough edges and manual intervention if you want
to try it. Most find SELinux more complex to configure and maintain than
AppArmor.

This isn't meant to be a comprehensive guide, more to get people thinking about
security on NixOS.

## Resources

- [AppArmor and apparmor.d on NixOS](https://hedgedoc.grimmauld.de/s/hWcvJEniW#)

- [SELinux on NixOS](https://tristanxr.com/post/selinux-on-nixos/)

- [Paranoid NixOS](https://xeiaso.net/blog/paranoid-nixos-2021-07-18/)

- [NixOS Wiki Security](https://wiki.nixos.org/wiki/Security)

- [Luks Encrypted File Systems](https://nixos.org/manual/nixos/unstable/index.html#sec-luks-file-systems)

- [Discourse A Modern and Secure Desktop](https://discourse.nixos.org/t/a-modern-and-secure-desktop-setup/41154)

- [notashelf NixOS Security 1 Systemd](https://notashelf.dev/posts/insecurities-remedies-i)

- [ryanseipp hardening-nixos](https://ryanseipp.com/post/hardening-nixos/)
