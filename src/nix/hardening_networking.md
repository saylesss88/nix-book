# Hardening Networking

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

> ⚠️ I am not a security expert, but I have carefully researched and tested the
> configurations in this chapter personally.Not every setting or method will be
> appropriate or necessary for all setups; some may cause compatibility or
> connectivity issues depending on your environment and needs.
>
> Always carefully research any changes before applying them, especially when
> they affect critical components like DNS or firewalls. Network hardening can
> occasionally disrupt legitimate services, degrade performance, or block access
> in unexpected ways.
>
> Test adjustments in a controlled environment first, understand the tradeoffs
> involved, and only use what fits your threat model and workflow. Security is
> complex, and it’s important to tailor your approach to your unique
> requirements. Take what’s useful, adapt as needed, and don’t hesitate to seek
> expert guidance for advanced scenarios.

## Simple Tips

A few simple things you can do to help protect your network:

- HTTPS encrypts the data between a web browser and a website. This is
  especially important for services that need logging into such as a bank
  account, email, etc. Most browsers either have a setting or an extension to
  enforce HTTPS everywhere, use it.

- [Discourse Declare Firefox extensions and settings](https://discourse.nixos.org/t/declare-firefox-extensions-and-settings/36265)

- [Cloudflare What is HTTPS](https://cloudflare.com/learning/ssl/what-is-https)

- Check sketchy urls first with
  [VirusTotal](https://www.virustotal.com/gui/home/url) where you can plug the
  URL into a scanner to ensure it's safe.

- [Surveillance Self-Defence](https://ssd.eff.org/) has a lot of helpful info to
  protect your privacy.

- [What is Fingerprinting](https://ssd.eff.org/module/what-fingerprinting),more
  than you realize is being tracked constantly.

- Take potentially dangerous PDFs, office documents, or images and convert them
  to a safe PDF with [dangerzone](https://github.com/freedomofpress/dangerzone)
  Be especially careful with torrents.

- Use a metadata-cleaner before you publish or share images, audio, video, etc.
  Nixpkgs has `pkgs.metadata-cleaner` and `pkgs.mat2`

- This may be unrelated but don't scan random QR codes either, you can download
  a QR scan checker to ensure the code isn't malicious. There are different apps
  for this on Android and IOS.

- Don't use your browsers "remember my password" or "Autofill" functions,
  disable and delete the history. Use a password manager instead. (security not
  usability)

- If you need to enter your credentials for something, don't click use Google or
  FaceBook to create the account. Doing so opens up all of those services if one
  of them gets compromised. Take the extra time to create an account with it's
  own unique password and user.

- Delete cookies and site data when the browser is closed. (security not
  usability). I believe this is the default on LibreWolf, I recently started
  using this and it's not too bad with bitwarden.

- Use passkeys to store complex passwords, use your password manager to generate
  unique and complex passphrases/passwords for you and save them right away.
  Make sure to follow the next suggestion for your password manager.

- Use 2 factor authentication everywhere possible.

I never liked the argument, "I'm not doing anything illegal so I don't care if
they spy on me and make a profit off of my data". Regardless of what you do
online, I think it's your business and yours alone. Why make it easy on them if
you can limit your exposure?

### Choosing a secure Browser

On a hardened Linux system, the browser is most often the weakest link exposed
to the internet, and so security, privacy, and anti-tracking features of
browsers are now as important, or even more important than platform-level
protections.

#### Tor Browser

Tor routes your internet traffic through a global volunteer-operated network,
masking your IP address and activities from local observers, ISPs, websites, and
surveillance systems. This helps you protect personal information and maintain
anonymity when browsing, communicating, or using online services.

- [Tor on NixOS](https://wiki.nixos.org/wiki/Tor)
  - [Tor Browser User Manual](https://tb-manual.torproject.org/)

  - [Tor staying-anonymous](https://support.torproject.org/faq/staying-anonymous/)

  - [How to Use Tor](https://ssd.eff.org/module/how-to-use-tor)

  - [Cool Graphic Showing Secure Connections with Tor](https://torproject.github.io/manual/secure-connections/)

#### SearXNG

SearXNG is a privacy-respecting metasearch engine that aggregates results from
various search services, such as Google, DuckDuckGo, etc without tracking you or
profiling your searches.

- [SearXNG on NixOS](https://wiki.nixos.org/wiki/SearXNG)
  - [Welcome to SearXNG](https://docs.searxng.org/)

#### Hardening Other Browsers

If the above browsers aren't for you, you can harden the browser you're using
now. For example, Firefox has a lot of documentation on hardening:

- [Firefox Hardening Guide](https://brainfucksec.github.io/firefox-hardening-guide)

- [STIG Firefox Hardening](https://simeononsecurity.com/guides/enhance-firefox-security-configuring-guide/)

- [Mozilla Firefox Security Technical Implementation Guide](https://stigviewer.com/stigs/mozilla_firefox)
  The STIG for Mozilla Firefox (Security Technical Implementation Guide) is a
  set of security configuration standards developed by the U.S. Department of
  Defense. They are created by the Defense Information Systems Agency (DISA) to
  secure and harden DoD information systems and software.

**LibreWolf** is an open-source fork of Firefox with a strong focus on privacy,
security, and user freedom. LibreWolf enables always HTTPS, includes
uBlockOrigin, and only includes privacy focused search engines by default among
other strong defaults. Also, since LibreWolf is a fork of Firefox most of the
other hardening options work for it as well.

- [NixOS Wiki LibreWolf](https://wiki.nixos.org/wiki/Librewolf), the options in
  the wiki make it less secure and aren't recommended settings to use. They
  explicitly disable several of LibreWolf's default privacy-enhancing features,
  such as fingerprinting resistance and clearing session data on shutdown.

- [LibreWolf Features](https://librewolf.net/docs/features/) You still need to
  enable DNS over HTTPS through privacy settings.

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

Thanks to `JosefKatic` for putting the above settings in NixOS format.

There are more hardening parameters that can be set but this should be a good
starting point for a hardened version of LibreWolf.

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

`inputs.oisd` refers to the flake input oisd blocklist, it prevents your device
from connecting to unwanted or harmful domains.

- [oisd.nl](https://oisd.nl/) the oisd website

`dnscrypt-proxy2` filters ads/trackers (using oisd), enforces DNSSEC, and uses
encrypted transports (DNS-over-HTTPS/DoH, DNSCrypt, optionally
DNS-over-TLS/DoT).

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

Review listening ports: After each rebuild, use `ss -tlpn`, `nmap` or `netstat`
to see which services are accepting connections. Close or firewall anything
unnecessary.

## OpenSnitch

- [NixOS Wiki OpenSnitch](https://wiki.nixos.org/wiki/OpenSnitch)

[Opensnitch](https://github.com/evilsocket/opensnitch) is an open-source
application firewall that focuses on monitoring and controlling outgoing network
connections on a per-application basis.

This can be used to block apps from accessing the internet that shouldn't need
to (i.e., block telemetry and more). Opensnitch will report that the app has
attempted to make an outbound internet connection and block it or allow it based
on the rules you set.
