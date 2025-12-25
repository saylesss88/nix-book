---
title: Browsing Security
date: 2025-12-21
author: saylesss88
collection: "blog"
tags: ["hardening", "privacy", "browsers"]
draft: false
---

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

# Browser/Browsing Security: Defense in Depth

> "The major problem with current systems is their inability to provide
> effective isolation between various programs running on one machine. E.g. if
> the user's Web browser gets compromised (due to a bug exploited by a malicious
> web site), the OS is usually unable to protect other user's applications and
> data from also being compromised." --Qubes arch-spec

The web browser is the most complex, most exposed, and most vulnerable
application on a hardened Linux system. It is your primary interface with the
internet and, consequently, the primary vector for exploitation and tracking.

The **Three Pillars of Web Defense** To secure your browsing, you must balance
three often-conflicting goals:

1. **Security (Exploit Mitigation)**: Preventing malicious sites from escaping
   the browser sandbox to access your local files or execute code.

2. **Privacy (Tracking Protection)**: Preventing advertisers and sites from
   linking your current session to your real-world identity or browsing history.

3. **Anonymity (Identity Obfuscation)**: Making your traffic indistinguishable
   from thousands of other users to hide your physical location and legal
   identity.

## Methods of Protection

- **Browser hardening** focuses on reducing attack surface and blocking tracking
  by disabling or restricting features like JavaScript, cookies, telemetry, and
  third-party scripts.

- **Fingerprint protection**, on the other hand, aims to make your browser
  indistinguishable from others. Instead of just blocking data collection, it
  ensures that your browser’s configuration; screen size, fonts, user agent,
  etc. matches a large group of users, so you blend in.

- **Anonymity**: Maximizing anonymity often means restricting or masking
  features (setting a generic fingerprint, disabling browser APIs, blocking
  trackers) so the browser blends in with many others. This reduces uniqueness
  but can break website functionality, cause CAPTCHAs, and limit usability.

- **Browser compartmentalization** is a technique where different browsers are
  dedicated to distinct online activities to isolate cookies, trackers, and
  browsing data. For example, Mullvad Browser can be used solely for activities
  where fingerprinting resistance is critical, such as anonymous browsing or
  visiting privacy-sensitive sites. Meanwhile, a hardened LibreWolf or Firefox
  can be used for general browsing, email, or banking where you want solid
  security and feature flexibility but aren't as concerned about fingerprint
  uniqueness.

On a hardened Linux system, the browser is most often the weakest link exposed
to the internet, and so security, privacy, and anti-tracking features of
browsers are now as important, or even more important than platform-level
protections.

---

Browsers leak identity in two main ways: **network identifiers** (IP address,
DNS, TLS metadata) and **browser identifiers** (fingerprinting + tracking). This
chapter focuses on the browser side first, then covers when a VPN or Tor changes
the network side. Before tweaking anything, pick the browsing goal (anonymity vs
privacy vs convenience), because the “best” settings differ.

If the goal is blending in, prefer a browser that ships with a shared,
consistent fingerprint (Tor Browser / Mullvad Browser). If the goal is mainly
reducing cross-site tracking for normal browsing, a hardened Firefox/LibreWolf
profile with minimal extensions is usually easier to live with.

### Fingerprinting

Modern web APIs make rich, customized experiences possible, but they also reveal
enough low‑level details about your device and browser to build a unique
fingerprint. This fingerprint can be used for hidden, persistent tracking, even
when cookies are blocked.

Browser fingerprinting is a tracking technique, often done by third-party
companies that specialize in it. They provide code (usually JavaScript) that a
website owner can embed on their site. When you visit the site, the script runs
in the background, silently collecting data about your device and browser.

