---
title: Git
date: 2025-11-30
author: saylesss88
description: Git
collection: "notes"
tags: ["vcs", "git"]
---

# Version Control with Git

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![Git Logo](../images/git1.png) -->

⚠️ **Important**: Never commit secrets (passwords, API keys, tokens, etc.) in
plain text to your Git repository. If you plan to publish your NixOS
configuration, always use a secrets management tool like sops-nix or agenix to
keep sensitive data safe. See the
[Sops-Nix Guide](https://saylesss88.github.io/installation/enc/sops-nix.html)
for details.

It's also important to understand that **all files in the `/nix/store` are
world-readable by default** This has important security implications for anyone
managing sensitive data on a NixOS system.

What Does "World-Readable" Mean?

- All files in /nix/store are readable by any user on the system.

- This is by design, the nix store is intended to be shared, immutable store of
  all packages and configuration files.

- Permissions are typically set to `r-xr-xr-x`(read and execute for everyone)

**Security Implications**

- Never store secrets or sensitive data in plane text in the Nix store.

- If you include secrets directly in your configuration, they will end up in the
  `/nix/store` and be accessible to any user or process on the system.

- This applies to files, environment variables, and any data embedded in
  derivations.

**Best Practices**

- Always use a secrets management tool (like `sops-nix` or `agenix`) that
  decrypts secrets at activation time and stores them outside the Nix store,
  with restricted permissions.

- Do not embed secrets directly in Nix expressions or configuration files that
  will be build into the store.

- Even hashed passwords can be vulnerable when stored in a public repository, be
  conscious of what you store where.

- If you’re unsure about what’s safe to share, start with a private repository.
  This gives you time to learn about secrets management and review your
  configuration before making anything public.

First, I'll briefly explain some of the limitations of NixOS Rollbacks and then
I'll go into how Git compliments them.

## Limitations of NixOS Rollbacks

NixOS is famous for its ability to roll back to previous system generations,
either from the boot menu or with commands like `nixos-rebuild --rollback`.

When you perform rollbacks in NixOS, whether from the boot menu or using
commands like `nixos-rebuild --rollback` only the contents and symlinks managed
by the Nix store are affected. The rollback works by switching which system
generation is active, atomically updating symlinks to point to the previous
version of all packages, `systemd` units and services stored in `/nix/store`.

However, it’s important to understand what these rollbacks actually do and what
they don’t do. What NixOS Rollbacks Cover

- System generations: When you rebuild your system, NixOS creates a new
  “generation” that you can boot into or roll back to. This includes all
  packages, services, and system configuration managed by Nix.

- Quick recovery: If an upgrade breaks your system, you can easily select an
  older generation at boot and get back to a working state

**Key Limitations**:

- **Configuration files are not reverted**: Rolling back only changes which
  system generation is active, it does not revert your actual configuration
  files (like `configuration.nix` or your flake files)

- **User data and service data are not rolled back**: Only files managed by Nix
  are affected. Databases, user files, and other persistent data remain
  unchanged, which can cause problems if, for example, a service migrates its
  database schema during an upgrade

- **Manual changes persist**: Any manual edits to configuration files or system
  state outside of Nix are not reverted by a rollback

## How Git Helps

<!-- ![Git Logo 2](../images/git3.png) -->

- The [gh-cli](https://docs.github.com/en/github-cli/github-cli/quickstart),
  simplifies quite a few things for working with GitHub from the command line.

- **Tracks every configuration change**: By version-controlling your NixOS
  configs with Git, you can easily see what changed, when, and why.

- **True config rollback**: If a configuration change causes issues, you can use
  `git checkout` or `git revert` to restore your config files to a previous good
  state, then rebuild your system

- **Safer experimentation**: You can confidently try new settings or upgrades,
  knowing you can roll back both your system state (with NixOS generations) and
  your config files (with Git).

- **Collaboration and backup**: Git lets you share your setup, collaborate with
  others, and restore your configuration if your machine is lost or damaged.

In summary: NixOS rollbacks are powerful for system state, but they don’t manage
your configuration file history. Git fills this gap, giving you full control and
traceability over your NixOS configs making your system both robust and truly
reproducible. Version control is a fundamental tool for anyone working with
NixOS, whether you’re customizing your desktop, managing servers, or sharing
your configuration with others. Git is the most popular version control system
and is used by the NixOS community to track, share, and back up system
configurations.

**Why use Git with NixOS?**

- **Track every change**: Git lets you record every modification to your
  configuration files, so you can always see what changed, when, and why.

- **Experiment safely**: Try new settings or packages without fear—if something
  breaks, you can easily roll back to a previous working state.

- **Sync across machines**: With Git, you can keep your NixOS setups in sync
  between your laptop, desktop, or servers, and collaborate with others.

- **Disaster recovery**: Accidentally delete your config? With Git, you can
  restore it from your repository in minutes.

Installing Git on NixOS

You can install Git by adding it to your system packages in your
configuration.nix or via Home Manager:

## Git Tips

<!-- ![Octocat](../images/octocat.png) -->

If you develop good git practices on your own repositories it will make it
easier to contribute with others as well as get help from others.

## Atomic Commits

**Atomic commits** are a best practice in Git where each commit represents a
single, focused, and complete change to the codebase. The main characteristics
of atomic commits are:

- **One purpose**: Each commit should address only one logical change or task.

- **Complete**: The commit should leave the codebase in a working state.

- **Descriptive**: The commit message should be able to clearly summarize the
  change in a single sentence.

**Why Atomic Commits Matter**

- **Easier debugging**: You can use tools like `git bisect` to quickly find
  which commit introduced a bug, since each commit is isolated.

- **Simpler reverts**: You can revert without affecting unrelated changes.

- **Better collaboration**: Code reviews and merges are more manageable when
  changes are small and focused.

When you lump together a bunch of changes into a single commit it can lead to
quite a few undesirable consequences. They make it harder to track down bugs,
it's more difficult to revert undesired changes without reverting desired ones,
make larger tickets harder to manage.

**Every time a logical component is completed, commit it**. Smaller commits make
it easier for other devs and yourself to understand the changes and roll them
back if necessary. This also makes it easier to share your code with others to
get help when needed and makes merge conflicts less frequent and complex.

**Finish the component, then commit it**: There's really no reason to commit
unfinished work, use `git stash` for unfinished work and `git commit` for when
the logical component is complete. Use common sense and break complex components
into logical chunks that can be finished quickly to allow yourself to commit
more often.

**Write Good Commit Messages**: Begin with a summary of your changes, add a line
of whitespace between the summary and the body of your message. Make it clear
why this change was necessary. Use consistent language with generated messages
from commands like `git merge` which is imperative and present tense
(`<<change>>`, not `<<changed>>` or `<<changes>>`).

### Tips for Keeping Commits Atomic with a Linear History

Squashing limits the benefits of atomic commits as it combines them all into a
single commit as if you didn't take the time to write them all out atomically.

🧠 Why Rebasing Wins for Linear History

- No Merge Bubbles: Rebasing avoids those extra merge commits that clutter
  `git log --graph`. You get a clean, readable timeline.

- Atomic Commit Integrity: Each commit stands alone and tells a story. Rebasing
  preserves that narrative without diluting it with merge noise.

- Better Blame & Bisect: Tools like git blame and git bisect work best when
  history is linear and logical.

- Time-Travel Simplicity: Cherry-picking or reverting is easier when commits
  aren’t tangled in merge commits.

By default, when you run `git pull` git merges the commits into your local repo.
To change this to a rebase you can set the following:

```bash
git config --global pull.rebase true
git config --global rebase.autoStash true
git config --global fetch.prune true  # auto delets remote-tracking branches that no longer exist
git config --global pull.ff only          # blocks merge pulls
```

Note: With pull.ff only pulls will fail if they would have had to merge. This
could happen if your local branch has diverged from the remote (e.g., someone
pushed new commits and you also committed locally) `git pull` will throw an
error like:

```bash
fatal: Not possible to fast-forward, aborting.
```

**How to fix it**

You basically do what Git won't auto-do:

```bash
git fetch origin
git rebase origin/main
```

This rewinds your local commits, applies remote commits, and replays yours on
top, keeping the history linear.

If you don't care about your local changes and want to discard them you can use
the following command:

```bash
git reset --hard origin/main
```

This just makes your branch identical to the remote, no rebase required. This
prevents rogue merge commits, preserving atomic commits and linear logs.

You could set an alias for this with:

```bash
git config --global alias.grs '!git fetch origin && git rebase origin/main'
```

To check whether a setting is active or now you can use:

```bash
git config --get rebase.autoStash
true
```

To set these options with home-manager:

```nix
# ... snip ...
    extraConfig = lib.mkOption {
      type = lib.types.attrs;
      default = {
        commit.gpgsign = true;
        gpg.format = "ssh";
        user.signingkey = "/etc/ssh/ssh_host_ed25519_key.pub";
        extraConfig = {
          pull = {
            rebase = true;
            ff = "only";
        };
        };
        rebase = {
          autoStash = true; # Auto stashes and unstashes local changes during rebase
        };
        fetch = {
          prune = true; # Automatically deletes remote-tracking branches that no longer exist
        };
# ... snip ...
```

## Time Travel in Git

<details>
<summary> ✔️ Click to Expand Time Travel Section </summary>

**View an old commit**:

```bash
git checkout <commit_hash>
```

This puts you in a "detached HEAD" state, letting you explore code as it was at
that commit. To return, checkout your branch again.

**Go back and keep history (revert)**:

```bash
git revert <commit_hash>
```

**Go back and rewrite history (reset)**:

- Soft reset (keep changes staged):

```bash
git reset --soft <commit_hash>
```

- Mixed reset (keep changes in working directory):

```bash
git reset <commit_hash>
```

- Hard reset (discard all changes after the commit):

```bash
git reset --hard <commit_hash>
```

Use the above command with caution, it can delete commits from history.

- Relative time travel:

```bash
git reset --hard HEAD@{5.minutes.ago}
```

or

```bash
git reset --hard HEAD@{yesterday}
```

**Create a branch from the past**:

```bash
git checkout -b <new-brach> <commit_hash>
```

This starts a new branch from any previous commit, preserving current changes.

</details>

Some repositories have guidelines, such as Nixpkgs:

<details>
<summary> ✔️ Click to Expand Nixpkgs Commit Conventions </summary>

**Commit conventions**

- Create a commit for each logical unit.

- Check for unnecessary whitespace with `git diff --check` before committing.

- If you have commits pkg-name: oh, forgot to insert whitespace: squash commits
  in this case. Use `git rebase -i`. See Squashing Commits for additional
  information.

- For consistency, there should not be a period at the end of the commit
  message's summary line (the first line of the commit message).

- When adding yourself as maintainer in the same pull request, make a separate
  commit with the message maintainers: `add <handle>`. Add the commit before
  those making changes to the package or module. See Nixpkgs Maintainers for
  details.

  Make sure you read about any commit conventions specific to the area you're
  touching. See: Commit conventions for changes to `pkgs`. Commit conventions
  for changes to `lib`. Commit conventions for changes to `nixos`. Commit
  conventions for changes to `doc`, the Nixpkgs manual.

**Writing good commit messages**

In addition to writing properly formatted commit messages, it's important to
include relevant information so other developers can later understand why a
change was made. While this information usually can be found by digging code,
mailing list/Discourse archives, pull request discussions or upstream changes,
it may require a lot of work.

Package version upgrades usually allow for simpler commit messages, including
attribute name, old and new version, as well as a reference to the relevant
release notes/changelog. Every once in a while a package upgrade requires more
extensive changes, and that subsequently warrants a more verbose message.

Pull requests should not be squash merged in order to keep complete commit
messages and GPG signatures intact and must not be when the change doesn't make
sense as a single commit.

</details>

A **Git workflow** is a recipe or recommendation for how to use Git to
accomplish work in a consistent and productive manner. Having a defined workflow
lets you leverage Git effectively and consistently. This is especially important
when working on a team.

**Origin** is the _default name_ (alias) for the **remote repository** that your
**local repository** is connected to, usually the one you cloned from.

**Remote Repositories** are versions of your project that are hosted on the
internet or network somewhere.

- When you run `git push origin main`, you're telling Git to push your changes
  to the remote repo called `origin`.

- You can see which URL `origin` points to with `git remote -v`.

- You can have multiple remotes (like `origin`, `upstream`, etc.) each pointing
  to a different remote repo. Each of which is generally either read-only or
  read/write for you. Collaborating involves managing these remotes and pushing
  and pulling data to and from them when you need to share work.

> ❗ You can have a remote repo on your local machine. The word "remote" doesn't
> imply that the repository is somewhere else, only that it's elsewhere.

- The name `origin` is just a convention, it's not special. It is automatically
  set when you clone a repo.

<!-- ![git local remote](../images/git_local-remote.png) -->

**Local** is your local copy of the repository, git tracks the differences
between **local** and **remote** which is a repo hosted elsewhere (e.g., GitHub
GitLab etc.)

The **Upstream** in Git typically refers to the original repository from which
your local repository or fork was derived. The **Upstream** is the remote repo
that serves as the main source of truth, often the original project you forked
from. You typically fetch changes from upstream to update your local repo with
the latest updates from the original project, but you don't push to upstream
unless you have write access.

### A Basic Git Workflow

<!-- ![Git logo 3](../images/git2.png) -->

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

<!-- ![git commit add](../images/git-add-commit.png) -->

2. Make and Track Changes:

Now that you've saved a snapshot of your working configuration, you're free to
experiment and try new things, even if they might break your setup.

Suppose you want to try a new desktop environment, like Xfce. You edit your
`configuration.nix` to add:

```nix
services.xserver.desktopManager.xfce.enable = true;
```

You run:

```bash
sudo nixos-rebuild switch # if configuration.nix is in /etc/nixos/
```

But something goes wrong: the system boots, but your desktop is broken or won't
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

With Git you're always on a branch and the default branch is `master`. Many
change it to `main` because of the suggestion Git gives you. I think people are
too easily offended these days, just keep this in mind that `main` and `master`
refer to the main development branch.

You can get a listing of your current branches with:

```bash
git branch
* (no branch)
  main
```

The `*` is next to the current branch and is where the `HEAD` is currently
pointing. It says `(no branch)` because I'm currently in detached `HEAD` where
`HEAD` points to no branch. The reason for this is because I've been trying out
Jujutsu VCS and that's JJ's default setting, a detached `HEAD`.

Git actually gives you a warning about working in a detached `HEAD`:

```bash
You are in 'detached HEAD' state. You can make experimental
changes and commit them, and you can discard any commits you make
in this state without impacting any branch by switching back.

If you want to create a new branch to retain commits you create,
you can do so now (using 'git switch -c <new-branch-name>') or
later (using 'git branch <new-branch-name> <commit-id>').

See 'git help switch' for details.
```

To attach the `HEAD` (i.e., have the pointer pointing to a branch), use the
`git checkout` command

```bash
git checkout main
Switched to branch 'main'
```

```bash
git branch
* main
# Ensure that you have the latest "tip" from the remote repository `origin`
git fetch origin main
From github.com:sayls8/flake
 * branch            main       -> FETCH_HEAD
```

Although we're working on our own repo and there is basically no chance of our
local branch diverging from our remote, it's still good to get in the practice
of getting everything in sync before merging or rebasing etc.

`git fetch` doesn't update `main`, it just updates your references. To update
`main` you would use `git pull origin/main` or `git rebase origin/main`

You can inspect your upstream branches with the following command:

```bash
git remote show origin
* remote origin
  Fetch URL: git@github.com:saylesss88/flake.git
  Push  URL: git@github.com:saylesss88/flake.git
  HEAD branch: main
  Remote branch:
    main tracked
  Local ref configured for 'git push':
    main pushes to main (fast-forwardable)
```

`* branch     main      -> FETCH_HEAD`: This line signifies that the `main`
branch from the remote repository (likely `origin`) was successfully fetched,
and the commit ID of its current tip (its latest commit) is now stored in your
local `FETCH_HEAD` reference.

Now that we know our local `main` is up to date with our remote `origin/main` we
can safely create a new feature branch:

```bash
git checkout -b feature/prose_wrap
Switched to a new branch 'feature/prose_wrap'
```

Right now the branch `feature/prose_wrap` is exactly the same as `main` and we
can safely make changes without affecting `main`. We can try crazy or even
"dangerous" things and always be able to revert to a working state with
`git checkout main`.

If our crazy idea works out, we can then merge our feature branch into `main`.

Ok the feature works, I've added and committed the change. Now it's time to
point the `HEAD` to `main` and then either merge or rebase the feature branch
into `main`:

```bash
git checkout main
git fetch origin main
git merge feature/prose_wrap
Updating c8bd54c..b281f79
Fast-forward
 home/editors/helix/default.nix | 69 +++++++++++++++++++++++++++++++--------------------------------------
 1 file changed, 31 insertions(+), 38 deletions(-)
```

- "fast-forward" means that our `feature/prose_wrap` branch was directly ahead
  of the last commit on `main`. When you merge one commit with another commit
  that can be reached by following the first commits history, remember the
  feature branch is exactly the same as `main` until I made another commit. If
  the branches diverged more and the history can't be followed, Git will perform
  a 3-way merge where it creates a new "merge commit" that combines the 2
  changes.

If you have a bunch of branches and forget which have been merged yet use:

```bash
git branch --merged
feature/prose_wrap
* main
# OR to see branches that haven't been merged use:
git branch --no-merged
```

It's now safe to delete the feature branch:

```bash
git branch -d feature/prose_wrap
Deleted branch feature/prose_wrap (was b281f79)
```

> ❗ TIP: If your feature branch has a lot of sloppy commits that won't be of
> much benefit to anyone, squash them first then merge. The workflow would look
> something like this:
>
> ```bash
>  # Make sure you're on the main branch
>  git checkout main
>
>  # Merge the feature branch with squash
>  git merge --squash feature/prose_wrap
> ```
>
> - This combines all the commits in your branch and adds them to your `main`
>   staging area, it doesn't move HEAD or create a merge commit for you. To
>   apply the changes into one big commit, finalize it with:
>
> ```bash
>  git commit -m "Add prose wrapping feature"
> ```
>
> This is often referred to as the "squash commit".

Branching means to diverge from the main line of development and continue to do
work without risking messing up your main branch. There are a few commits on
your main branch so to visualize this it would look something like this, image
is from [Pro Git](https://git-scm.com/book/en/v2):

<!-- ![Git Branch 1](../images/git-branch3.png) -->

## Nix flake update example with branches

Let's say you haven't ran `nix flake update` in a while and you don't want to
introduce errors to your working configuration. To do so we can first, make sure
we don't lose any changes on our main branch:

```bash
git add .
git commit -m "Staging changes before switching branches"
# I always like to make sure the configuration will build before pushing to git
sudo nixos-rebuild switch --flake .
# If everything builds and looks correct
git push origin main
```

OR, if you have incomplete changes that you don't want to commit yet you can
stash them with `git stash`:

```bash
git status
On branch main
Your branch is ahead of 'origin/main' by 1 commit.
  (use "git push" to publish your local commits)

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   home/git.nix

no changes added to commit (use "git add" and/or "git commit -a")
```

Now we want to switch branches, without committing the incomplete changes to
`git.nix`:

```bash
git stash
Saved working directory and index state WIP on main: 0e46d6b git: lol alias

git status
On branch main
Your branch is ahead of 'origin/main' by 1 commit.
  (use "git push" to publish your local commits)

nothing to commit, working tree clean
```

> ❗ `git stash` is equivalent to `git stash push`

To see which stashes you have stored, use `git sash list`:

```bash
git stash list
stash@{0}: WIP on main: 0e46d6b git: lol alias
```

To apply the most recent stash:

```bash
git stash apply
git add home/git.nix
On branch main
Your branch is ahead of 'origin/main' by 1 commit.
  (use "git push" to publish your local commits)

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   home/git.nix

# or for multiple stashes
git stash apply stash@{2}
```

Running `git stash apply` applies the changes that were in your stash but
doesn't automatically restage them, to apply the changes and stage them in one
command:

```bash
git stash apply --index
```

Now let's create our branch so we can safely update:

```bash
git checkout -b update-test
Switched to a new branch 'update-test'
```

`-b` is to switch to the branch that was just created

Some may prefer a more descriptive branch name such as: `update/flake-inputs`, I
kept it short for the example. Or if your company uses an issue tracker,
including the ticket number in the branch name can be helpful:
`update/123-flake-inputs`

The above command is equivalent to:

```bash
git branch update-test
git checkout update-test
```

~~Now our branches would look something like this, note how both branches
currently point to the same commit:~~ I discovered that Git Book has pretty
restrictive licensing and will eventually find a replacement.

<!-- ![Git Branch 2](../images/git-branch2.png) -->

Now, lets run our update:

```bash
nix flake update
sudo nixos-rebuild test --flake .
# If everything looks ok let's try applying the changes
sudo nixos-rebuild switch --flake .
# And if everything looks ok:
git add .
git commit -m "feat: Updated all flake inputs"
git push origin update-test
```

> ❗ This is the same workflow for commiting a PR. After you first fork and
> clone the repo you want to work on, you then create a new feature branch and
> push to that branch on your fork. This allows you to create a PR comparing
> your changes to their existing configuration.

~~At this point our graph would look similar to the following~~:

<!-- ![Git Branch 3](../images/git-branch1.png) -->

If we are satisfied, we can switch back to our `main` branch and merge
`update-test` into it:

```bash
git checkout main
git merge origin/update-test
git branch -D update-test
sudo nixos-rebuild test --flake .
sudo nixos-rebuild switch --flake .
```

It's good practice to delete a branch after you've merged and are done with it.

## Rebasing Branches

To combine two seperate branches into one unified history you typically use
`git merge` or `git rebase`.

`git merge` takes two commit pointers and finds a common base commit between
them, it then creates a "merge commit" that combines the changes.

`git rebase` is used to move a sequence of commits to a new base commit.

<!-- ![Git rebase](../images/rebase.png) -->

## Configure Git Declaratively

The following example is the `git.nix` from the hydenix project it shows some
custom options and a way to manage everything from a single location:

```nix
# git.nix from hydenix: declarative Git configuration for Home Manager
{ lib, config, ... }:

let
  cfg = config.hydenix.hm.git;
in
{

  options.hydenix.hm.git = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = config.hydenix.hm.enable;
      description = "Enable git module";
    };

    name = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Git user name";
    };

    email = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Git user email";
    };
  };

  config = lib.mkIf cfg.enable {

    programs.git = {
      enable = true;
      userName = cfg.name;
      userEmail = cfg.email;
      extraConfig = {
        init.defaultBranch = "main";
        pull.rebase = false;
      };
    };
  };
}
```

> ❗ You can easily change the name of the option, everything after `config.` is
> custom. So you could change it to for example, `config.custom.git` and you
> would enable it with `custom.git.enable = true;` in your `home.nix` or
> equivalent.

Then he has a `hm/default.nix` with the following to enable it.

```nix
#...snip...

 # hydenix home-manager options go here
  hydenix.hm = {
    #! Important options
    enable = true;
      git = {
        enable = true; # enable git module
        name = null; # git user name eg "John Doe"
        email = null; # git user email eg "john.doe@example.com"
      };
    }

    # ... snip ...
```

You can enable git, and set your git username as well as git email right here.

### Resources

- [GitCommitBestPractices](https://gist.github.com/luismts/495d982e8c5b1a0ced4a57cf3d93cf60)

- [ProGit](https://git-scm.com/book/en/v2)

- [Oh shit Git](https://ohshitgit.com/)
