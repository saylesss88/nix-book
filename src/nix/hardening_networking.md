# Hardening Networking

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

> ⚠️ While I am not a security expert, I have carefully researched and tested
> the configurations in this chapter. The advice presented here is grounded in
> widely accepted best practices and is generally safe and effective.

> However, since networks and systems vary, some adjustments may cause
> unexpected issues, especially around critical components like DNS or
> firewalls. Always review and test changes in a controlled environment before
> applying them broadly.

> Understand the trade-offs and tailor the settings to your threat model and
> workflow. Take what’s useful, adapt as needed, and seek expert guidance for
> more advanced scenarios.

## Introduction

This chapter offers a comprehensive set of network and privacy hardening
practices suitable for Linux, especially NixOS. You can confidently implement
the entire guide to strengthen your security and privacy.

That said, every setup is unique, feel free to adapt or skip sections based on
your needs. Start with the basics and build up as you gain confidence. The goal
is practical, tested hardening tailored to you.

### Practice Safe Browsing Hygiene

**Adopt Encrypted DNS and HTTPS Everywhere**

- Configure your system and browsers to use DNS over HTTPS (DoH), DNS over TLS
  (DoT), or DNSCrypt to prevent DNS leakage. Use HTTPS-Only mode in browsers to
  encrypt all web traffic. Prefer browsers with strong privacy defaults or add
  recommended extensions.

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
  (`pkgs.mat2`)

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

```nix
environment.systemPackages = [
    pkgs.keepassxc
    pkgs.kpcli     # KeePass CLI
    # OR
    pkgs.bitwarden-desktop
    pkgs.bitwarden-cli
];
```

I’ve never agreed with the argument, "I’m not doing anything illegal, so I don’t
mind if they spy on me and profit from my data." Whatever your online activities
may be, your privacy is your right alone. Why make it easier for others to
access your personal information when you have the power to limit your exposure?

### Why Follow These Basics?

These recommended steps help protect your privacy and security while maintaining
usability and minimizing system interruptions. They catch common threats like
network eavesdropping, password reuse, fingerprinting, and data leakage,
providing a solid foundation to build on.

A vast majority of secure and privacy-focused browsers available for NixOS are
based on Firefox. Chromium derivatives like Ungoogled Chromium and Brave do
exist in Nixpkgs, but are less recommended by privacy advocates.

### Choosing Secure Browsers and Search Engines

On a hardened Linux system, the browser is most often the weakest link exposed
to the internet, and so security, privacy, and anti-tracking features of
browsers are now as important, or even more important than platform-level
protections.

#### Tor Browser

Tor is a modified version of Firefox specifically designed for use with Tor.

Tor routes your internet traffic through a global volunteer-operated network,
masking your IP address and activities from local observers, ISPs, websites, and
surveillance systems. This helps you protect personal information and maintain
anonymity when browsing, communicating, or using online services.