- [Entropy](<https://en.wikipedia.org/wiki/Entropy_(computing)>): in this
  context, is a measure of how much unique information a specific browser
  feature contributes to your fingerprint. It's often quantified in **bits of
  entropy**, where higher bits mean more uniqueness (i.e., easier to identify
  you).
  - A "bit" is a basic unit of information for computers. Entropy measuring
    sites results are measured in "bits of identifying information".

There are two main approaches to obfuscating your fingerprint:

- **Standardization**: Make browsers standardized and therefore have the same
  fingerprint to blend into a crowd. This is what Tor and Mullvad Browser do.
  Best for anonymity; increases the crowd you blend into, but may decrease
  usability (site breakage, CAPTCHAs); adversaries may still find subtle
  differences.

- **Randomization**: Randomize fingerprint metrics so it's not directly linkable
  to you. Brave has this feature, if you run coveryourtracks with Brave you will
  get a result of "your browser has a randomized fingerprint". This is good for
  privacy but may be detectable by advanced scripts.

### Fingerprint Testing

You can test your browser to see how well you are protected from tracking and
fingerprinting at [Cover Your Tracks](https://coveryourtracks.eff.org/).

Also check out, [Am I Unique](https://amiunique.org/fingerprint)

> ⚠️ WARNING: Don't put too much weight into the results as people often check
> their fingerprint, change one metric and check it again over and over skewing
> the results. It is helpful for knowing the fingerprint values that trackers
> track.

- [Browser Fingerprinting Tor Forum](https://forum.torproject.org/t/browser-fingerprinting/1228/25)

- [Madaidans Hot Take on Browser Tracking](https://madaidans-insecurities.github.io/browser-tracking.html)

---

### Browsers

I currently run NixOS in a VM with a secureblue host,
[Trivalent](https://github.com/secureblue/Trivalent) is my default browser, a
security-focused, Chromium-based browser.

In this section, the goal is to outline several browser options on NixOS and
show concrete configurations (with a focus on LibreWolf that can be adapted to
Firefox). Browsers are complicated and people have different security, privacy,
and usability needs, so this is not an endorsement of a single “best” choice.
Instead, the following subsections describe trade-offs and example setups so you
can decide which browser and configuration best match your own threat model and
workflow.

IMO there aren’t many Chromium-based browsers on NixOS that hit the sweet spot
of being both security-forward and privacy-respecting. Many of the “privacy
browser” options you’ll see on NixOS are Firefox forks; with the right tweaks
they can be made pretty private, but they generally don’t match Chromium-family
browsers on exploit mitigations and sandbox depth. ​

#### Brave

Brave is basically “Chromium, but with privacy features turned on by default,”
and the big win is that most of the protection comes from the browser itself
(Shields + anti-fingerprinting) instead of from a pile of extensions. Brave’s
fingerprinting story is also unusually practical: instead of trying to make
every Brave user look identical, it mixes API blocking with per-site/per-session
randomization (“farbling”) so the fingerprint is harder to reuse across
contexts.

On NixOS, IMO the best available Chromium-based browser is Brave. Brave strips
out Google's tracking code and includes a native add/tracker blocker (Brave
Shields). It also ships a best‑effort anti‑fingerprinting system that combines
(1) blocking/removing/modifying certain high-signal APIs and (2) “privacy
through randomization” (farbling), where it returns slightly altered values so
fingerprints don’t stay stable across sites/sessions. ​

**Things to Note about Brave**:

- Shields is a bundle of controls (trackers/ads, cookies, fingerprinting
  defenses, HTTPS upgrades, referrer/query stripping, storage cleanup), so
  turning Shields off for a broken site is a big privacy downgrade for that
  site—not just “adblock off.” ​

- Brave explicitly recommends validating fingerprinting defenses with realistic
  tests (new private window, restart browser, different profile, clear site
  storage) and expects the fingerprint to change across those boundaries. ​

- Extension bloat will break Brave’s privacy and security story: each extension
  increases attack surface and usually has broad privileges, while Brave’s core
  pitch is that you can get most of the privacy wins without third-party code in
  your browser process

**Drawbacks worth mentioning**:

- Brave is still Chromium/Blink, so using it doesn’t help with engine diversity;
  if Gecko dies, the web becomes even more “whatever Chromium implements,” which
  is a long-term ecosystem risk.

- Brave ships a lot (Rewards, Wallet, VPN upsells, Leo/AI features depending on
  build/channel), and every extra subsystem is more UI complexity and
  potentially more bugs/attack surface than some browsers.

- Brave’s model assumes the browser does most privacy work; piling on extensions
  increases privileged code, fingerprint uniqueness, and the chance of a
  malicious/compromised extension.

- Farbling and API defenses reduce stability/linkability, but sophisticated
  trackers can still correlate behavior, logins, IP ranges, and high-level
  patterns—so “I enabled anti-fingerprinting” shouldn’t be read as “I can’t be
  tracked.”

- Brave’s built-in ad system (Rewards) is opt-in, but Brave also promotes
  features like Rewards and can show sponsored content (e.g., sponsored new-tab
  images) unless you disable/hide it

#### Firefox

Firefox is kind of its own thing: it runs on Gecko, not on Chromium or WebKit,
so it’s one of the only mainstream browsers that isn’t just another Chrome fork.
That uniqueness matters if you care about engine diversity and not having the
entire web effectively dictated by a single vendor. It’s also very tweakable, so
if you’re willing to flip some prefs and add a couple of key extensions, you can
turn it into a solid privacy‑focused daily driver without giving up a
non‑Chromium stack.

Firefox will usually get security fixes sooner than any fork, and some forks lag
behind on patching, which can leave known vulnerabilities exploitable for
longer. If you use Firefox’s built‑in Enhanced Tracking Protection (ETP), Resist
Fingerprinting (RFP), and hardening templates like ghacks or Arkenfox together
with uBlock Origin configured for dynamic filtering, you can replicate what used
to require a pile of separate extensions.

#### Site Isolation & Firefox Links

Firefox does implement Site Isolation via
[Project Fission](https://wiki.mozilla.org/Project_Fission), but it’s newer and
historically less mature than Chromium’s site‑per‑process model, and it may not
be enabled everywhere by default. To check that it is active, go to
`about:config` and ensure both `fission.autostart` and `gfx.webrender.all` are
set to `true`.

With uBlock Origin you can disable JavaScript per‑site (similar to NoScript),
enable a bunch of high‑quality blocklists, and selectively relax rules when a
trusted site breaks. To turn on Enhanced Tracking Protection and fingerprinting
protections in the UI, go to `Settings -> Privacy & Security` ->
`Enhanced Tracking Protection -> Custom`; if that causes breakage on a
particular website, click the shield icon in the URL bar and disable protections
just for that site.

- [uBlock Wiki](https://github.com/gorhill/uBlock/wiki)

Once you select `Custom`, you'll see that among the options is to block
`Known fingerprinters` as well as `Suspected fingerprinters`. The "Known
Fingerprinters" protection works by blocking scripts listed in
[Disconnect's fingerprinting list](https://disconnect.me/trackerprotection#categories_of_trackers)
For most users they suggest using the above FPP to avoid breakage. To go further
and enable RFP, go to `about:config` and set `privacy.resistFingerprinting` to
`true`.

<details>
<summary> ✔️ Further Reading on Firefox Defenses </summary>

- [Mozilla Resist Fingerprinting](https://support.mozilla.org/en-US/kb/resist-fingerprinting)
  - To ensure Site Isolation is enabled, in `about:config`, set
    `fission.autostart`, and `gfx.webrender.all` prefs to `true`.(It's disabled
    by default on android).

- [Entropy](<https://en.wikipedia.org/wiki/Entropy_(computing)>): in this
  context, is a measure of how much unique information a specific browser
  feature contributes to your fingerprint. It's often quantified in **bits of
  entropy**, where higher bits mean more uniqueness (i.e., easier to identify
  you).
  - A "bit" is a basic unit of information for computers. Entropy measuring
    sites results are measured in "bits of identifying information".

- [Origin](https://developer.mozilla.org/en-US/docs/Glossary/Origin): Web
  content's _origin_ is defined by the _scheme_ (protocol), _hostname_ (domain),
  and port of the URL used to access it. Two objects have the same origin only
  when the scheme, hostname, and port all match.

- [Same-origin policy](https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy):
  is a critical security mechanism that restricts how a document or script
  loaded by one origin can interact with a resource from another origin. It
  helps isolate potentially malicious documents, reducing possible attack
  vectors.

- [Firefox Site-Isolation](https://blog.mozilla.org/security/2021/05/18/introducing-site-isolation-in-firefox/).
  Firefox does provide site-isolation as well.

- [Protection from side-channel attacks](https://www.mozilla.org/en-US/security/advisories/mfsa2018-01/)

- [MDN Insecure passwords](https://developer.mozilla.org/en-US/docs/Web/Security/Insecure_passwords)
  - [Risks of reused passwords](https://blog.mozilla.org/tanvi/2016/01/28/no-more-passwords-over-http-please/)

</details>

---

## LibreWolf

**LibreWolf** is an open-source fork of Firefox with a strong focus on privacy,
security, and user freedom. LibreWolf enables always HTTPS, includes
uBlockOrigin, and only includes privacy focused search engines by default.

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

Also, go to
[accounts.firefox](https://accounts.firefox.com/settings#data-collection) and
turn off "Allow Mozilla accounts to send technical and interaction data to
Mozilla". Also set 2-fa in
[Security Settings](https://accounts.firefox.com/settings#security)

I always set `Max Protection` for DNS over HTTPS and personally set a custom
resolver to `https://dns.quad9.net/dns-query`

- Mullvad is also a good option:
  [Mullvad no-logging-data-policy](https://mullvad.net/en/help/no-logging-data-policy)

<details>
<summary> ✔️ Alternative LibreWolf Configuration utilizing Arkenfox </summary>

```nix
{
  pkgs,
  lib,
  config,
  ...
}: let
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
        DontCheckDefaultBrowser = true;
        DisablePocket = true;
        DisableAppUpdate = true;
      };
      profiles.my-default = {
        isDefault = true;
        name = "Default Profile";
        extraConfig = ''
          ${builtins.readFile ./user.js}
          "general.autoScroll" = true;
          "sidebar.verticalTabs" = true;
        '';

        settings = {
        };
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

Download the
[Arkenfox user.js](https://github.com/arkenfox/user.js/blob/master/user.js) and
review it making sure that you agree with the settings. If you do, place it in
the same directory as your `librewolf.nix`.

Read the [Arkenfox Wiki](https://github.com/arkenfox/user.js/wiki)

The `user.js` is full of comments and information, read it and adjust it for
your needs. The following enables RFP fingerprint protection:

```js
***/ user.js ***/
user_pref("privacy.resistFingerprinting", true); // [FF41+]
user_pref("privacy.resistFingerprinting.pbmode", true); // [FF114+]
```

As you learn more, you can get more strict if you so choose.

Rebuild, launch LibreWolf, and check your `~/.librewolf/my-default/user.js`. It
should match the Arkenfox settings. Initially, only the `user.js` will be
listed, as you run LibreWolf other profile files and folders are created
dynamically.

In LibreWolf type `Ctrl + Shift + J` and look for any errors.

Type `about:config` into the address bar and search a few of the settings that
Arkenfox changes, do they match?

The `user.js` is read **in order**, if there are 2 of the same setting, the last
one will be applied. Adding overrides to the settings attribute above places the
changes at the **beginning** of the `user.js` which isn't what we want. Placing
them after the `${builtins.readFile ./user.js}` in `extraConfig` amends them to
the **end** of the `user.js` allowing us to override the defaults.

The process is the same with Firefox but since Arkenfox strongly recommends
Ublock Origin and it is built into LibreWolf it makes sense to use the browser
with the stronger defaults.

> ❗ NOTE: There is a home-manager module called `arkenfox-nixos` that is
> supposed to make updates easier but IMO the documentation leaves you guessing
> how to use it. As updates come in to Firefox/LibreWolf some of the settings
> become unnecessary so it's important to keep an eye on both Firefox and
> Arkenfox updates. Which both have RSS feeds that will alert you upon changes.

I personally use [Feeder](https://feeder.co/) as my open-source RSS feed reader,
available in most app stores including F-Droid. It is listed on
[PrivacyTools](https://www.privacytools.io/privacy-rss-feed-readers).

- [Arkenfox Recent Commits RSS feed](https://github.com/arkenfox/user.js/commits/master.atom)

- [Arkenfox Release Notes RSS](https://github.com/arkenfox/user.js/releases.atom)

- [Firefox Nightly release notes](https://www.mozilla.org/en-US/firefox/nightly/notes/feed/)

</details>

#### Search Defaults

**Startpage**: Advertised as the world's most private search engine. "Startpage
delivers Google search results via their proprietary personal data protection
technology."

- [Startpage](https://www.startpage.com/)

- To add Startpage as a search engine, add
  `https://www.startpage.com/sp/search?query=%s`.

**SearXNG** an open-source, privacy-respecting metasearch engine that aggregates
results from various search services, such as Google, DuckDuckGo, etc. without
tracking you or profiling your searches. You can add SearXNG to firefox by going
to `about:preferences#search` and at the bottom click `Add`, URL will be
`https://searx.be/search?q=%s`.

> ❗️ NOTE: SearXNGs google results are not working as of 11-17-25 and haven't
> for a while now leading to bad results being returned for most instances. It's
> my understanding this is because Google is actively blocking automated
> requests from SearXNG. Devs sometimes publish patches or workarounds, but
> these are quickly blocked when Google changes their back-end.

> ❗️ NOTE: The above searx is the default and doesn't give many relevant
> results. To get relevant results find a
> [public instance](https://searx.space/) with a good rating from your area and
> add the `search?q=%s` to the end of it. For example, I'm using
> `https://priv.au/search?q=%s`.

Searx is a bit different, you can choose which search engine you want for your
current search with `!ddg search term` to use duckduckgo for example.

</details>

#### Tor Browser

> ❗ NOTE: Tor is **not** the most secure browser, anonymity and security can
> often be at odds with each other. Having the exact same browser as many other
> people isn't the best security practice, but it is great for anonymity. Tor is
> also based on Firefox Esr, which only receives patches for vulnerabilities
> considered Critical or High which can be taken advantage of.

Tor is a modified version of Firefox specifically designed for use with Tor.

Tor routes your internet traffic through a global volunteer-operated network,
masking your IP address and activities from local observers, ISPs, websites, and
surveillance systems. This helps you protect personal information and maintain
anonymity when browsing, communicating, or using online services.

Adding browser plugins to Tor can de-anonymize you, don't do it. Tor is already
built with the necessary plugins and privacy protecting rules, so adding more is
unnecessary and actually dangerous for your anonymity.

A Tor exit node can easily see your traffic, and if you're not using HTTPS then
it may be able to modify that traffic. Only use HTTPS when browsing the clear
net with Tor, this doesn't apply to onion services (`.onion`) as the traffic
stays inside the Tor network all the way to the destination.

You can visit both the clear web and `.onion` sites on Tor. Whenever possible
you should utilize Onion Services (`.onion` addresses) so communications and web
browsing stay within the Tor network. `.onion` URLS form a tunnel that is
end-to-end encrypted using a random rendezvous point and incorporating
[perfect forward secrecy (PFS)](https://en.wikipedia.org/wiki/Forward_secrecy).

Bridges are only necessary in countries that don't allow people to use Tor.
Using Bridges when they aren't needed takes resources away from people in
oppressive regimes that need, only use them if necessary. Read the guides, and
use Tails OS, or Whonix when it really matters.

- [Whonix KVM on NixOS](https://saylesss88.github.io/nix/whonix_kvm.html)

You will see a lot of conflicting information about using Tor with a VPN. If you
are in an area that blocks access to Tor or it is dangerous to use Tor, by all
means use a trusted VPN.

### TorPlusVPN

- [Tor Project Wiki TorPlusVPN](https://gitlab.torproject.org/legacy/trac/-/wikis/doc/TorPlusVPN)

- [Safely Connecting to Tor](https://www.privacyguides.org/en/advanced/tor-overview/#safely-connecting-to-tor)

**Learn about Tor**

I recommend starting with
[Privacy Guides In Praise of Tor](https://www.privacyguides.org/articles/2025/04/30/in-praise-of-tor/#onion-sites-you-can-visit-using-the-tor-browser)
and then reading their
[Tor Overview](https://www.privacyguides.org/en/advanced/tor-overview/) they
have been the most informative resources I've come across yet.

The Electronic Frontier Foundation sponsors and helps fund Tor and so does the
United States Government.

If you are fortunate to live outside of oppressive regimes with extreme
censorship, using Tor for every day, mundane activities is likely safe and won’t
put you on any harmful “list.” Even if it did, you'd be in good company—these
lists mostly contain great people working tirelessly to defend human rights and
online privacy worldwide.

By using Tor regularly for ordinary browsing, you help strengthen the network,
making it more robust and anonymous for everyone. This collective support makes
staying private easier for activists, journalists, and anyone facing online
surveillance or censorship. The writer of the PrivacyGuides article mentions
using Tor when he needs to access Google Maps to protect his privacy

So, consider embracing Tor not only for sensitive browsing but also for daily
routine tasks. Every user adds valuable noise to the network, helping protect
privacy and freedom for all.

**Tor is at risk, and needs our help**. Despite its strength and history, Tor
isn't safe from the same attacks oppressive regimes and misinformed legislators
direct at encryption and many other privacy-enhancing
technologies.--[How to Support Tor](https://www.privacyguides.org/articles/2025/04/30/in-praise-of-tor/#how-to-support-tor)

- [Tor on NixOS](https://wiki.nixos.org/wiki/Tor)
  - [Tor Browser User Manual](https://tb-manual.torproject.org/)

  - [Tor staying-anonymous](https://support.torproject.org/faq/staying-anonymous/)

  - [How to Use Tor](https://ssd.eff.org/module/how-to-use-tor)

  - [Cool Graphic Showing Secure Connections with Tor](https://torproject.github.io/manual/secure-connections/)

---

#### Mullvad-Browser

Rather than try to tweak a browser into fingerprinting submission, I recommend
using either Tor or Mullvad-Browser when fingerprintability is the highest
issue. Both Tor and Mullvad-Browser were designed specifically for this purpose
and you likely won't get as much out of tweaking another browser.

Mullvad-Browser is free and open-source and was developed by the Tor Project in
collaboration with Mullvad VPN.(Another Firefox Derivative). It is also the top
recommended browser from PrivacyGuides.

It is the Tor Browser without the Tor Network, allowing you to use the privacy
features Tor created along with a VPN if you so choose.

- [Mullvad-Browser](https://mullvad.net/en/browser), is in Nixpkgs as:
  `pkgs.mullvad-browser`

---

## Making Your Browser Amnesic, the Nix Way

**Problem**: Browsers leak data via `.cache` and `.config`.

**Pro Tip**: Even if you use a persistent home directory, you should mount your
`~/.cache` folder to `tmpfs`.

- **Performance**: Browsers perform thousands of small read/writes to the cache.
  In-memory storage is significantly faster.

- **Disk Health**: It prevents "SSD wear" from constant caching of temporary web
  assets.

- **Forensic Hygiene**: It ensures that volatile "junk" like images, scripts,
  and stylesheets never touches your physical platter.

> **Note**: This will **not** wipe your browser session (tabs, cookies,
> history), as those are stored in `~/.config`. If you want a truly "amnesic"
> browsing session that wipes everything on reboot, you must also mount your
> browser's profile directory (e.g., `~/.mozilla` or `~/.config/BraveSoftware`)
> to `tmpfs`.

```nix
fileSystems."/home/youruser/.cache" = {
  device = "none";
  fsType = "tmpfs";
  options = [ "size=4G" "mode=777" ];
};
```

Ensure it was applied with:

```bash
findmnt /home/youruser/.cache
```

You can apply the same `tmpfs` logic to your config folder. **Warning**: This
will wipe your settings, extensions, and history every reboot.

Manual `tmpfs` mounts in `configuration.nix` are powerful because they happen at
the system level, but they require explicit ownership (`uid`/`gid`) to work with
user-level applications.

```nix
fileSystems."/home/youruser/.config/BraveSoftware" = {
  device = "none";
  fsType = "tmpfs";
  options = [
  "size=4G"
  "mode=700" # Use 700 for privacy
  "noatime"
  # Replace `1000` with the output of `id -u`
  "uid=1000"
  # Replace `100` with output of `id -g`
  "gid=100"
  ];
};
```

**How to find your UID/GID**

```bash
id -u && id -g
```

Replace the above `uid=`, and `gid=` values with the output of the above
command.

---

<details>
<summary> ✔️ Example Script to wipe cache and generate new `machine-id` </summary>

If you followed the above "Nix way" of Amnesic cache, the following script is
unnecessary, I'm leaving it here for now for those that are interested in
changing their machine-id imperatively.

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
protection against tracking and fingerprinting available is to use Tor. Many
add-ons are redundant, do some research and avoid using an add-on for something
that can be accomplished with built-in settings.

- [Surveillance Self-Defense How to: Use Tor](https://ssd.eff.org/module/how-to-use-tor)

There are more hardening parameters that can be set but this should be a good
starting point for a hardened version of LibreWolf. When testing with Cover your
tracks, customized LibreWolf tested as having stronger tracking protection than
default Mullvad-Browser and NoScript significantly cuts down the data available
for fingerprinting by disabling JavaScript.

- The [Garuda Privacy-Guide](https://wiki.garudalinux.org/en/privacy-guide) has
  good tips and recommendations for browser add-ons.

---

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

I see over and over again that Mullvad VPN is the best, I am in no way
affiliated with them this is just what I hear. They allow you to pay with cash
completely anonymously and keep very minimal metadata. Metadata is a big deal,
the US gov has admitted to killing people based solely on their metadata.

Your ISP almost certainly does sketchy stuff with your data, personally I would
rather trust a company like Mullvad whose whole reputation is based on their
trustworthiness, transparency, and data protection.

You can use a VPN with Tor, but it's not recommended by the Tor Project unless
you're an advanced user who knows how to configure both in a way that doesn't
compromise your privacy.

**Popular VPNs on NixOS**

- [Mullvad VPN](https://wiki.nixos.org/wiki/Mullvad_VPN) Mullvad VPN uses
  WireGuard under the hood and only works if `systemd-resolvd` is enabled.

- [WireGuard VPN](https://wiki.nixos.org/wiki/WireGuard), WireGuard is a
  protocol, but also a VPN provider on NixOS.

- [Tailscale](https://wiki.nixos.org/wiki/Tailscale)

- [OpenVPN](https://wiki.nixos.org/wiki/OpenVPN), OpenVPN is both a protocol and
  full-featured VPN provider on NixOS.
