---
title: Hardening Networking
date: 2025-12-25
author: saylesss88
collection: "blog"
tags: ["hardening", "networking", "firewall"]
draft: false
---

# Hardening Networking

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

> Since networks and systems vary, some adjustments may cause unexpected issues,
> especially around critical components like DNS or firewalls. Always review and
> test changes in a controlled environment before applying them broadly.

> Understand the trade-offs and tailor the settings to your threat model and
> workflow. Take what’s useful, adapt as needed, and seek expert guidance for
> more advanced scenarios.

## Introduction

Every setup is unique, feel free to adapt or skip sections based on your needs.
Start with the basics and build up as you gain confidence. The goal is
practical, tested hardening tailored to you.

### Safe Browsing / Privacy Enhancing Habits

I recently broke this chapter down and added another chapter:
[Browser/Browsing Security](https://saylesss88.github.io/nix/browsing_security.html)

**Adopt Encrypted DNS and HTTPS Everywhere**

- Configure your system and browsers to use DNS over HTTPS (DoH), DNS over TLS
  (DoT), or DNSCrypt to prevent DNS leakage. Use HTTPS-Only mode in browsers to
  encrypt all web traffic. Prefer browsers with strong privacy defaults or add
  recommended extensions.

- [Privacy Guides dnscrypt-proxy recommendation](https://www.privacyguides.org/en/dns/#dnscrypt-proxy)

- Disable browser "remember password" and autofill features, clear cookies and
  site data upon exit, and carefully vet suspicious URLs with tools like
  [VirusTotal](https://www.virustotal.com/gui/home/url).

**Limit Account Linking and Use Unique Credentials**

- Create separate accounts with unique passwords instead of signing in with
  Google, Facebook, or similar services to limit broad data exposure from
  compromises.

**Use Metadata Cleaning Tools**

- Many files like images, PDFs, and office documents contain hidden metadata
  information such as location data, device details, and more that can reveal
  your identity or other sensitive information when you share files publicly.

- To protect your privacy, always sanitize files by removing this metadata
  before sharing. Tools like [mat2](https://0xacab.org/jvoisin/mat2) are
  designed to strip metadata from a wide range of media files efficiently.
  (`pkgs.mat2`). You just type `mat2 swappy-2025.png` for example and there will
  then be a new `mat2 swappy-2025.cleaned.png` that can safely be shared.

**Use Anonymous File-Sharing Tools**

- For sensitive transfers, consiter tools like
  [OnionShare](https://github.com/onionshare/onionshare) that provide anonymity
  and security.(`pkgs.onionshare`)

**Avoid Scanning Random QR Codes Without Verification**

- Use QR code scanner apps that check for malicious content before loading
  links.

**Understand Your Threat Model**

- Apply these basics universally, but tailor advanced hardening according to
  your unique environment, connectivity needs, and risk profile.

**Delete cookies and site data when the browser is closed**. (security not
usability).

**Use Strong, Unique Passwords and a Password Manager**

- Avoid reused passwords by using reliable password managers like KeePassXC or
  Bitwarden, both available on NixOS. Pair this with enabling two-factor
  authentication **(2FA) wherever possible**.

- It's advisable to only use the desktop version and not the browser extension
  for a number of reasons. One is that you can store your passwords completely
  offline and have complete ownership of them.

```nix
environment.systemPackages = [
    pkgs.keepassxc
    pkgs.kpcli     # KeePass CLI
    # OR
    pkgs.bitwarden-desktop
    pkgs.bitwarden-cli
];
```

With KeePassXC, you can require 3 different authentication methods at the same
time. You can choose a password, a keyfile, and a security key where it won't
open unless all 3 are present giving you additional security. All 3 might not be
necessary but it's possible. It's also easy to migrate to KeePassXC, you can
import your vault from many different managers.

KeepassXC also makes it easy to keep your complete password database offline
which can significantly reduce the risk of a breach.

With Bitwarden, to enable 2 factor authentication, you need to log in with your
master password through the web interface.

- [PrivacyGuides Intro to Passwords](https://www.privacyguides.org/en/basics/passwords-overview/)

---

### Why Follow These Basics?

These recommended steps help protect your privacy and security while maintaining
usability and minimizing system interruptions. They catch common threats like
network eavesdropping, password reuse, fingerprinting, and data leakage,
providing a solid foundation to build on.

A vast majority of secure and privacy-focused browsers available for NixOS are
based on Firefox.

> ❗ NOTE: Firefox does lack some security features available in Chrome and
> sandbox escapes in Linux are relatively easy. People such as madaidan say to
> never use Linux or Firefox period when you're worried about security and
> privacy. I'm not personally going to jump to proprietary software with known
> backdoors in a misguided attempt at increasing security/privacy.

- [EU Hits Google with 3.5 Billion Antitrust](https://techstory.in/eu-hits-google-with-3-5-billion-antitrust-fine-over-adtech-practices/)

This [GrapheneOS article](https://grapheneos.org/usage#web-browsing), breaks
down why they use Chromium-based browsers and specifically mentions that it's
not recommended to use Firefox, especially on Linux because of the weak
sandboxing.

As a Chromium-based browser, Brave has been growing on me. Brave uses
randomization rather than standardization for fingerprinting protection. If you
run Cover Your Tracks with Brave, it will show a randomized fingerprint.

<details>
<summary> ✔️ Click To Expand United States Patriot Act Overview </summary>

[Section 215 USA Patriot Act](https://www.csis.org/analysis/fact-sheet-section-215-usa-patriot-act)
permits the collection of "Tangible Things" or "Business Records", e.g., your
phone records, medical records, etc. for an investigation to obtain foreign
intelligence information. If it does relate to a US person it must be relevant
to preventing terrorism or espionage, and not be based solely on activities
protected by the first amendment. "Relevant" is the key word here and it is at
the governments discretion meaning they sweep everything and sift it later.
Criticized for violating American citizens Fourth Amendment protections against
warrantless search and seizure and proven to be ineffective.

</details>

What is "normal" and allowed today might be suppressed tomorrow, look at the UK
[Online Safety Act](https://en.wikipedia.org/wiki/Online_Safety_Act_2023)
purported to protect children, accused of banning privacy. This is because the
only way to verify age is to make everyone submit KYC with their drivers license
or ID, completely taking away any anonymity of adults and children alike.

Also see
[BBC 4chan refuses to pay fine](https://www.bbc.com/news/articles/cq68j5g2nr1o)

The mere existence of a surveillance state breeds fear and conformity and
stifles free
expression.--[The Intercept](https://theintercept.com/2016/04/28/new-study-shows-mass-surveillance-breeds-meekness-fear-and-self-censorship/)

There are much more scary examples in
[Privacy, The new Oil](https://thenewoil.org/en/guides/prologue/why/)

## Protections from Surveillance in the U.S.

<details>
<summary> ✔️ Click to Expand U.S. Surveillance protections </summary>

> ⚠️ A crucial caveat to keep in mind regarding surveillance protections in the
> U.S., whether grounded in the Fourth Amendment, the First Amendment, or
> statutory laws is that **these protections are not foolproof and have
> repeatedly failed or been circumvented in practice**.

- **Fourth Amendment Basics**: It demands reasonableness in searches and usually
  requires a warrant. This means government agents cannot arbitrarily listen to
  your private communications or search your digital data without judicial
  approval

- **Electronic Surveillance Challenges**: Courts have wrestled with how the
  Fourth Amendment applies to modern communications. The Supreme Court has ruled
  in some cases that pervasive or non-consensual electronic surveillance
  violates reasonable expectations of privacy, but other rulings have allowed
  broader state actions in national security contexts.

- **The Third-Party Doctrine**: A major limitation arises from the "third-party
  doctrine," which holds that information voluntarily shared with third parties
  (like phone companies or internet providers) has reduced Fourth Amendment
  protections. This means data held by third parties may be subject to
  government access without a warrant in some cases

- **The First Amendment** guarantees free speech and the freedom to receive
  information without government censorship or intimidation. Excessive or
  secretive government surveillance can chill free speech by making people
  afraid their communications are monitored, discouraging open expression and
  participation in public discourse.
  - Advocates argue that courts should recognize government surveillance not
    only as a Fourth Amendment search issue but also as a First Amendment
    violation where surveillance suppresses or chills constitutionally protected
    expression.

While the Fourth Amendment traditionally governs searches and surveillance
legality, the First Amendment frames the broader impact on free speech and
democratic engagement. Invoking both provides a more comprehensive
constitutional shield against intrusive surveillance practices.

</details>

---

## Encrypted DNS

DNS (Domain Name System) resolution is the process of translating a website's
domain name into its corresponding IP address. By default, this traffic isn't
encrypted, which means anyone on the network, from your ISP to potential
hackers, can see the websites you're trying to visit. **Encrypted DNS** uses
protocols to scramble this information, protecting your queries and responses
from being intercepted and viewed by others.

> ❗ NOTE: There are many other ways for someone monitoring your traffic to see
> what domain you looked up via DNS that it's effectiveness is questionable
> without also using Tor or a VPN. Encrypted DNS will not help you hide any of
> your browsing activity.

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

**Hot Take**:

- [Encrypted DNS is ineffective without a VPN or Tor by madaidan](https://madaidans-insecurities.github.io/encrypted-dns.html)

</details>

The following sets up dnscrypt-proxy using ODoH (Oblivious DNS over HTTPS) with
an oisd blocklist:

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

<details>
<summary> ✔️ Add more blocklists: HaGeZi Multi PRO </summary>

To use the Hagezi Multi PRO Blocklist either with oisd or alone you could do the
following:

```nix
# flake.nix
inputs = {
    oisd = {
      url = "https://big.oisd.nl/domainswild";
      flake = false;
    };
    hagezi = {
      url = "https://raw.githubusercontent.com/hagezi/dns-blocklists/main/wildcard/pro-onlydomains.txt";
      flake = false;
    };
};
```

add it to the `extraBlocklist` variable in the following `dnscrypt-proxy.nix`:

```nix
# dnscrypt-proxy.nix
extraBlocklist = builtins.readFile inputs.hagezi;
```

More blocklist url's:

```text
# NextDNS CNAME cloaking list
https://raw.githubusercontent.com/nextdns/cname-cloaking-blocklist/master/domains

# AdGuard Simplified Domain Names filter
https://adguardteam.github.io/AdGuardSDNSFilter/Filters/filter.txt

# OISD Big list
https://big.oisd.nl/domainswild

# HaGeZi Multi PRO
https://raw.githubusercontent.com/hagezi/dns-blocklists/main/wildcard/pro-onlydomains.txt

# HaGeZi Threat Intelligence Feeds
https://raw.githubusercontent.com/hagezi/dns-blocklists/main/wildcard/tif-onlydomains.txt
```

</details>

> ❗ NOTE: The `oisd` blocklist is a plain text file that updates frequently.
> This can cause `nh os switch` to fail with a `NarHash` mismatch error. To fix
> this, you need to run `nix flake update` to refresh the blocklist and its hash
> in your `flake.lock` file. After that, you can run your `nh` command again.

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
  extraBlocklist = "";
  blocklist_txt = pkgs.writeText "blocklist.txt" ''
    ${extraBlocklist}
    ${blocklist_base}
  '';
  hasIPv6Internet = true;
  StateDirName = "dnscrypt-proxy"; # Used for systemd StateDirectory
  StatePath = "/var/lib/${StateDirName}";
in {
  networking = {
    nameservers = ["127.0.0.1" "::1"];
    networkmanager.dns = "none";
  };

  services.resolved.enable = lib.mkForce false;

  services.dnscrypt-proxy = {
    enable = true;
    settings = {
      sources.public-resolvers = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/public-resolvers.md"
          "https://download.dnscrypt.info/resolvers-list/v3/public-resolvers.md"
        ];
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
        cache_file = "${StatePath}/public-resolvers.md";
      };

      sources.relays = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/relays.md"
          "https://download.dnscrypt.info/resolvers-list/v3/relays.md"
        ];
        cache_file = "${StatePath}/relays.md";
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
      };

      sources.odoh-servers = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/odoh-servers.md"
          "https://download.dnscrypt.info/resolvers-list/v3/odoh-servers.md"
        ];
        cache_file = "${StatePath}/odoh-servers.md";
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
      };

      sources.odoh-relays = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/odoh-relays.md"
          "https://download.dnscrypt.info/resolvers-list/v3/odoh-relays.md"
        ];
        cache_file = "${StatePath}/odoh-relays.md";
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
      };

      server_names = ["odoh-cloudflare" "odoh-snowstorm"];

      # This creates the [anonymized_dns] section in dnscrypt-proxy.toml
      anonymized_dns = {
        skip_incompatible = true;
        routes = [
          {
            server_name = "odoh-snowstorm";
            via = ["odohrelay-crypto-sx"];
          }
          {
            server_name = "odoh-cloudflare";
            via = ["odohrelay-crypto-sx"];
          }
        ];
      };

      ipv6_servers = hasIPv6Internet;
      block_ipv6 = !hasIPv6Internet;
      blocked_names.blocked_names_file = "${blocklist_txt}";
      require_dnssec = true;
      require_nolog = false;
      require_nofilter = false;
      odoh_servers = true;
      dnscrypt_servers = true;
    };
  };

  # This creates /var/lib/dnscrypt-proxy with correct permissions
  systemd.services.dnscrypt-proxy2.serviceConfig.StateDirectory = StateDirName;
}
```

This module follows a "Zero Trust" model for your internet traffic, ensuring no
single entity can see both **who you are** and **where you are going**.

```bash
# You should see that dnscrypt-proxy chooses the Server with the lowest initial latency
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

The system's DNS settings (`networking.nameservers`) point to localhost, so
**all DNS queries** go to dnscrypt-proxy accept for your browser. Your browser
has to be configured separately with a local resolver in which I haven't figured
out yet. I recommend setting your browsers DNS over HTTPS to strict with a
respected custom DNS resolver such as `https://dns.quad9.net/dns-query`.

`inputs.oisd` refers to the flake input oisd blocklist, it prevents your device
from connecting to unwanted or harmful domains.

`dnscrypt-proxy2` then encrypts and forwards our DNS requests to third-party
public DNSCrypt or DoH servers.

- ODoH Relays: This is the "Oblivious" part. It breaks the link between your IP
  address and your browsing history.

### Setting up Tailscale

I was surprised at how easy this actually was to set up. Either go to
<https://www.tailscale.com> and/or download the app for either Android or IOS,
sign up with your identity provider, and click `Start connecting devices ->`

- [Tailscale quickstart](https://tailscale.com/kb/1017/install)

To add tailscale to NixOS:

```nix
# tailscale.nix
{...}: {
  services.tailscale.enable = true;
  # Tell the firewall to implicitly trust packets routed over Tailscale:
  networking.firewall.trustedInterfaces = ["tailscale0"];
}
```

Tailscale will automatically use the hostname of your device as the name of the
network. If you want to change it to something else:

```bash
sudo tailscale set --hostname=<name>
# You can also give your account a nickname
sudo tailscale set --nickname=<name>
```

This allows you to refer to your network by `name` rather than IP address.

Tailscale uses [MagicDNS](https://tailscale.com/kb/1081/magicdns) which is
enabled by default, and they recommend you keep it enabled.

The docs say that by default, devices in your tailnet prefer their local DNS
settings and only use the tailnet's DNS servers when needed. I had to completely
disable my Androids DNS settings for tailscale to access the internet through
MagicDNS.

```bash
sudo tailscale set --accept-dns=false
```

To connect to tailscale after rebuilding you can run:

```bash
sudo tailscale up
```

Use `nslookup` to review and debug DNS responses:

```bash
nslookup google.com
Server:         127.0.0.1
Address:        127.0.0.1#53

Non-authoritative answer:
Name:   google.com
Address: 142.251.40.206
Name:   google.com
Address: 2a00:1450:4001:827::200e
```

- The `127.0.0.1#53` indicate that instead of using the DNS server pushed by
  your ISP, router, or Tailscale's MagicDNS, the system is sending all DNS
  requests through the loopback device to `dnscrypt-proxy` in my case.

Get the status of your connections to other Tailscale devices:

```bash
tailscale status
1           2         3           4         5
100.1.2.3   device-a  apenwarr@   linux     active; direct <ip-port>, tx 1116 rx 1124
100.4.5.6   device-b  crawshaw@   macOS     active; relay <relay-server>, tx 1351 rx 4262
100.7.8.9   device-c  danderson@  windows   idle; tx 1214 rx 50
100.0.1.2   device-d  ross@       iOS       —
```

- [Tailscale Best Practices](https://tailscale.com/kb/1196/security-hardening)

- [Tailscale CLI](https://tailscale.com/kb/1080/cli)

- There is much more you can do with Tailscale, including integrating
  Mullvad-VPN and using Exit Nodes.

## MAC Randomization

All network cards have a unique identifier called a MAC address. They're stored
in hardware and are used to assign an address to computers on the local network.

The MAC address is typically only traceable on the local network, it's not
passively sent out beyond the local router making it more critical on untrusted,
public networks.

Leak-proof MAC randomization is very difficult to implement:

- [Leak-proof MAC Randomization Implementation Challenges](https://www.kicksecure.com/wiki/Dev/MAC#Leak-proof_MAC_Randomization_-_Technical_Implementation_Challenges)

Android and iPhone already implement MAC Randomization by default.

MAC Randomization enhances privacy by making it harder for third parties to
track users across different networks.

Randomizing MAC adresses obscures a device's unique hardware identity when
scanning for or connecting to Wi-Fi, blocking passive tracking as well as
location tracking across networks.

If you use NetworkManager you can set MAC randomization with:

```nix
    networking = {
      networkmanager = {
        enable = true;
        wifi.scanRandMacAddress = true;
        wifi.macAddress = "random";
        plugins = [];
      };
```

Right when I rebuilt, I got an alert from my router saying that a new device
just connected to the network.

There is also a utility for viewing/manipulating the MAC address of network
interfaces, `pkgs.macchanger`. This is less reliable than the NetworkManager
setting.

## Firewalls

NixOS includes an integrated firewall based on iptables/nftables.

<details>
<summary> ✔️ Click to Expand Firewall Resources </summary>

[Cloudflare What is a Firewall](https://www.cloudflare.com/learning/security/what-is-a-firewall/)

[Beginners guide to nftables](https://linux-audit.com/networking/nftables/nftables-beginners-guide-to-traffic-filtering/)

[Arch Wiki nftables](https://wiki.archlinux.org/title/Nftables)

</details>

The following firewall setup is based on the dnscrypt setup above utilizing
nftables.

This nftables firewall configuration is a strong recommended practice for
enforcing encrypted DNS on your system by restricting all outbound DNS traffic
to a local dnscrypt-proxy process. It greatly reduces DNS leak risks and
enforces privacy by limiting DNS queries to trusted, encrypted upstream
servers.(This was edited on 08-08-25) replace `<DNSCRYPT-UID>` with the UID
given from the command `ps -o uid,user,pid,cmd -C dnscrypt-proxy`:

```nix
{ ... }: {
  networking.nftables = {
    enable = true;

    ruleset = ''
      table inet filter {
        chain output {
          type filter hook output priority 0; policy accept;

          # Allow localhost DNS for dnscrypt-proxy2
          ip daddr 127.0.0.1 udp dport 53 accept
          ip6 daddr ::1 udp dport 53 accept
          ip daddr 127.0.0.1 tcp dport 53 accept
          ip6 daddr ::1 tcp dport 53 accept

          # Allow dnscrypt-proxy2 to talk to upstream servers
          # Replace <DNSCRYPT-UID> with:
          # ps -o uid,user,pid,cmd -C dnscrypt-proxy
          meta skuid <DNSCRYPT-UID> udp dport { 443, 853 } accept
          meta skuid <DNSCRYPT-UID> tcp dport { 443, 853 } accept

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
      # Ports open for inbound connections.
      # Limit these to reduce the attack surface.

      22 # SSH – Keep open only if you need remote access.
         # To change the SSH port in NixOS:
         # services.openssh.ports = [ 2222 ];
         # Update this list to match the new port.

      # 53  # DNS – Only if running a public DNS server.
      # 80  # HTTP – Only if hosting a website.
      # 443 # HTTPS – Only if hosting a secure website.
    ];
    allowedUDPPorts = [
      # Ports open for inbound UDP traffic.
      # Most NixOS workstations won't need any here.

      # 53 # DNS – Only if running a public DNS server.
    ];
  };
}
```

<details>
<summary> ✔️ Click to Expand Tip on changing the default SSH Port </summary>

> ❗ TIP: Reduce SSH noise by changing the default port On most systems, SSH
> listens on TCP port 22 — which means automated bots and scanners will hit it
> constantly. While this doesn’t replace real security measures, moving SSH to a
> different port drastically cuts down on drive-by brute-force attempts you’ll
> see in your logs.
>
> In NixOS, change both the SSH daemon port and your firewall rule:
>
> ```nix
>  # Example: Move SSH to port 2222
>  networking.firewall.allowedTCPPorts = [ 2222 ];
>  services.openssh.ports = [ 2222 ];
> ```
>
> - After rebuilding, test from another terminal/session before closing your
>   existing one:
>
> ```bash
> ssh -p 2222 user@host
> ```

</details>

`nft` is a cli tool used to set up, maintain and inspect packet filtering and
classification rules in the Linux kernel, in the nftables framework. The Linux
kernel subsystem is known as nftables, and 'nf' stands for Netfilter.--`man nft`

```bash
sudo nft list ruleset
```

- Since we declare our firewall, we'll only use `nft` to inspect our ruleset.

## NixOS Firewall vs `nftables` Ruleset

`networking.nftables`: This section provides a raw `nftables` ruleset that gives
you granular, low-level control. The rules here are more specific and are meant
to handle the intricate logic of the DNS proxy setup. They will be applied
directly to the kernel's `nftables` subsystem and prevent DNS leaks.

`networking.firewall`: This is a higher-level, simpler NixOS option that uses
`iptables` rules to open ports for inbound traffic. The rules defined here
(allowing port 22) is for incoming SSH connections to the machine, not for
outbound traffic, so they do not interfere with the `nftables` rules that filter
the outgoing traffic. (Make sure to comment out or remove this if you don't SSH
into your machine).

The firewall ensures only authorized, local encrypted DNS proxy process can
speak DNS with the outside world, and that all other DNS requests from any other
process are blocked unless they're to `127.0.0.1` (our local proxy). This is a
robust policy against both DNS leaks and local compromise.

## Testing

Review listening ports: After each rebuild, use `ss -tlpn`, `nmap` or `netstat`
to see which services are accepting connections. Close or firewall anything
unnecessary.

You can also test firewall DNS restrictions using `dig`:

```bash
dig @127.0.0.1 example.com  # Should work

dig @8.8.8.8 example.com    # Should fail/time out for normal users
```

- This test is actually what alerted me of an improper configuration in the
  above firewalls nftables rules allowing me to fix it. Initially the second
  `dig` command gave results letting me know that the restrictions weren't being
  applied correctly.

Since we defined an `output` chain inside `table inet filter` with the line:

```bash
type filter hook output priority 0; policy accept;
```

This attaches the chain to the kernel’s OUTPUT hook, so all locally generated
packets, including DNS queries are filtered by this chain.

Within this chain, the rules:

- Explicitly allow DNS queries to localhost addresses (`127.0.0.1` and `::1`).

- Allow the `dnscrypt-proxy` process (running with UID `62396`) to send DNS
  queries on ports 443 and 853 (for DNS-over-HTTPS and DNS-over-TLS).

- Drop all other outbound DNS traffic on ports `53` and `853`.

Because of this setup, dig queries to your local resolver at `127.0.0.1` pass,
but queries directly to public DNS servers like `8.8.8.8` are blocked for
users/processes other than the allowed DNS proxy.

## OpenSnitch

- [NixOS Wiki OpenSnitch](https://wiki.nixos.org/wiki/OpenSnitch)

[Opensnitch](https://github.com/evilsocket/opensnitch) is an open-source
application firewall that focuses on monitoring and controlling outgoing network
connections on a per-application basis.

This can be used to block apps from accessing the internet that shouldn't need
to (i.e., block telemetry and more). Opensnitch will report that the app has
attempted to make an outbound internet connection and block it or allow it based
on the rules you set.

### Resources

<details>
<summary> ✔️ Click to Expand Resources </summary>

- [Cloudflare What is HTTPS](https://cloudflare.com/learning/ssl/what-is-https)

- [Surveillance Self-Defence](https://ssd.eff.org/) has a lot of helpful info to
  protect your privacy.

- [What is Fingerprinting](https://ssd.eff.org/module/what-fingerprinting), more
  than you realize is being tracked constantly.

- [oisd.nl](https://oisd.nl/) the oisd website

- For potentially dangerous file types like PDFs, office documents, or images,
  especially those downloaded from untrusted sources such as torrents, consider
  converting them to a safe PDF format with
  [dangerzone](https://github.com/freedomofpress/dangerzone). Dangerzone not
  only removes metadata but also applies robust sanitization to neutralize
  malicious content.

- [NixOS Wiki LibreWolf](https://wiki.nixos.org/wiki/Librewolf), the options in
  the wiki make it less secure and aren't recommended settings to use. They
  explicitly disable several of LibreWolf's default privacy-enhancing features,
  such as fingerprinting resistance and clearing session data on shutdown.

- [LibreWolf Features](https://librewolf.net/docs/features/) You still need to
  enable DNS over HTTPS through privacy settings.

- [SearXNG on NixOS](https://wiki.nixos.org/wiki/SearXNG)
  - [Welcome to SearXNG](https://docs.searxng.org/)

- [Firefox Hardening Guide](https://brainfucksec.github.io/firefox-hardening-guide)

- [Firefox ghacks](https://www.ghacks.net/2015/08/18/a-comprehensive-list-of-firefox-privacy-and-security-settings/)

- [Arkenfox](https://github.com/arkenfox/user.js)

- [PrivacyTools.io](https://www.privacytools.io/private-browser)

- [simeononsecurity Firefox-Privacy-Script](https://github.com/simeononsecurity/FireFox-Privacy-Script)

- [brianfucksec firefox-hardening-Guide 2023](https://brainfucksec.github.io/firefox-hardening-guide)

- [STIG Firefox Hardening](https://simeononsecurity.com/guides/enhance-firefox-security-configuring-guide/)

> If you should trust the U.S. Governments recommendations is another story but
> it can be good to compare and contrast with other trusted resources. You'll
> have to think whether the CISA recommending that everyone uses Signal is solid
> advice or guiding you towards a honeypot, I can't say for sure.

- [Mozilla Firefox Security Technical Implementation Guide](https://stigviewer.com/stigs/mozilla_firefox)
  The STIG for Mozilla Firefox (Security Technical Implementation Guide) is a
  set of security configuration standards developed by the U.S. Department of
  Defense. They are created by the Defense Information Systems Agency (DISA) to
  secure and harden DoD information systems and software.

- [Privacy, The New Oil (Why Privacy & Security Matter)](https://thenewoil.org/en/guides/prologue/why/)

- [PrivacyGuides](https://www.privacyguides.org/en/)

- [Firefox Relay](https://relay.firefox.com/accounts/profile/) can be used to
  create email aliases that forward to your real email address. The paid plan
  also lets you create phone number aliases that forward to your phone number.

- [Zebra Crossing digital safety checklist](https://zebracrossing.narwhalacademy.org/)

- [DataDetoxKit](https://datadetoxkit.org/en/privacy/essentials#step-1)

- [DataDetox Degooglise](https://datadetoxkit.org/en/privacy/degooglise/)

- [Tor Browser User Manual](https://tb-manual.torproject.org/)

- [Tor Wiki](https://gitlab.torproject.org/tpo/team/-/wikis/home)

- [Linux Network Administrators Guide](https://tldp.org/LDP/nag2/x-087-2-intro.html)

- [IBM Networking](https://www.ibm.com/think/topics/networking)

</details>