- [Tor on NixOS](https://wiki.nixos.org/wiki/Tor)
  - [Tor Browser User Manual](https://tb-manual.torproject.org/)

  - [Tor staying-anonymous](https://support.torproject.org/faq/staying-anonymous/)

  - [How to Use Tor](https://ssd.eff.org/module/how-to-use-tor)

  - [Cool Graphic Showing Secure Connections with Tor](https://torproject.github.io/manual/secure-connections/)

#### Mullvad-Browser

Mullvad-Browser is free and open-source and was developed by the Tor Project in
collaboration with Mullvad VPN.(Another Firefox Derivative)

It is the Tor Browser without the Tor Network, allowing you to use the privacy
features Tor created along with a VPN if you so choose.

- [Mullvad-Browser](https://mullvad.net/en/browser), is in Nixpkgs as:
  `pkgs.mullvad-browser`

## LibreWolf

**LibreWolf** is an open-source fork of Firefox with a strong focus on privacy,
security, and user freedom. LibreWolf enables always HTTPS, includes
uBlockOrigin, and only includes privacy focused search engines by default such
as:

**SearXNG** an open-source, privacy-respecting metasearch engine that aggregates
results from various search services, such as Google, DuckDuckGo, etc without
tracking you or profiling your searches.

Example LibreWolf config implementing many of the STIG recommendations:

<details>
<summary> ✔️ Click to expand LibreWolf Example </summary>

```nix
# librewolf.nix
{pkgs, lib, config, ...}: let
  cfg = config.custom.librewolf;
in {
  options.custom.librewolf = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable the LibreWolf Module";
    };
  };

  config = lib.mkIf cfg.enable {
    programs.librewolf = {
      enable = true;
      policies = {
        # A bit annoying
        DontCheckDefaultBrowser = true;
        # Pocket is insecure according to DoD
        DisablePocket = true;
        # No imperative updates
        DisableAppUpdate = true;
      };
      settings = {
        # // SV-16925 - DTBF030
        "security.enable_tls" = true;
        # // SV-16925 - DTBF030
        "security.tls.version.min" = 2;
        # // SV-16925 - DTBF030
        "security.tls.version.max" = 4;

        # // SV-111841 - DTBF210
        "privacy.trackingprotection.fingerprinting.enabled" = true;

        # // V-252881 - Retaining Data Upon Shutdown
        "browser.sessionstore.privacy_level" = 0;

        # // SV-251573 - Customizing the New Tab Page
        "browser.newtabpage.activity-stream.enabled" = false;
        "browser.newtabpage.activity-stream.feeds.section.topstories" = false;
        "browser.newtabpage.activity-stream.showSponsored" = false;
        "browser.newtabpage.activity-stream.feeds.snippets" = false;

        # // V-251580 - Disabling Feedback Reporting
        "browser.chrome.toolbar_tips" = false;
        "browser.selfsupport.url" = "";
        "extensions.abuseReport.enabled" = false;
        "extensions.abuseReport.url" = "";

        # // V-251558 - Controlling Data Submission
        "datareporting.policy.dataSubmissionEnabled" = false;
        "datareporting.healthreport.uploadEnabled" = false;
        "datareporting.policy.firstRunURL" = "";
        "datareporting.policy.notifications.firstRunURL" = "";
        "datareporting.policy.requiredURL" = "";

        # // V-252909 - Disabling Firefox Studies
        "app.shield.optoutstudies.enabled" = false;
        "app.normandy.enabled" = false;
        "app.normandy.api_url" = "";

        # // V-252908 - Disabling Pocket
        "extensions.pocket.enabled" = false;

        # // V-251555 - Preventing Improper Script Execution
        "dom.disable_window_flip" = true;

        # // V-251554 - Restricting Window Movement and Resizing
        "dom.disable_window_move_resize" = true;

        # // V-251551 - Disabling Form Fill Assistance
        "browser.formfill.enable" = false;

        # // V-251550 - Blocking Unauthorized MIME Types
        "plugin.disable_full_page_plugin_for_types" = "application/pdf,application/fdf,application/xfdf,application/lso,application/lss,application/iqy,application/rqy,application/lsl,application/xlk,application/xls,application/xlt,application/pot,application/pps,application/ppt,application/dos,application/dot,application/wks,application/bat,application/ps,application/eps,application/wch,application/wcm,application/wb1,application/wb3,application/rtf,application/doc,application/mdb,application/mde,application/wbk,application/ad,application/adp";
      };
    };
    xdg.desktopEntries.librewolf = {
      name = "LibreWolf";
      exec = "${pkgs.librewolf}/bin/librewolf";
    };
    xdg.mimeApps = {
      enable = true;
      defaultApplications = {
        "text/html" = "librewolf.desktop";
        "x-scheme-handler/http" = "librewolf.desktop";
        "x-scheme-handler/https" = "librewolf.desktop";
        "x-scheme-handler/about" = "librewolf.desktop";
        "x-scheme-handler/unknown" = "librewolf.desktop";
      };
    };
  };
}
```

And enable it in your `home.nix` or equivalent with:

```nix
# home.nix
custom.librewolf.enable = true;
```

The `xdg` settings at the end make LibreWolf the defaults for what is listed.

Thanks to `JosefKatic` for putting the above STIG settings in NixOS format.

</details>

You can test your browser to see how well you are protected from tracking and
fingerprinting at [Cover Your Tracks](https://coveryourtracks.eff.org/).

<details>
<summary> ✔️ Click to Expand Script to wipe cache and generate new `machine-id` </summary>

- [man page machine-id(5)](https://www.man7.org/linux/man-pages/man5/machine-id.5.html)

- The following example is adapted from
  [Firejail All About Tor](https://firejail.wordpress.com/all-about-tor/)
  section, adapted for NixOS.

Save the following script as `cleanup.sh`, change `Your-User` to your username:

```bash
#!/bin/sh -e
USER="Your-User"
HOME_DIR="/home/$USER"
# clear user cache directly as root
sudo -u "$USER" rm -fr "$HOME_DIR/.cache"
# generate a new machine-id
rm -f /var/lib/machine-id
dbus-uuidgen > /var/lib/machine-id
cp /var/lib/machine-id /etc/machine-id
chmod 444 /etc/machine-id
exit 0
```

The `~/.cache` directory is where most programs store runtime information:
webpages you visited, torrent trackers you connected to, and deleted emails.
It's a good idea to remove them at shutdown. --Firejail all-about-tor

Check `/etc/machine-id` & `~/.cache` before running the script:

```bash
cat /etc/machine-id
# Output
0b46feb27a20469da0ee62baaeb51c5c
ls ~/.cache
```

```bash
chmod +x cleanup.sh
sudo ./cleanup.sh
```

Recheck your `machine-id` and `~/.cache` directories, you should have a newly
generated `machine-id` and minimal files in the `~/.cache` directory. The
Firejail example shows a systemd unit that runs the above script at every
shutdown but that may be overkill, I suggest running it occasionally to make it
harder for sites to link your `machine-id` to you.

</details>

Privacy protection doesn't need to be perfect to make a difference. The best
protection against tracking and fingerprinting available is to use Tor. For your
daily driver, tools like Ghostery, UblockOrigin, Privacy Badger, Disconnect, and
NoScript reduce fingerprinting significantly but not completely. Some privacy
guides will suggest not to use add-ons at all because they make your browser
unique, this isn't one of them. Just use Tor when it truly matters.

- [Surveillance Self-Defense How to: Use Tor](https://ssd.eff.org/module/how-to-use-tor)

There are more hardening parameters that can be set but this should be a good
starting point for a hardened version of LibreWolf. When testing with Cover your
tracks, customized LibreWolf tested as having stronger tracking protection than
default Mullvad-Browser and NoScript significantly cuts down the data available
for fingerprinting by disabling JavaScript.

- The [Garuda Privacy-Guide](https://wiki.garudalinux.org/en/privacy-guide) has
  good tips and recommendations for browser add-ons.

</details>

### Virtual Private Networks (VPNs)

A **VPN** (Virtual Private Network) encrypts your Internet connection and routes
your traffic through a VPN provider’s servers, masking your IP address from
local network observers, ISPs, and websites. Using a VPN can prevent your ISP or
local Wi-Fi owner from tracking what sites you visit (they only see a connection
to the VPN), and can help circumvent some regional restrictions or filtering.

However, VPNs simply shift your trust: Instead of your ISP seeing your activity,
your VPN provider can, so you must trust their privacy policies and
infrastructure. Quality and privacy protections vary widely from one VPN company
to another.

You can use a VPN with Tor, but it's not recommended unless you're an advanced
user who knows how to configure both in a way that doesn't compromise your
privacy.

**Popular VPNs on NixOS**

- [Mullvad VPN](https://wiki.nixos.org/wiki/Mullvad_VPN) Mullvad VPN uses
  WireGuard under the hood and only works if `systemd-resolvd` is enabled.

- [WireGuard VPN](https://wiki.nixos.org/wiki/WireGuard)

- [Tailscale](https://wiki.nixos.org/wiki/Tailscale)

- [OpenVPN](https://wiki.nixos.org/wiki/OpenVPN)

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
      # See https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v3/public-resolvers.md
      sources.public-resolvers = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/public-resolvers.md"
          "https://download.dnscrypt.info/resolvers-list/v3/public-resolvers.md"
        ];
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3";
        cache_file = "/var/lib/${StateDirectory}/public-resolvers.md";
      };

      # Use servers reachable over IPv6 -- Do not enable if you don't have IPv6 connectivity
      ipv6_servers = hasIPv6Internet;
      block_ipv6 = ! hasIPv6Internet;
      blocked_names.blocked_names_file = blocklist_txt;
      require_dnssec = true;
      # Logs can get large very quickly...
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

- Above, we have a local DNS proxy that encrypts and forwards queries.

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
**all DNS queries from the computer** go to dnscrypt-proxy.

`inputs.oisd` refers to the flake input oisd blocklist, it prevents your device
from connecting to unwanted or harmful domains.

`dnscrypt-proxy2` then encrypts and forwards our DNS requests to third-party
public DNSCrypt or DoH servers.

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

- [Cloudflare What is HTTPS](https://cloudflare.com/learning/ssl/what-is-https)

- [What is Fingerprinting](https://ssd.eff.org/module/what-fingerprinting), more
  than you realize is being tracked constantly.

- [Surveillance Self-Defence](https://ssd.eff.org/) has a lot of helpful info to
  protect your privacy.

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

- [STIG Firefox Hardening](https://simeononsecurity.com/guides/enhance-firefox-security-configuring-guide/)

- [Mozilla Firefox Security Technical Implementation Guide](https://stigviewer.com/stigs/mozilla_firefox)
  The STIG for Mozilla Firefox (Security Technical Implementation Guide) is a
  set of security configuration standards developed by the U.S. Department of
  Defense. They are created by the Defense Information Systems Agency (DISA) to
  secure and harden DoD information systems and software.
