---
title: Hardening NixOS
date: 2026-01-13
author: saylesss88
collection: "blog"
tags: ["security", "privacy", "hardening"]
draft: false
---

# Hardening NixOS

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

![guy fawks hacker](../images/guy_fawks.png)

Securing your NixOS system begins with a philosophy of minimalism, explicit
configuration, and proactive control. As desktop Linux attracts more novice
users, it has become an increasingly valuable target for attackers. This makes
it crucial to adopt security best practices early to protect your desktop from
common attack vectors and to avoid configuration mistakes that could expose
vulnerabilities.

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
> either, always do your own research, things change frequently. Madaidan is
> also a contributor to both
> [Kicksecure](https://www.kicksecure.com/wiki/Contributors) and
> [Whonix](https://www.whonix.org/wiki/Contributors).

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
security and sandboxing if configured correctly. See
[Running NixOS in a VM](https://saylesss88.github.io/nix/kvm.html) for more
details on running NixOS in a Secureblue VM for additional security.

It's crucial to **document every change** you make. By creating smaller,
feature-complete commits, each with a descriptive message, you're building a
clear history. This approach makes it far simpler to revert a breaking change
and quickly identify what went wrong. Over time, this discipline allows you to
create security-focused checklists and ensure all angles are covered, building a
more robust and secure system.

Don't rely on single solutions or products, develop processes and defense in
depth. Think ahead and fail securely so that a single failure doesn't mean total
insecurity.

Attackers often monitor the latest Linux CVEs (Common Vulnerabilities and
Exposures) and check if and when specific distributions like NixOS have
implemented fixes. The unstable branch will receive the security patches and
fixes faster than stable which is another thing to keep in mind.

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

For example, to disable Xwayland for sway on home-manager you would add:

```nix
wayland.windowManager.sway = {
  enable = true;
  extraConfig = ''
    xwayland disable
  '';
}
```

- You may get an error saying you're only able to disable xwayland at boot,
  restart your system and you'll be all set.

You can explicitly disable `xdg-desktop-portal-wlr` with systemd in your
`configuration.nix` like this:

```nix
# configuration.nix
systemd.user.services."xdg-desktop-portal-wlr" = {
  enable = false;  # Masks/stops the wlr service
};
xdg.portal.wlr.enable = false;
```

## Common Attack Vectors for Linux

<details>
<summary> ✔️ Common Attack Vectors in Linux </summary>

**Privilege escalation**: The unauthorized act of gaining elevated permissions
rather than legitimate, controlled privilege use. It's a very common tactic that
threat actors use to take over a system, steal data, delete files, and more.

**Processes to protect against Privilege escalation**

- Adopt the principle of least privilege, only giving users the permissions that
  they require to perform their duties.

- Harden your system: Minimize the attack surface, use strong passwords, and
  follow best practices.

- Monitor relevant sources such as the
  [NIST National Vulnerability Database](https://www.strongdm.com/nist-compliance),
  [NixOS Security Advisories](https://github.com/NixOS/nix/security/advisories),
  and
  [NixOS Discourse Security](https://discourse.nixos.org/c/announcements/security/56)
  So you'll know the latest CVEs and vulnerabilities in Linux and NixOS.

- While not made for NixOS the
  [linPEAS Privilege Escalation Awesome Script](https://github.com/peass-ng/PEASS-ng/tree/master/linPEAS)
  gives you some useful info such as active capabilities and potential risks.

- Remove unnecessary SUID binaries to reduce the attack surface.

---

**Use after Free/Double free**:

**Use-After-Free (UAF)** is a type of software vulnerability that occurs in
memory unsafe languages (C C++) when a program continues to use a memory
location after it has been freed or deallocated.

**Double free**: is a flaw where a program frees the same memory block twice
using `free()` or `delete`, leading to undefined behavior and potential
exploitation.

Mitigation techniques include the use of hardened allocators such as
`hardened_malloc`, which improve memory management to detect and prevent UAF and
double-free bugs. Recent versions of `glibc` also incorporate built-in checks to
catch double frees.

---

**Unauthorized Access**:

Unauthorized access is the entry or use of your system, networks, or data by
individuals without permission. It's a common way for adversaries to exfiltrate
data, execute malicious code, and cause damage.

**Protections against Unauthorized Access**

- Strong Passwords, MFA, and robust Secrets management. In 2025, 22% of breaches
  involved stolen credentials overall; in basic web app attacks, 88% used stolen
  credentials.
  --[StrongDM data-breach-statistics](https://www.strongdm.com/blog/data-breach-statistics)

- Close unused ports with a Firewall

- Encrypt data in transit and at rest

- Watch your Logs, and deploy intrusion detection systems such as AIDE.

- [SQL Injection CWE](https://cwe.mitre.org/data/definitions/89.html), SQL
  injection is the most common critical web application vulnerability.

- [Cross Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/)

---

**Misconfiguration**

- With many new users trying NixOS, misconfiguration is common and an easy way
  for an attacker to gain control over your system.

- It is recommended to start slowly and try to ensure that you understand your
  configuration. Avoid copy-pasting config files that you don't understand yet.

---

**Zero Day Exploits**:

The term "Zero-Day" refers to a security vulnerability or flaw that is unknown
to the software developers or security teams, meaning they have had zero days to
create a patch or fix for it. This term is often associated with concepts such
as Vulnerabilities, Exploits, and Threats, and it’s important to distinguish
among them:

- A **Zero-Day Vulnerability** is a previously undiscovered security weakness or
  flaw in software that malicious actors can exploit.

- A **Zero-Day Exploit** describes the specific method or technique attackers
  use to take advantage of that vulnerability to compromise a system.

- A **Zero-Day Attack** happens when malicious actors launch an attack using a
  zero-day exploit before the software vendor has had a chance to patch or fix
  the vulnerability.

- [Project Zero's 0day spreadsheet](https://docs.google.com/spreadsheets/d/1lkNJ0uQwbeC1ZTRrxdtuPLCIl7mlUreoKfSIgajnSyY/view?gid=0#gid=0).
  You'll see that a majority of zero-days are Memory Corruption bugs.

- [Zero-Day tracking project](https://www.zero-day.cz/database/)

- [ Trend Micro's zero day inituative](https://www.zerodayinitiative.com/advisories/published/)

</details>

---

## Minimal Installation with LUKS

Begin with NixOS’s minimal installation image. This gives you a base system with
only essential tools and no extras that could introduce vulnerabilities.

NixOS's declarative model makes auditing the installed packages and services
easy, do so regularly.

---

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

---

## Guided Encrypted BTRFS Subvol install using disko

Use LUKS encryption to protect your data at rest, the following guide is a
minimal disko encrypted installation:
[Encrypted Install](https://saylesss88.github.io/installation/enc/enc_install.html)

---

## Installing Software

> The 2025 Edgescan study examined full-stack applications and found that
> one-third contained critical or severe vulnerabilities, putting them at risk.
> Over 45% of large enterprises leave unresolved vulnerabilities for more than a
> year. This shows the necessity of containing your apps in sandboxes when
> possible.
> --[edgescan Vulnerability Report](https://www.edgescan.com/stats-report/)

> ⚠️ For system security it is strongly advised to not install
> [proprietary](https://en.wikipedia.org/wiki/Proprietary_software),
> [non-freedom](https://www.gnu.org/proprietary/proprietary.html) software.
> Instead, use of
> [Free Software](https://www.fsf.org/about/what-is-free-software) is
> [recommended](https://www.gnu.org/philosophy/shouldbefree.html) --Kicksecure

- [Proprietary Software is Often Malware](https://www.gnu.org/proprietary/proprietary.html)
  NOTE: While I respect the importance of software freedom, I choose to focus on
  practical, technical solutions rather than engage with the ideological tone
  often present in related advocacy.
  - [User Freedom Threats](https://www.kicksecure.com/wiki/Miscellaneous_Threats_to_User_Freedom)

  - [Proprietary Back Doors](https://www.gnu.org/proprietary/proprietary-back-doors.html)

  - [EFF Back Doors](https://www.eff.org/deeplinks/2015/02/who-really-owns-your-drones)

```nix
# configuration.nix
nixpkgs.config.allowUnfree = false;
```

To explicitly disable it for flakes:

```nix
# ...snip...
pkgs = import nixpkgs {
  system = "x86_64-linux";
  config = {
    allowUnfree = false;
  };
};
# ...snip...
```

Most users don't fully understand that running any software without sandboxing
gives it unrestricted access to their user data and system resources. There is a
widespread lack of awareness that Linux apps generally run with the full
permissions of the user. It's easy to overlook the fact that "trusted source"
doesn't mean "safe to run uncontained". --summarized from kicksecure docs

**Pre-Install Recommendations**

When installing software, first check
[search.nixos](https://search.nixos.org/packages), and follow the `Homepage`
link to ensure that said package is maintained.

For example, when I search for `doas`, and go to the
[Homepage](https://github.com/Duncaen/OpenDoas) link, I can see that the most
recent commit was made 3 years ago. For certain software this might not be an
issue but `doas` isn't one of them.

Looking at the `sudo-rs`
[Homepage](https://github.com/trifectatechfoundation/sudo-rs) I can see that it
was updated yesterday (11-19-25) and might be a better alternative. It's
maintained and written in a memory safe language.

For critical apps like `sudo`, you should also check for vulnerabilities in said
software. If you did so for `sudo-rs`, you'd see
[CVE-2025-64170](https://nvd.nist.gov/vuln/detail/CVE-2025-64170) and see that
it's been patched. You can then look at the
[sudo-rs package.nix](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/su/sudo-rs/package.nix)
to ensure that the versions match. (As of 11-20-25 they match).

---

**nixpkgs-unstable Security Overview**

- `nixpkgs-unstable` tracks the master branch of the Nixpkgs repo and is
  constantly updated.

- This branch gets security updates faster, patching vulnerabilities faster.

- Since it's a rolling-release, packages are less thoroughly tested. This
  increases the risk of new, undiscovered bugs or regressions. Some of which
  could have security implications.

- The packages are generally the most recent upstream versions, which is
  important for security-sensitive software like browsers and kernels, as old
  versions may have publicly known, unpatched vulnerabilities.

- As the name states, `nixpkgs-unstable` is less stable and an update is more
  likely to cause your system to fail to build due to breaking changes in Nix
  expressions.

- I personally use unstable for everything, but I don't mind having to fix
  issues that arise.

---

**Stable (e.g., `nixos-24.05`) Security Overview**

Stable Nixpkgs channels correspond to point release (e.g., released every 6
months) and are supported for a limited period (typically one month past the
next release).

- Stable channels generally only receive conservative bug and security fixes.
  Major version bumps for features are typically avoided to maintain "stability
  against deliberate changes", which means you won't get the latest upstream
  features or general bug fixes.

- While critical security updates are backported quickly, updates for less
  critical packages may be slower or not happen at all if they require a
  significant refactoring or version bump.

- Stable channels are generally more stable, meaning updates are less likely to
  introduce breaking changes to your configuration or system environment.

- Many packages will be older versions. If a critical security vulnerability
  requires a major upstream version update (which is often avoided in a stable
  channel), the maintainers must backport the patch, a process which can
  introduce its own set of risks and delays.

---

**What should you use?**

The primary security trade-off is between **patching speed for known
vulnerabilities** and **stability/exposure to new bugs**:

- Choose `unstable` if you prioritize getting the latest security fixes
  (especially for end-user apps like browsers) as soon as they are available
  upstream, accepting a higher risk of non-security-related system breakage or
  new, undiscovered bugs.

- Choose `stable` if you prioritize system predictability and stability, relying
  on dedicated backports for critical vulnerabilities, while accepting that
  non-critical security and bug fixes will be delayed or absent until the next
  major release.

A common hybrid approach is to use the `stable` channel as the base for the OS
and selectively pin specific packages from `unstable` to ensure they receive
rapid security updates.

With flakes it's easy to add both `stable` and `unstable` as flake inputs and
access each with some simple logic.

<details>
<summary> ✔️ Click to Expand Flake example using both stable & unstable </summary>

```nix
{
  description = "NixOS configuration with two or more channels";

 inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, nixpkgs-unstable, ... }:
    {
      nixosConfigurations."your-host" = nixpkgs.lib.nixosSystem {
        modules = [
          {
            nixpkgs.overlays = [
              (final: prev: {
                unstable = nixpkgs-unstable.legacyPackages.${prev.system};
                # use this variant if unfree packages are needed:
                # unstable = import nixpkgs-unstable {
                #   inherit prev;
                #   system = prev.system;
                #   config.allowUnfree = true;
                # };
              })
            ];
          }
          ./configuration.nix
        ];
      };
    };
}
```

- This is also how you enable unfree packages for flakes rather than in your
  `configuration.nix`.

Now you can specify which packages are to be installed with which channel like
so:

```nix
# configuration.nix
{ pkgs, ... }:
{
  environment.systemPackages = [
    pkgs.firefox
    pkgs.unstable.helix
  ];
  # ...
}
```

</details>

---

## Users and SUID Binaries

**Replacing sudo with run0**

> NOTE: The point here is to avoid using the setuid binary (`sudo`), `run0` is a
> wrapper over `systemd-run` which speaks over Inter-process Communication
> Mechanisms (IPC) to PID1 which is considered safer than running a setuid
> binary. We separate our daily user from administration tasks and authenticate
> through our admin account. This reduces the attack surface by removing sudo as
> well as reduces the risk of local privilege escalation.

- **IPC** is the mechanism that allows processes to communicate. There are two
  methods of IPC, shared memory and message passing. An OS can implement both.

- **PID 1** is the first userspace process the kernel starts (the init system),
  which becomes the ancestor and reaper of all other processes; because it runs
  as root, is always present, and controls the system lifecycle, any bugs or
  design issues in PID 1 have outsized security impact and can translate into
  system-wide compromise or denial of service.

<details>
<summary> Click to Expand SUID and run0 resources </summary>

- [run0 explained by Lennart](https://mastodon.social/@pid_eins/112353324518585654)

- [setuid Wikipedia](https://en.wikipedia.org/wiki/Setuid)

- Using `run0` removes of these classes of
  [attacks](https://ruderich.org/simon/notes/su-sudo-from-root-tty-hijacking)

- The following lists some of the downsides
  [kicksecure vs secureblue](https://www.kicksecure.com/wiki/Dev/secureblue)

</details>

`run0` is not a SUID, it asks the service manager to invoke a command or shell
under the target user's UID. The target command is invoked in an isolated exec
context, freshly forked off PID1 without inheriting any context from the client.

The core danger of **setuid** (Set User ID) lies in its ability to allow a
low-privilege user to execute a program with the **permissions of the file's
owner**, which is most often the powerful **root user**.

### 💥 The Danger of setuid

For granting limited, controlled privilege escalation to apps, the primary
choices are broadly between traditional **setuid/setgid permissions** and more
modern **Linux capabilities**. [Jump to Capabilities](#capabilities)

- [Understanding setuid/setgid](https://www.cbtnuggets.com/blog/technology/system-admin/linux-file-permissions-understanding-setuid-setgid-and-the-sticky-bit)

Use the following command to find all SUID binaries:

```bash
sudo find / -perm -4000 -type f -ls 2>/dev/null
```

The `setuid` permission is dangerous because it creates a privilege escalation
pathway that can be exploited for malicious purposes.

- Temporary Root Access: When a file has the setuid bit set and is owned by
  `root`, any user who executes that program instantly and temporarily gains the
  full power of the root user while the program runs.

- If a setuid program (such as `passwd`, or `sudo`) contain a security flaw,
  such as a buffer overflow (Common in C) or improper input validation, an
  attacker can exploit the flaw.

- Since the program is running with root privileges, the attacker can execute
  shell code or commands with root access, completely compromising the entire
  system.

Normally the root user (UID 0) gets unrestricted access to almost everything on
the entire system.

I rebuild/update way too often to completely separate the accounts and allow no
admin tasks for my daily user. That may be a better option for servers, etc.

Create an admin user for administrative tasks and remove your daily user from
the `wheel` group, and disable the `sudo`, `su`, and `pkexec` SUIDs:

```nix
{ config, pkgs, lib }:
{
users.users.admin = {
    isNormalUser = true;
    description  = "System administrator";
    extraGroups  = [ "wheel" ];   # wheel = sudo
    # run `mkpasswd --method=yescrypt` and replace "changeme" w/ the result
    initialHashedPassword = "changeme";           # change with `passwd admin` later
    openssh.authorizedKeys.keys = [
      # (optional) paste your SSH public key here
      # "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
    ];
  };
    users.groups.admin = {};
    users.mutableUsers = false;

  # --------------------------------------------------------------------
  # 2. Existing daily user – remove from wheel, keep everything else
  # --------------------------------------------------------------------
  users.users.daily = {
    isNormalUser = true;
    description  = "Daily driver account";
    extraGroups  = lib.mkForce [ "networkmanager" "audio" "video" ]; # keep useful groups
    initialHashedPassword = "changeme";
    # Remove `wheel` by *not* listing it (mkForce overrides any default)
  };
  users.groups.daily = {};

security = {
# alias sudo = 'run0'
run0.enableSudoAlias = true;
polkit.enable = true;
# Disable sudo
sudo.enable = false;
wrappers = {
    su.setuid = lib.mkForce false;
    # Not needed when sudo is disabled
    # sudo.setuid = lib.mkForce false;
    # sudoedit.setuid = lib.mkForce false;
    sg.setuid = lib.mkForce false;
    fusermount.setuid = lib.mkForce false;
    fusermount3.setuid = lib.mkForce false;
    mount.setuid = lib.mkForce false;
    pkexec.setuid = lib.mkForce false;
    newgrp.setuid = lib.mkForce false;
    newgidmap.setuid = lib.mkForce false;
    newuidmap.setuid = lib.mkForce false;
};
# Or hyprlock, required for swaylock to accept your password
pam.services.swaylock = {
  text = ''
    auth include login
    account include login
    password include login
    session include login
  '';
  };
};
```

The `security.wrappers...` removes the setuid bit making the commands unusable
removing the SUID vulnerabilities for `su` and `pkexec`. You can find the other
SUID wrappers in `/run/wrappers/bin/`, such as `fusermount` and more.

SUID's that can be disabled:

- `umount`: Allows unprivileged users to unmount devices listed in your fstab.

- `mount`: Same as above but for mounting.

- `sg`: Executes a command as a different group.

- `mtr-packet`: Used by mtr to create network sockets.

- `fusermount`, `fusermount3`: Allows unprivileged users to mount FUSE
  filesystems. Can be disabled if you don't use FUSE (e.g., Appimages, etc.)

- `newuidmap`, `newgidmap`: Used for user namespace creation (Often used for
  unprivileged containers). (Disable if you don't use unprivileged
  containers/namespaces)

---

Never Disable:

- `unix_chkpwd`: This is a core PAM helper to securely check user passwords
  against the root-readable `/etc/shadow`.

Check again which SUID binaries are active:

```bash
sudo find / -perm -4000 -type f -ls 2>/dev/null
```

---

You will have to use `run0` to authenticate your daily user, for example:

```bash
run0 nixos-rebuild switch --flake .
```

Since `run0` doesn't cache results and `nixos-rebuild` calls on Polkit 3 times,
so on every rebuild, you will be asked for your password 3 times which isn't
ideal. I found the following workaround that will only ask for your password
once.

Add the following to your `configuration.nix`, replacing `user-name` with your
username:

```nix
 security.polkit.extraConfig = ''
     polkit.addRule(function(action, subject) {
       if (subject.user == "user-name") {
         if (action.id.indexOf("org.nixos") == 0) {
           polkit.log("Caching admin authentication for single NixOS operation");
           return polkit.Result.AUTH_ADMIN_KEEP;
         }
       }
     });
   '';
```

Create a zsh function for easy access:

```nix
# zsh.nix
#...snip...
initContent = ''
  fr() {
    run0 nixos-rebuild switch --flake "/home/$USER/flake#"$(hostname)
  }
'';
```

Needless to say, this is less secure but much more convenient than entering your
password 3 times on every single rebuild.

Without the `pam` settings for swaylock, it won't accept your password to log
back in.

**run0 Usage Example**

When you are in a privileged shell, `run0` changes the color of the background
to red to remind you of this.

Example creating a user:

1. `run0`

2. `adduser admin`

3. `usermod -aG wheel admin`

4. `passwd admin`

5. `exit`

6. `reboot`

This is just an example, since we manage our users declaratively the user
created would be discarded on the next rebuild because of the
`users.mutableUsers = false;` setting. You could of course change this to `true`
to manage your users imperatively but I don't recommend it.

---

### Capabilities

<details>

<summary> ✔️ Click to expand capabilities examples </summary>

One way to help get rid of setuid binaries is to replace them with capabilities.
I personally only remove the SUID bit and don't try to replace with capabilities
as of now. You can still use the commands from `security.wrappers` such as
`run0 su -`.

Capabilities provide a subset of what is available to root to a process. This
breaks up root privileges into smaller units that can independently grant access
to processes. This reduces the full set of privileges, decreasing the risk of
exploitation.

(This is just an example):

```nix
{
  # a setuid root program
  doas =
    { setuid = true;
      owner = "root";
      group = "root";
      source = "${pkgs.doas}/bin/doas";
    };

  # a setgid program
  locate =
    { setgid = true;
      owner = "root";
      group = "mlocate";
      source = "${pkgs.locate}/bin/locate";
    };

  # a program with the CAP_NET_RAW capability
  ping =
    { owner = "root";
      group = "root";
      capabilities = "cap_net_raw+ep";
      source = "${pkgs.iputils.out}/bin/ping";
    };
}
```

List the highest capability number for your kernel with:

```bash
cat /proc/sys/kernel/cap_last_cap
# Output:
40
```

List available Linux capabilities:

```bash
capsh --print
```

List processes:

```bash
ps
# Example Output
PID    TTY     TIME   CMD
8063   pts/1    02     zsh
```

```bash
cat /proc/8063/status | grep Cap
# Output
CapInh: 0000000800000000
CapPrm: 0000000000000000
CapEff: 0000000000000000
CapBnd: 000001ffffffffff
CapAmb: 0000000000000000
```

```bash
capsh --decode=000001ffffffffff
# Output
0x000001ffffffffff=cap_chown,cap_dac_override,cap_dac_read_search,cap_fowner,cap_fsetid,cap_kill,cap_setgid,cap_setuid,cap_setpcap,cap_linux_immutable,cap_net_bind_service,cap_net_broadcast,cap_net_admin,cap_net_raw,cap_ipc_lock,cap_ipc_owner,cap_sys_module,cap_sys_rawio,cap_sys_chroot,cap_sys_ptrace,cap_sys_pacct,cap_sys_admin,cap_sys_boot,cap_sys_nice,cap_sys_resource,cap_sys_time,cap_sys_tty_config,cap_mknod,cap_lease,cap_audit_write,cap_audit_control,cap_setfcap,cap_mac_override,cap_mac_admin,cap_syslog,cap_wake_alarm,cap_block_suspend,cap_audit_read,cap_perfmon,cap_bpf,cap_checkpoint_restore
```

`cap_net_raw`: Allows the program to use raw and unbuffered network sockets,
which is what `ping` and `mtr-packet` need to send ICMP packets.

`cap_sys_admin`: Grants a variety of system administration operations, including
the ability to perform FUSE mounts. This is a powerful capability, but it's
still more restrictive than full root SUID.

- `+ep`: This is crucial. It stands for:
  - `e` (Effective): The set of capabilities actually used by the process when
    running.

  - `p` (Permitted): The set of capabilities that can be enabled by the process.

By using this approach, you are following the security principle of least
privilege, significantly reducing the attack surface compared to traditional
SUID binaries.

- [security.wrappers](https://search.nixos.org/options?channel=unstable&show=security.wrappers&query=security.wrappers)

- [Linux Audit capabilities 101](https://linux-audit.com/kernel/capabilities/linux-capabilities-101/)

- [Kicksecure's take on capabilities](https://www.kicksecure.com/wiki/Dev/secureblue#capabilities)

- [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html)

- [capabilities and seccomp](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux_atomic_host/7/html/container_security_guide/linux_capabilities_and_seccomp)

</details>

---

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

## Replace timesyncd with a chron job that enables Network Time Security (NTS)

This is implementing the GrapheneOS/secureblue NTS chrony settings to NixOS:

```nix
{ config
, ...
}:
{
  services.chrony = {
    enable = true;
    enableNTS = true;
    servers = [
        "server time.cloudflare.com iburst nts"
        "server ntppool1.time.nl iburst nts"
        "server nts.netnod.se iburst nts"
        "server ptbtime1.ptb.de iburst nts"
        "server time.dfm.dk iburst nts"
        "server time.cifelli.xyz iburst nts"
     ];
    # havent worked out the kinks yet
  #  extraConfig = ''
  #      minsources 3
  #      authselectmode require

  #      # EF
  #      dscp 46

  #      driftfile /var/lib/chrony/drift
  #      dumpdir /var/lib/chrony
  #      ntsdumpdir /var/lib/chrony

  #      leapseclist /usr/share/zoneinfo/leap-seconds.list
  #      makestep 1.0 3

  #      rtconutc

  #      cmdport 0

  #      noclientlog
  #  '';
  };
}
```

Ensure NTS is being used with:

```bash
sudo chronyc -N authdata
```

---

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

---

### The Kernel

The Kernel Self Protection Project:

- [KSPP Recommended_Settings](https://kspp.github.io/Recommended_Settings)

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

---

### The Hardened Kernel

> NOTE: Expect breakage when using the hardened kernel. `linux-hardened`
> completely disables
> [unprivileged user namespaces](https://secureblue.dev/articles/userns), which
> are required for Flatpak, chromium-based browsers, and more.

The `linuxPackages_latest_hardened` attribute has been deprecated. If you want
to use a hardened kernel, it is now recommended to use `linux_hardened`, which
is aliased to `linux_default.kernel`.

You can find the latest available hardened kernel packages by searching
[pkgs/top-level/linux-kernels.nix](https://github.com/NixOS/nixpkgs/blob/master/pkgs/top-level/linux-kernels.nix).

It is recommended to use `linux_hardened` without specifying a version, such as:

```nix
boot.kernelPackages = pkgs.linuxPackages_hardened;
```

`linux_hardened` is aliased to the `linux_default.kernel`.

Note that this not only replaces the kernel, but also packages that are specific
to the kernel version, such as NVIDIA video drivers. This also removes your
ability to use the `.extend` kernel attribute, they are only available to
_kernel package sets_ (e.g., `linuxPackages_hardened`)

- If you decide to use this, read further before rebuilding.

You can inspect
[nixpkgs/pkgs/os-specific/linux/kernel/hardened/patches.json](https://github.com/NixOS/nixpkgs/blob/master/pkgs/os-specific/linux/kernel/hardened/patches.json)
to see the metadata of the patches that are applied. You can then follow the
links in the `.json` file to see the patch diffs.

---

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

---

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

---

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

Also see the
[Kernel Self Protection Projects sysctls](https://kspp.github.io/Recommended_Settings#sysctls)

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
    # "kernel.unprivileged_userns_clone" = 0; # Set to 1 because it makes NH and other programs fail
    # This should be set to 0 if you don't rely on flatpak, NH, Docker, etc.
    "kernel.unprivileged_userns_clone" = 1;
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

---

## Hardening Boot Parameters

`boot.kernelParams` can be used to set additional kernel command line arguments
at boot time. It can only be used for built-in modules.

You can find the following settings in the
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

---

**Hardening Modprobe**

You can use both `extraModprobeConfig` & `blacklistedKernelModules` to disable
different features. If you prefer, you can place these in the next section as
well.

```nix
boot.extraModprobeConfig = ''
     # firewire and thunderbolt
    install firewire-core /bin/false
    install firewire_core /bin/false
    install firewire-ohci /bin/false
    install firewire_ohci /bin/false
    install firewire_sbp2 /bin/false
    install firewire-sbp2 /bin/false
    install firewire-net /bin/false
    install thunderbolt /bin/false
    install ohci1394 /bin/false
    install sbp2 /bin/false
    install dv1394 /bin/false
    install raw1394 /bin/false
    install video1394 /bin/false
'';
# OR
#boot.blacklistedKernelModules = [
#  "firewire-core"
#  # ... snip ...
#];
```

---

**Blacklisting Kernel Parameters**

Blacklisting unused kernel modules reduces the attack surface.

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

Also see
[SecureBlue's blacklist.conf](https://github.com/secureblue/secureblue/blob/live/files/system/etc/modprobe.d/blacklist.conf)
for more ideas.

---

## Hardened Memory Allocator

> NOTE: There is a performance cost to enabling a hardened memory allocator, and
> some apps will not work without a workaround such as Firefox, Thunderbird,
> Torbrowser, LibreWolf, and ZenBrowser to name a few.

With memory corruption bugs being the leading zero day category, it's clearly
something that you should be concerned with.

The grapheneOS `hardened_malloc` is available for NixOS in two variants, add
either to your `configuration.nix` or equivalent to apply them:

1. `environment.memoryAllocator.provider = "graphene-hardened";`: This is the
   default configuration template that has all normal optional security features
   enabled. It's aggressive, you can expect app breakage and a performance cost.

2. `environment.memoryAllocator.provider = "graphene-hardened-light";`: The
   light template disables the slap quarantines, write after free check, slot
   randomization and raises the guard slab interval from 1 to 8 but leaves
   zero-on-free and slab canaries enabled. This version has solid performance
   and is still far more secure than the standard allocator.

`libhardened_malloc.so` is typically installed to
`/usr/local/lib/libhardened_malloc.so` and referenced from `/etc/ld.so.preload`.

- [NixOS Manual memoryAllocator](https://nixos.org/manual/nixos/stable/options#opt-environment.memoryAllocator.provider)

- [GrapheneOS hardened_malloc](https://github.com/GrapheneOS/hardened_malloc?tab=readme-ov-file#traditional-linux-based-operating-systems)

- [GrapheneOS/secureblue discussion on hardened_malloc issues](https://github.com/secureblue/secureblue/issues/193#issuecomment-1953323680)

- [ld.so man page](https://man7.org/linux/man-pages/man8/ld.so.8.html)

- [Exploring hardened_malloc](https://www.synacktiv.com/en/publications/exploring-grapheneos-secure-allocator-hardened-malloc)

---

## Hardening Systemd

<!-- ![Hacker](../images/hacker.png) -->

`systemd` is the core "init system" and service manager that controls how
services, daemons, and basic system processes are started, stopped and
supervised on modern Linux distributions, including NixOS. It provides a suite
of basic building blocks for a Linux system as well as a system and service
manager that runs as `PID 1` and starts the rest of the system.

Because it launches and supervises almost all system services, hardening systemd
means raising the baseline security of your entire system.

Disable coredumps:

```nix
systemd.coredump.enable = false;
# ➡️ Sets the kernel's resource limit (ulimit -c 0)
  security.pam.loginLimits = [
    {
      domain = "*"; # Applies to all users/sessions
      type = "-"; # Set both soft and hard limits
      item = "core"; # The soft/hard limit item
      value = "0";   # Core dumps size is limited to 0 (effectively disabled)
    }
  ];
```

Disabling coredumps helps save space and improves security/privacy because when
a program fails, a coredump contains an exact copy of a programs running memory
at the time of the crash. This can inadvertently expose sensitive data.

If a program is handling private information when it crashes, the core dump file
could contain:

- **Passwords**: Stored in memory before being sent or hashed.

- **Encryption Keys**: Used for securing network connections.

- **Personal Info**: Chat messages, website forms, etc.

It can give a minor performance upgrade and does reduce the attack surface. If a
malicious program were to gain access to your system, one of the first things it
might look for are core dump files to extract sensitive data. By disabling them,
you eliminate this potential source of information leakage.

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

Optionally disable vulnerable services to reduce the attack surface, obviously
don't disable what you need, or change your habits:

```nix
services = {
    # mDNS/DNS-SD
    avahi.enable = false;
    # Geoclue (location services)
    geoclue2.enable = false;
    udisks2.enable = false;
    accounts-daemon.enable = false;
  };
  # Only needed for WWAN/3G/4G modems, otherwise it runs `mmcli` unnecessarily
  networking.modemmanager.enable = false;
  # Bluetooth has a long history of vulnerabilities
  hardware.bluetooth.enable = false;
  # Prefer manual upgrades on a hardened system
  system.autoUpgrade.enable = false;
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

<details>
<summary> Click to expand `systemd.nix` example implementing many of the recommendations </summary>

```nix
{lib, ...}: {
  systemd.services = {
    # "home-manager-jr".after = ["network-online.target"];
    # "home-manager-jr".wantedBy = ["multi-user.target"];
    "user@".serviceConfig = {
      ProtectSystem = "strict";
      ProtectClock = true;
      ProtectHostname = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      ProtectProc = "invisible";
      PrivateTmp = true;
      PrivateNetwork = true;
      MemoryDenyWriteExecute = false;
      RestrictAddressFamilies = [
        "AF_UNIX"
        "AF_NETLINK"
        "AF_BLUETOOTH"
      ];
      RestrictNamespaces = true;
      RestrictRealtime = true;
      RestrictSUIDSGID = true;
      SystemCallFilter = [
        "~@keyring"
        "~@swap"
        "~@debug"
        "~@module"
        "~@obsolete"
        "~@cpu-emulation"
      ];
      SystemCallArchitectures = "native";
    };
    acpid.serviceConfig = {
      ProtectSystem = "full";
      ProtectHome = true;
      RestrictAddressFamilies = ["AF_INET" "AF_INET6"];
      SystemCallFilter = "~@clock @cpu-emulation @debug @module @mount @raw-io @reboot @swap";
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
    };

    auditd.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "full";
      ProtectHome = true;
      ProtectHostname = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectControlGroups = true;
      ProtectProc = "invisible";
      ProtectClock = true;
      PrivateTmp = true;
      PrivateNetwork = true;
      PrivateMounts = true;
      PrivateDevices = true;
      RestrictNamespaces = true;
      RestrictRealtime = true;
      RestrictSUIDSGID = true;
      RestrictAddressFamilies = [
        "~AF_INET6"
        "~AF_INET"
        "~AF_PACKET"
      ];
      MemoryDenyWriteExecute = true;
      LockPersonality = true;
      SystemCallFilter = [
        "~@clock"
        "~@module"
        "~@mount"
        "~@swap"
        "~@obsolete"
        "~@cpu-emulation"
      ];
      SystemCallArchitectures = "native";
      CapabilityBoundingSet = [
        "~CAP_CHOWN"
        "~CAP_FSETID"
        "~CAP_SETFCAP"
      ];
    };

    cups.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "full";
      ProtectHome = true;
      ProtectKernelModules = true;
      ProtectKernelTunables = true;
      ProtectKernelLogs = true;
      ProtectControlGroups = true;
      ProtectHostname = true;
      ProtectClock = true;
      ProtectProc = "invisible";
      RestrictRealtime = true;
      RestrictNamespaces = true;
      RestrictSUIDSGID = true;
      RestrictAddressFamilies = [
        "AF_UNIX"
        "AF_NETLINK"
        "AF_INET"
        "AF_INET6"
        "AF_PACKET"
      ];

      MemoryDenyWriteExecute = true;
      SystemCallFilter = [
        "~@clock"
        "~@reboot"
        "~@debug"
        "~@module"
        "~@swap"
        "~@obsolete"
        "~@cpu-emulation"
      ];
      SystemCallArchitectures = "native";
      LockPersonality = true;
    };

    NetworkManager.serviceConfig = {
      NoNewPrivileges = true;
      ProtectHome = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      ProtectControlGroups = true;
      ProtectClock = true;
      ProtectHostname = true;
      ProtectProc = "invisible";
      PrivateTmp = true;
      RestrictRealtime = true;
      RestrictAddressFamilies = [
        "AF_UNIX"
        "AF_NETLINK"
        "AF_INET"
        "AF_INET6"
        "AF_PACKET"
      ];
      RestrictNamespaces = true;
      RestrictSUIDSGID = true;
      MemoryDenyWriteExecute = true;
      SystemCallFilter = [
        "~@mount"
        "~@module"
        "~@swap"
        "~@obsolete"
        "~@cpu-emulation"
        "ptrace"
      ];
      SystemCallArchitectures = "native";
      LockPersonality = true;
    };

    wpa_supplicant.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "strict";
      ProtectHome = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      ProtectControlGroups = true;
      ProtectClock = true;
      ProtectHostname = true;
      ProtectProc = "invisible";
      PrivateTmp = true;
      PrivateMounts = true;
      RestrictRealtime = true;
      RestrictAddressFamilies = [
        "AF_UNIX"
        "AF_NETLINK"
        "AF_INET"
        "AF_INET6"
        "AF_PACKET"
      ];
      RestrictNamespaces = true;
      RestrictSUIDSGID = true;
      MemoryDenyWriteExecute = true;
      SystemCallFilter = [
        "~@mount"
        "~@raw-io"
        "~@privileged"
        "~@keyring"
        "~@reboot"
        "~@module"
        "~@swap"
        "~@resources"
        "~@obsolete"
        "~@cpu-emulation"
        "ptrace"
      ];
      SystemCallArchitectures = "native";
      LockPersonality = true;
      CapabilityBoundingSet = "CAP_NET_ADMIN CAP_NET_RAW";
    };

    dbus.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "stric";
      ProtectControlGroups = true;
      ProtectHome = true;
      ProtectHostname = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      PrivateMounts = true;
      PrivateDevices = true;
      PrivateTmp = true;
      RestrictSUIDSGID = true;
      RestrictRealtime = true;
      RestrictAddressFamilies = [
        "AF_UNIX"
      ];
      RestrictNamespaces = true;
      SystemCallErrorNumber = "EPERM";
      SystemCallArchitectures = "native";
      SystemCallFilter = [
        "~@obsolete"
        "~@resources"
        "~@debug"
        "~@mount"
        "~@reboot"
        "~@swap"
        "~@cpu-emulation"
      ];
      LockPersonality = true;
      IPAddressDeny = ["0.0.0.0/0" "::/0"];
      MemoryDenyWriteExecute = true;
      DevicePolicy = "closed";
      UMask = 0077;
    };

    nscd.serviceConfig = {
      ProtectClock = true;
      ProtectHostname = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      ProtectControlGroups = true;
      ProtectProc = "invisible";
      RestrictNamespaces = true;
      RestrictRealtime = true;
      MemoryDenyWriteExecute = true;
      LockPersonality = true;
      SystemCallFilter = [
        "~@mount"
        "~@swap"
        "~@clock"
        "~@obsolete"
        "~@cpu-emulation"
      ];
      SystemCallArchitectures = "native";
      CapabilityBoundingSet = [
        "~CAP_CHOWN"
        "~CAP_FSETID"
        "~CAP_SETFCAP"
      ];
    };
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
    systemd-rfkill.serviceConfig = {
      ProtectSystem = "strict";
      ProtectHome = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectControlGroups = true;
      ProtectClock = true;
      ProtectProc = "invisible";
      ProcSubset = "pid";
      PrivateTmp = true;
      MemoryDenyWriteExecute = true;
      NoNewPrivileges = true;
      LockPersonality = true;
      RestrictRealtime = true;
      SystemCallArchitectures = "native";
      UMask = "0077";
      IPAddressDeny = "any";
    };
    systemd-machined.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "strict";
      ProtectHome = true;
      ProtectClock = true;
      ProtectHostname = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectKernelLogs = true;
      ProtectProc = "invisible";
      PrivateTmp = true;
      PrivateMounts = true;
      PrivateUsers = true;
      PrivateNetwork = true;
      RestrictNamespaces = true;
      RestrictRealtime = true;
      RestrictSUIDSGID = true;
      RestrictAddressFamilies = ["AF_UNIX"];
      MemoryDenyWriteExecute = true;
      SystemCallArchitectures = "native";
    };
    systemd-udevd.serviceConfig = {
      NoNewPrivileges = true;
      ProtectSystem = "strict";
      ProtectHome = true;
      ProtectKernelLogs = true;
      ProtectControlGroups = true;
      ProtectClock = true;
      ProtectProc = "invisible";
      RestrictNamespaces = true;
      CapabilityBoundingSet = "~CAP_SYS_PTRACE ~CAP_SYS_PACCT";
    };
    nix-daemon.serviceConfig = {
      NoNewPrivileges = true;
      ProtectControlGroups = true;
      ProtectKernelModules = true;
      PrivateMounts = true;
      PrivateTmp = true;
      PrivateDevices = true;
      RestrictSUIDSGID = true;
      RestrictRealtime = true;
      RestrictNamespaces = ["~cgroup"];
      RestrictAddressFamilies = [
        "AF_UNIX"
        "AF_NETLINK"
        "AF_INET6"
        "AF_INET"
      ];
      CapabilityBoundingSet = [
        "~CAP_SYS_CHROOT"
        "~CAP_BPF"
        "~CAP_AUDIT_WRITE"
        "~CAP_AUDIT_CONTROL"
        "~CAP_AUDIT_READ"
        "~CAP_SYS_PTRACE"
        "~CAP_SYS_NICE"
        "~CAP_SYS_RESOURCE"
        "~CAP_SYS_RAWIO"
        "~CAP_SYS_TIME"
        "~CAP_SYS_PACCT"
        "~CAP_LINUX_IMMUTABLE"
        "~CAP_IPC_LOCK"
        "~CAP_WAKE_ALARM"
        "~CAP_SYS_TTY_CONFIG"
        "~CAP_SYS_BOOT"
        "~CAP_LEASE"
        "~CAP_BLOCK_SUSPEND"
        "~CAP_MAC_ADMIN"
        "~CAP_MAC_OVERRIDE"
      ];
      SystemCallErrorNumber = "EPERM";
      SystemCallArchitectures = "native";
      SystemCallFilter = [
        "~@resources"
        "~@module"
        "~@obsolete"
        "~@debug"
        "~@reboot"
        "~@swap"
        "~@cpu-emulation"
        "~@clock"
        "~@raw-io"
      ];
      LockPersonality = true;
      MemoryDenyWriteExecute = false;
      DevicePolicy = "closed";
      UMask = 0077;
    };
    systemd-journald.serviceConfig = {
      NoNewPrivileges = true;
      ProtectProc = "invisible";
      ProtectHostname = true;
      PrivateMounts = true;
    };
  };
}
```

</details>

---

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

```text
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

---

## Key generation

### ssh-keygen

The `ed25519` algorithm is significantly faster and more secure when compared to
`RSA`. You can also specify the key derivation function (KDF) rounds to
strengthen protection even more.

For example, to generate a strong key for GitHub:

```bash
ssh-keygen -t ed25519 -a 32 -f ~/.ssh/id_ed25519_github_$(date +%Y-%m-%d) -C "SSH Key for GitHub"
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
algorithms, and best practices: (EDITED: 10-07-25 to follow best-practices on
post-quantum crypto)

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
            # Post-Quantum: https://www.openssh.org/pq.html
            "mlkem768x25519-sha256"
            "sntrup761x25519-sha512"
            "curve25519-sha256@libssh.org"
            "ecdh-sha2-nistp521"
            "ecdh-sha2-nistp384"
            "ecdh-sha2-nistp256"
            "diffie-hellman-group-exchange-sha256"
          ];
          Ciphers = [
            "aes256-gcm@openssh.com"
            "aes128-gcm@openssh.com"
            # stream cipher alternative to aes256, proven to be resilient
            # Very fast on basically anything
            "chacha20-poly1305@openssh.com"
            # industry standard, fast if you have AES-NI hardware
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

---

## Encrypted Secrets

Never store secrets in plain text in repositories. Use something like
[sops-nix](https://github.com/Mic92/sops-nix), which lets you keep encrypted
secrets under version control declaratively.

Another option is [agenix](https://github.com/ryantm/agenix)

- [NixOS Wiki Agenix](https://wiki.nixos.org/wiki/Agenix)

### Sops-nix Guide

Protect your secrets, the following guide is on setting up Sops on NixOS:
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)

---

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

---

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

---

## Doas over sudo (Warning Doas is unmaintained)

<details>
<summary> ✔️ Click to Expand Unmaintained Doas example </summary>

> NOTE: I have moved to `run0` for authentication which is included by default
> with systemd. It's actually a symlink to the existing `systemd-run` tool. It
> behaves like a secure `sudo` alternative: it spawns a transient service under
> PID 1 for privilege escalation, without relying on SUID (set user ID)
> binaries.

> ⚠️ Warning the Nixpkgs version of `doas`,
> [OpenDoas](https://github.com/Duncaen/OpenDoas) is unmaintained and hasn't
> been updated in 3 to 4 years. If you don't like `run0`, consider using
> `sudo-rs`. I'm leaving this here for now, may remove it in the future to not
> promote using unmaintained software, you've been warned.

- [Why run0](https://mastodon.social/@pid_eins/112353324518585654)

- SUID = "Set User ID": When a binary has the SUID bit set, it runs with the
  privileges of the file's owner (often root). There is a long history of
  vulnerabilities with SUID binaries.

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

</details>

---

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
> their sandboxing technologies operate. Flatpak also won't work with the
> hardened kernel because they require unprivileged user namespaces which the
> hardened kernel completely disables.

- [Flatpak permissions & What they Do](https://docs.flatpak.org/en/latest/sandbox-permissions.html#permissions-guidelines)
  Reference this while setting permissions with Flatseal, many apps come with
  more permissions than they need to function effectively breaking the sandbox.

- [Portals](https://docs.flatpak.org/en/latest/sandbox-permissions.html#portals)
  provide mediated, user-controlled access to host resources outside the
  sandbox, so apps don’t need broad blanket permissions.

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
xdg = {
  portal = {
    enable = true;
    extraPortals = [ pkgs.xdg-desktop-portal-gtk ];
    config.common.default = [ "gtk" ];
  };
};
# Disables screencopy
systemd.user.services."xdg-desktop-portal-wlr" = {
  enable = false;
};
```

<details>
<summary> ✔️ declarative-flatpak </summary>

1. Add the flake input to your `flake.nix`:

```nix
inputs = {
  flatpaks.url = "github:in-a-dil-emma/declarative-flatpak/latest";
};
```

2. The following is a NixOS module that installs Firefox & Bitwarden:

```nix
# flatpak.nix
{
  pkgs,
  inputs,
  ...
}: {
  imports = [
    inputs.flatpaks.nixosModules.default
  ];
  services.flatpak = {
    enable = true;
    remotes = {
      "flathub" = "https://dl.flathub.org/repo/flathub.flatpakrepo";
      # "flathub-beta" = "https://dl.flathub.org/beta-repo/flathub-beta.flatpakrepo";
    };
    packages = [
      "flathub:app/org.mozilla.firefox//stable"
      "flathub:app/com.bitwarden.desktop//stable"
      # "flathub-beta:app/org.kde.kdenlive/x86_64/stable"
      # ":${./foobar.flatpak}"
      "flathub:/root/testflatpak.flatpakref"
    ];
    overrides = {
      # note: "global" is a flatpak thing
      # if you ever ran "flatpak override" without specifying a ref you will know
      "global".Context = {
        filesystems = [
          "home"
        ];
        sockets = [
          "!wayland"
          "!fallback-x11"
        ];
      };
      "org.mozilla.Firefox" = {
        Environment = {
          "MOZ_ENABLE_WAYLAND" = 1;
        };
        Context.sockets = [
          "!wayland"
          "!fallback-x11"
          # "x11"
        ];
      };
    };
  };
  xdg.portal = {
    enable = true;
    extraPortals = [
      pkgs.xdg-desktop-portal-gtk
    ];
    config = {
      common.default = ["gtk"];
    };
  };
}
```

> NOTE: I got the above configuration to build successfully, one of the
> hardening steps I took isn't allowing either app to launch. I'll update once I
> find which setting it is exactly. (01-13-26)

</details>

- [Flathub Verified Apps](https://docs.flathub.org/docs/for-users/verification)

- [Flatpak the good the bad the ugly](https://secureblue.dev/articles/flatpak)

Then you can either find apps through [FlatHub](https://flathub.org/en) or on
the cmdline with `flatpak search <app>`. Flatpak is best used for GUI apps, some
CLI apps can be installed through it but not all.

- There is also [nix-flatpak](https://github.com/gmodena/nix-flatpak), which
  enables you to manage your flatpaks declaratively.

- [Flatseal](https://flathub.org/en/apps/com.github.tchx84.Flatseal) is GUI
  utility that enables you to review and modify permissions from your Flatpak
  apps. Many apps by default come with smart-card support, X11 & Wayland
  support, and more, disabling unnecessary permissions is recommended.

- [Warehouse](https://flathub.org/en/apps/io.github.flattool.Warehouse) provides
  a simple UI to control complex Flatpak options, no cmdline required.

I have heard that it is not recommended to use Flatpak browsers because in order
for flatpak to work it has to disable some of the built-in browser sandboxing
which can reduce security. I haven't found any examples of Flatpak browsers
being exploited but it's something to keep in mind.

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

---

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

<details>
<summary> ✔️ Click to Expand Resources </summary>

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

- [secureblue FAQ](https://secureblue.dev/faq)

- [Excellent Kicksecure Docs](https://www.kicksecure.com/wiki/Documentation)

- [Awesome-Security-Hardening List](https://github.com/decalage2/awesome-security-hardening)

- [factorable.net (study of RSA and DSA crypto keys) FAQ](https://factorable.net/faq.html)

- [The cr.yp.to blog Entropy](https://blog.cr.yp.to/20140205-entropy.html)

- [NixOS Security wishlist](https://delroth.net/posts/nixos-security-wishlist/)

- [Beejus IPC guide](https://beej.us/guide/bgipc/html/)

- [GeeksforGeeks IPC](https://www.geeksforgeeks.org/operating-systems/inter-process-communication-ipc/)

neal.codes vulnerability scan script:

```bash
nix-shell -p grype sbomnix --run '
  sbomnix /run/current-system --csv /dev/null --spdx /dev/null --cdx sbom.cdx.json;
  grype sbom.cdx.json
'
```

- [neal.codes nixos-stig-anduril](https://github.com/nealfennimore/nixos-stig-anduril)

- [Suse Linux Hardening Guide](https://www.suse.com/c/linux-hardeningthe-complete-guide-to-securing-your-systems/)

**Government Resources 1st 6 come from gentoo's Security_Handbook)**

- [The Austrailian Cyber Security Centre's Informational Security Manual (ISM)](https://www.cyber.gov.au/sites/default/files/2023-03/Information%20Security%20Manual%20-%20%28March%202023%29.pdf)

- [The Australian Government's Protective Security Policy Framework (PSPF)](https://www.protectivesecurity.gov.au/policies)

- [The Australian Cyber Security Centre's Protect Yourself page](https://www.cyber.gov.au/protect-yourself)

- [The UK Government's Security Policy Framework (SPF)](https://www.gov.uk/government/publications/security-policy-framework/hmg-security-policy-framework)

- [The UK Government's Information Security Policy Framework (ISF)](https://www.gov.uk/government/publications/information-security-policy-framework)

- [The US National Institute of Standards and Technology's Cybersecurity page](https://www.nist.gov/cybersecurity)
- [NixOS STIG](https://stigviewer.com/stigs/anduril_nixos)

- STIGs are configuration standards developed by the Defense Information Systems
  Agency (DISA) to secure systems and software for the U.S. Department of
  Defense (DoD). They are considered a highly authoritative source for system
  hardening.There are recommendations for hardening all kinds of software in the
  [Stig Viewer](https://stigviewer.com/stigs)

- [CIS Benchmarks](https://www.cisecurity.org/cis-benchmarks)

- [NSA Cybersecurity Directorate](https://github.com/nsacyber)

</details>
