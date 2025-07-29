# Hardening Networking

<details>
<summary> Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

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
