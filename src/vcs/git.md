# Version Control with Git

First, I'll breefly explain some of the limitations of NixOS Rollbacks and then
I'll explain how Git compliments them.

## Limitations of NixOS Rollbacks

NixOS is famous for its ability to roll back to previous system generations,
either from the boot menu or with commands like `nixos-rebuild --rollback`.

When you perform rollbacks in NixOS, whether from the boot menu or using
commands like `nixos-rebuild --rollback` only the contents and symlinks managed
by the Nix store are affected. The rollback works by switching which system
generation is active, atomically updating symlinks to point to the previous
version of all packages, systemd units and services stored in `/nix/store`.

However, it’s important to understand what these rollbacks actually do and what
they don’t do. What NixOS Rollbacks Cover

- System generations: When you rebuild your system, NixOS creates a new
  “generation” that you can boot into or roll back to. This includes all
  packages, services, and system configuration managed by Nix.

- Quick recovery: If an upgrade breaks your system, you can easily select an
  older generation at boot and get back to a working state

**Key Limitations**:

- Configuration files are not reverted: Rolling back only changes which system
  generation is active, it does not revert your actual configuration files (like
  `configuration.nix` or your flake files)

- User data and service data are not rolled back: Only files managed by Nix are
  affected. Databases, user files, and other persistent data remain unchanged,
  which can cause problems if, for example, a service migrates its database
  schema during an upgrade

- Manual changes persist: Any manual edits to configuration files or system
  state outside of Nix are not reverted by a rollback

## How Git Helps

- Tracks every configuration change: By version-controlling your NixOS configs
  with Git, you can easily see what changed, when, and why.

- True config rollback: If a configuration change causes issues, you can use
  `git checkout` or `git revert` to restore your config files to a previous good
  state, then rebuild your system

- Safer experimentation: You can confidently try new settings or upgrades,
  knowing you can roll back both your system state (with NixOS generations) and
  your config files (with Git).

- Collaboration and backup: Git lets you share your setup, collaborate with
  others, and restore your configuration if your machine is lost or damaged.

In summary: NixOS rollbacks are powerful for system state, but they don’t manage
your configuration file history. Git fills this gap, giving you full control and
traceability over your NixOS configs making your system both robust and truly
reproducible. Version control is a fundamental tool for anyone working with
NixOS, whether you’re customizing your desktop, managing servers, or sharing
your configuration with others. Git is the most popular version control system
and is used by the NixOS community to track, share, and back up system
configurations.

Why use Git with NixOS?

- Track every change: Git lets you record every modification to your
  configuration files, so you can always see what changed, when, and why.

- Experiment safely: Try new settings or packages without fear—if something
  breaks, you can easily roll back to a previous working state.

- Sync across machines: With Git, you can keep your NixOS setups in sync between
  your laptop, desktop, or servers, and collaborate with others.

- Disaster recovery: Accidentally delete your config? With Git, you can restore
  it from your repository in minutes.

Installing Git on NixOS

You can install Git by adding it to your system packages in your
configuration.nix or via Home Manager:

### A Basic Git Workflow

1. Initialize your Repository:

If you haven't already created a Git repo in your NixOS config directory (for
example, in your flake or `/etc/nixos`):

```bash
cd ~/flake
git init
git add .
git commit -m "Initial commit: NixOS Configuration"
```

Taking this initial snapshot with Git is a best practice—it captures the exact
state of your working configuration before you make any changes.

- The command `git add .` stages all files in the directory (and its
  subdirectories) for commit, meaning Git will keep track of them in your
  project history.

- The command `git commit -m "message"` then saves a snapshot of these staged
  files, along with your descriptive message, into the repository.

  - Think of a commit as a "save point" in your project. You can always go back
    to this point if you need to, making it easy to experiment or recover from
    mistakes. This two-step process, staging with `git add` and saving with
    `git commit` is at the heart of how Git tracks and manages changes over
    time.

2. Make and Track Changes:

Now that you've saved a snapshot of your working configuration, you're free to
experiment and try new things, even if they might break your setup.

Suppose you you want to try a new desktop environment, like Xfce. You edit your
`configuration.nix` to add:

```nix
services.xserver.desktopManager.xfce.enable = true;
```

You run:

```bash
sudo nixos-rebuild switch # if configuration.nix is in /etc/nixos/
```

but something goes wrong: the system boots, but your desktop is broken or won't
start. You decide to roll back using the boot menu or:

```bash
sudo nixos-rebuild switch --rollback
```

**What happens?**

- Your system reverts to the previous working generation in `/nix/store`

- But: Your `configuration.nix` file is still changed, it still has the line
  enabling Xfce. If you rebuild again, you'll get the same broken system,
  because your config itself wasn't rolled back.

**How does Git Help on Failure?**

Git gives you quite a few options and ways to inspect what has been done.

- Use `git status` to see what's changed, and `git checkout -- <file>` to
  restore any file to its last committed state.

- Review your changes with `git diff` to see exactly what you modified before
  deciding whether to keep or revert those changes.

- Reset everything with `git reset --hard HEAD`, this will discard all local
  changes and return to your last commit.

With Git you can simply run:

```bash
git checkout HEAD~1 configuration.nix
# or, if you committed before the change:
git revert <commit-hash>
```

Show the full hash of the latest commit:

```bash
git rev-parse HEAD
f53fef375d89496c0174e70ce94993d43335098e
```

Short hash:

```bash
git log --pretty=format:'%h' -n 1
f53fef3
git revert f53fef3
```

Show a list of Recent commits:

```bash
git log
# a list of all commits, with hashes, author, date, and message
git log --oneline
git log --oneline
f53fef3 (HEAD -> main) thunar
b34ea22 thunar
801cbcf thunar
5e72ba5 sops
8b67c59 sops
1a353cb sops
```

You can copy the commit hash from any of these and use it in commands like
`git checkout <hash>` or `git revert <hash>`.

**Commit successful experiments**

- If your changes work, stage, and commit them:

```bash
git add .
# or more specifically the file you changed or created
git add configuration.nix
git commit -m "Describe the new feature or fix"
```

### Basic Branching
