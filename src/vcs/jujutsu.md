# Version Control with JJ

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![JJ Logo](../images/jujutsu.png)

⚠️ **Important**: Never commit secrets (passwords, API keys, tokens, etc.) in
plain text to your Git repository. If you plan to publish your NixOS
configuration, always use a secrets management tool like sops-nix or agenix to
keep sensitive data safe. See the
[Sops-Nix Guide](https://saylesss88.github.io/installation/enc/sops-nix.html)
for details.

You may want to check
[Steve's Jujutsu Tutorial](https://steveklabnik.github.io/jujutsu-tutorial/), he
does a great job of explaining how to use jj with practical examples. It is
recommended in the official JJ docs as a more up to date intro.

After installing JJ you can run `jj help -k tutorial`, for the official guide in
a pager but like I said Steve's guide is more up to date.

You can find help for nearly every command with the command followed by `--help`
or `man jj`, `man jj git init`.

> ❗ Pro Tip: Set the environment variable `export MANPAGER='nvim +Man!'` in
> your shell config to use Neovim as your manpager.

If you haven't taken the time to deep dive Git, it may be a good time to learn
about a new way of doing Version Control that is actually less complex and
easier to mentally map out in my opinion.

Jujutsu is a new front-end to Git, and it's a new design for distributed version
control. --jj init

You can use jujutsu (jj) with existing Git repositories with one command.
`jj git init --colocate` or `jj git init --git-repo /path/to/git_repository`.
The native repository format for jj is still a work in progress so people
typically use a `git` repository for backend.

Unlike `git`, `jj` has no index "staging area". It treats the working copy as an
actual commit. When you make changes to files, these changes are automatically
recorded to the working commit. There's no need to explicitly stage changes
because they are already part of the commit that represents your current working
state.

## What is the Jujutsu Working Copy

![SourceTree image](../images/sourcetree.png)

The **working copy** in Jujutsu is an actual **commit** that represents the
current state of the files you're working on. Unlike Git, where the working copy
is separate from commits and changes must be explicitly staged and committed, in
JJ the working copy is a live commit that automatically records changes as you
modify files.

Adding or removing files in the working copy implicitly tracks or untracks them
without needing explicit commands like `git add`

The working copy commit acts as a snapshot of your current workspace. When you
run commands, Jujutsu first syncs the filesystem changes into this commit, then
performs the requested operation, and finally updates the working copy if needed

To finalize your current changes and start a new set of changes, you use the
`jj new` command, which creates a new working-copy commit on top of the current
one. This replaces the traditional Git workflow of staging and committing
changes separately

Conflicts in the working copy are represented by inserting conflict markers
directly into the files. Jujutsu tracks the conflicting parts and can
reconstruct the conflict state from these markers. You resolve conflicts by
editing these markers and then committing the resolution in the working copy

- This means that you don't need to worry about making a change, running
  `git add .`, running `git commit -m "commit message"` because it's already
  done for you. This is handy with flakes by preventing a "dirty working tree"
  and can instantly be rebuilt after making a change.

## Example JJ Module

- For `lazygit` fans, Nixpkgs has `lazyjj`. I've seen that it's recommended to
  use jj with `meld`. I'll share my `jj.nix` here for an example:

```nix
{
  lib,
  config,
  pkgs,
  ...
}: let
  cfg = config.custom.jj;
in {
  options.custom.jj = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable the Jujutsu (jj) module";
    };

    userName = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = "saylss88";  # you can use `or` statements here also
      description = "Jujutsu user name";
    };

    userEmail = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = "sayls88@proton.me";
      description = "Jujutsu user email";
    };

    configFile = lib.mkOption {
      type = lib.types.lines;
      default = ''
        [ui]
        diff-editor = ["nvim", "-c", "DiffEditor $left $right $output"]
      '';
      description = "Content of the Jujutsu config.toml file";
    };

    packages = lib.mkOption {
      type = lib.types.listOf lib.types.package;
      default = with pkgs; [lazyjj meld];
      description = "Additional Jujutsu-related packages to install";
    };

    settings = lib.mkOption {
      type = lib.types.attrs;
      default = {
        ui = {
          default-command = ["status" "--no-pager"];
          diff-editor = ":builtin";
          merge-editor = ":builtin";
        };
      };
      description = "Jujutsu configuration settings";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = cfg.packages;

    home.file.".jj/config.toml".text = cfg.configFile;

    programs.jujutsu = {
      enable = true;
      settings = lib.mergeAttrs cfg.settings {
        user = {
          name = cfg.userName;
          email = cfg.userEmail;
        };
      };
    };
  };
}
```

To be honest, I have only played around with jj and recently am giving it
another shot. I'm not sure currently if the meld settings are correct FYI. They
are filler names and email addresses also.

In my `home.nix` I have this to enable it:

```nix
custom = {
    jj = {
        enable = true;
        userName = "sayls88";
        userEmail = "sayls88@proton.me";
    };
};
```

The `custom.jj` module allows me to override the username, email, and whether jj
is enabled from a single, centralized place within my Nix configuration. So only
if jj is enabled, `lazyjj` and `meld` will be installed.

## Issues I've Noticed

I have run into a few issues, such as every flake command reloading every single
input every time. **What I mean by this is what you see when you run a flake
command for the first time, it adds all of your flakes inputs.** I believe the
fix for this is deleting and regenerating your `flake.lock`. The same thing can
happen when you move your flake from one location to another.

That said, I recommend doing just that after running something like
`jj git init --colocate`. Delete your `flake.lock` and run `nix flake update`,
`nix flake lock --recreate-lock-file` still works but is being depreciated.

Sometimes the auto staging doesn't pick up the changes in your configuration so
rebuilding changes nothing, this has been more rare but happens occasionally.

## Here's an example of using JJ in an existing Git repo

Say I have my configuration flake in the `~/flakes/` directory that is an
existing Git repository. To use JJ as the front-end I could do something like:

```bash
cd ~/flakes
jj git init --colocate
jj describe -m "first jj commit"
jj commit
```

**Bookmarks** in jj are named pointers to specific revisions, similar to
branches in Git. When you first run `jj git init --git-branch .` in a git repo,
you will likely get a Hint saying "Run the following command to keep local
bookmarks updated on future pulls":

```bash
jj bookmark track main@origin
```

This command tells jj to track the remote bookmark `main@origin` with a local
bookmark named `main`. It is similar to setting an upstream branch in Git. In
JJ, there's no concept of a "current branch" commits are first-class, and
bookmarks are optional pointers.

**Remote bookmarks** are bookmarks that exist on a remote (like `origin`). jj
keeps track of the last-seen position of each remote bookmark (e.g.,
`main@origin`), similar to Git's remote-tracking branches

> NOTE: JJ is designed for a "branchless" workflow, so bookmarks are more
> lightweight and flexible than Git branches.

To push you use `jj git push`, (you must first set the bookmark as we did above)

```bash
jj git push
# or the full command is
jj git push --bookmark main
# example output after pushing my flake repo
Rebased 1 descendant commits onto updated working copy
Changes to push to origin:
  Move forward bookmark main from b48d4e9b361f to 6fb5e4c02617
remote: Resolving deltas: 100% (25/25), completed with 12 local objects.
```

## Create a Repo without an existing Git Repo

**Or** to do this in a directory that isn't already a git repo you can do
something like:

```bash
cargo new hello-world --vcs=none
cd hello-world
jj git init
Initialized repo in "."
```

---

### JJ and Git Side by Side

Or for example, with Git if you wanted to move to a different branch before
running `nix flake update` to see if it introduced errors before merging with
your main branch, you could do something like:

```bash
git checkout -b update-test

nix flake update

sudo nixos-rebuild test --flake .
```

If you're satisfied you can merge:

```bash
git checkout main
git add . # Stage the change
git commit -m "update"
git merge update-test
git branch -D update-test
sudo nixos-rebuild switch --flake .
```

With JJ a similar workflow could be:

```bash
jj new  # Create a new child commit/start working on a new change
nix flake update
sudo nixos-rebuild test --flake .
jj squash #  similar to `git commit -a --amend`
jj describe -m "update" # Similar to git commit -m
sudo nixos-rebuild switch --flake .
```

- With `jj` you're creating a new commit rather than a new branch.

- Amending vs. Squashing: Git's `git commit --amend` updates the last commit.
  `jj squash` combines the current commit with its parent, effectively doing the
  same thing in terms of history.

- Merging: Git's merge command is explicit. In `jj`, the concept is similar, but
  since there's no branch, you're "merging" by moving your working commit to
  include these changes. The `jj squash` here acts like merging the changes into
  the main line of development.

- No need to delete branches: Since there are no branches in `jj`, there's no
  equivalent to `git branch -D` to clean up. Instead commits that are no longer
  needed can be "abandoned" with `jj abandon` if you want to clean up your
  commit graph.

- `jj describe` without a flag just opens `$EDITOR` where you can write your
  commit message save and exit.

- In `git`, we finish a set of changes to our code by committing, but in `jj` we
  start new work by creating a change, and _then_ make changes to our code. It's
  more useful to write an initial description of your intended changes, and then
  refine it as you work, than it is creating a commit message after the fact.

- I have heard that jj can struggle with big repositories such as Nixpkgs and
  have noticed some issues here and there when using with NixOS. I'm hoping that
  as the project matures, it gets better on this front.

---

## The 2 main JJ Workflows

### The Squash Workflow

This workflow is the most similar to Git and Git's index.

The workflow:

1. Describe the work we want to do with `jj desc -m "message"`

2. We create a new empty change on top of that one with `jj new`

3. When we are done with a feature, we run `jj squash` to move the changes from
   `@` into the change we described in step 1. `@` is where your working copy is
   positioned currently.

For example, let's say we just ran `jj git init --colocate` in our configuration
Flake directory making it a `jj` repo as well using git for backend.

```bash
cd flake
jj git init --colocate
jj log
@  lnmmxwko sayls8@proton.me 2025-06-27 10:14:57 1eac6aa0
│  (empty) (no description set)
○  qnknltto sayls8@proton.me 2025-06-27 09:04:08 git_head() 5358483a
│  (empty) jj
```

The above log output shows that running `jj git init` creates an empty working
commit (`@`) on top of the `git_head()`

```bash
jj desc -m "Switch from nixVim to NVF"
jj new  # Create a new empty change
jj log
@  nmnmznmm sayls8@proton.me 2025-06-27 10:16:30 52dd7ee0
│  (empty) (no description set)
○  lnmmxwko sayls8@proton.me 2025-06-27 10:16:24 git_head() 3e8f9f3a
│  (empty) Switch from nixVim to NVF
○  qnknltto sayls8@proton.me 2025-06-27 09:04:08 5358483a
│  (empty) jj
```

The above log shows that running `jj desc` changes the current (`@`) commits
description, and then `jj new` creates a new empty commit on top of it, moving
(`@`) to this new empty commit.

The "Switch from nixVim to NVF" commit is now the parent of (`@`).

Now, we'd make the necessary changes and to add them to the commit we just
described in the previous steps.

The changes are automatically "staged" so theres no need to `git add` them, so
we just make the changes and squash them.

```bash
jj squash  # Squash the commit into its parent commit (i.e., our named commit)
jj log
@  zsxsolsq sayls8@proton.me 2025-06-27 10:18:01 2c35d83f
│  (empty) (no description set)
○  lnmmxwko sayls8@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
│  (empty) Switch from nixVim to NVF
```

This shows `jj squashes` effect, it merges the changes from the current (`@`)
commit into its parent. The (`@`) then moves to this modified parent, and a new
empty commit is created on top, ready for the next set of changes.

```bash
sudo nixos-rebuild switch --flake .
```

We're still in the nameless commit and can either continue working or run
`jj desc -m ""` again describing our new change, then `jj new` and `jj squash`
it's pretty simple. The nameless commit is used as an adhoc staging area.

---

### The Edit Workflow

This workflow adds a few new commands `jj edit`, and `jj next`.

Heres the workflow:

1. Create a new change to work on the new feature with `jj new`

2. If everything works exactly as planned, we're done.

3. If we realize we want to break this big change up into multiple smaller ones,
   we do it by making a new change before the current one, swapping to it, and
   making the necessary change.

4. Lastly, we go back to the main change.

The squash workflow leaves `@` at an empty undescribed change, with this
workflow, `@` will often be on the existing change.

If `@` wasn't at an empty change, we would start this workflow with:

```bash
jj new -m "Switch from NVF to nixVim"
```

since our `@` is already at an empty change, we'll just describe it and get
started:

For this example, lets say we want to revert back to nixVim:

```bash
jj desc -m "Switch from NVF to nixVim"
jj log
@  zsxsolsq sayls8@proton.me 2025-06-27 10:18:47 606abaa7
│  (empty) Switch from NVF to nixVim
○  lnmmxwko sayls8@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
│  (empty) Switch from nixVim to NVF
○  qnknltto sayls8@proton.me 2025-06-27 09:04:08 5358483a
│  (empty) jj
```

Again, this shows `jj desc` renaming the current empty `@` commit.

We make the changes, and it's pretty straightforward so we're done, every change
is automatically staged so we can just run `sudo nixos-rebuild switch --flake .`
now to apply the changes.

If we wanted to make more changes that aren't described we can use `jj new -B`
which is similar to `git add -a`.

```bash
jj new -B @ -m "Adding LSP to nixVim"
Rebased 1 descendant commits
Working copy  (@) now at: lpnxxxpo bf929946 (empty) Adding LSP to nixVim
Parent commit (@-)      : lnmmxwko 485eaee9 (empty) Switch from nixVim to NVF
```

The `-B` tells jj to create the new change _before_ the current one and it
creates a rebase. We created a change before the one we're on, it automatically
rebased our original change. This operation will _always_ succeed with jj, we
will have our working copy at the commit we've just inserted.

You can see below that `@` moved down one commit:

```bash
jj log
○  zsxsolsq sayls8@proton.me 2025-06-27 10:22:03 ad0713b6
│  (empty) Switch from NVF to nixVim
@  lpnxxxpo sayls8@proton.me 2025-06-27 10:22:03 bf929946
│  (empty) Adding LSP to nixVim
○  lnmmxwko sayls8@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
│  (empty) Switch from nixVim to NVF
○  qnknltto sayls8@proton.me 2025-06-27 09:04:08 5358483a
│  (empty) jj
○  qnknltto sayls8@proton.me 2025-06-27 09:04:08 git_head()
```

The "Adding LSP to nixVim" commit is directly above "Switch from nixVim to NVF"
(the old `git_head()`)

The "Switch from NVF to nixVim" commit (which was your `@` before `jj new -B`)
is now above "Adding LSP to nixVim" in the log output, meaning "Adding LSP to
nixVim" is its new parent.

`@` has moved to "Adding LSP to nixVim"

`jj log` example output

---

## Operation Log and Undo

JJ records every operation (commits, merges, rebases, etc.) in an operation log.
You can view and undo previous operations, making it easy to recover from
mistakes, a feature not present in Git’s core CLI

```bash
jj op log
@  fbf6e626df22 jr@magic 15 minutes ago, lasted 9 milliseconds
│  new empty commit
│  args: jj new -B @ -m 'Adding LSP to nixVim'
○  bde40b7c17cf jr@magic 19 minutes ago, lasted 8 milliseconds
│  describe commit 2c35d83f75031dc582bf28b64d4af1c218177f90
│  args: jj desc -m 'Switch from NVF to nixVim'
○  3a2bfe1c0b0a jr@magic 19 minutes ago, lasted 8 milliseconds
│  squash commits into 3e8f9f3a6a58fef86906e16e9b4375afb43e73e3
│  args: jj squash
○  80abcb58dcb6 jr@magic 21 minutes ago, lasted 8 milliseconds
│  new empty commit
│  args: jj new
○  8c80314cbcd7 jr@magic 21 minutes ago, lasted 8 milliseconds
│  describe commit 1eac6aa0b88ba014785ee9c1c2ad6e2abc6206e9
│  args: jj desc -m 'Switch from nixVim to NVF'
○  44b5789cb4d1 jr@magic 22 minutes ago, lasted 6 milliseconds
│  track remote bookmark main@origin
│  args: jj bookmark track main@origin
○  dbefee04aa85 jr@magic 23 minutes ago, lasted 4 milliseconds
│  import git head
│  args: jj git init --git-repo .
```

```bash
jj op undo <operation-id>
# or
jj op restore <operation-id>
```

---

## Conflict Resolution

In JJ, conflicts live inside commits and can be resolved at any time, not just
during a merge. This makes rebasing and history editing safer and more flexible

JJ treats conflicts as first-class citizens: conflicts can exist inside commits,
not just in the working directory. This means if a merge or rebase introduces a
conflict, the conflicted state is saved in the commit itself, and you can
resolve it at any time there’s no need to resolve conflicts immediately or use
“`--continue`” commands as in Git

Here's how it works:

When you check out or create a commit with conflicts, JJ materializes the
conflicts as markers in your files (similar to Git's conflict markers)

You can resolve conflicts by editing the files to remove the markers, or by
using:

```bash
jj resolve
```

---

## Revsets

[Jujutsu Revsets](https://jj-vcs.github.io/jj/latest/revsets/)

JJ includes a powerful query language for selecting commits. For example:

```bash
jj log -r "author(alice) & file(*.py)"
```

This command lists all commits by Alice that touch Python files.

## Filesets

[Jujutsu Filesets](https://jj-vcs.github.io/jj/latest/filesets/)

Jujutsu supports a functional language for selecting a set of files. Expressions
in this language are called "filesets" (the idea comes from Mercurial). The
language consists of file patterns, operators, and functions. --JJ Docs

---

### Resources

- [steves_jj_tutorial](https://steveklabnik.github.io/jujutsu-tutorial/)

- [jj_github](https://github.com/jj-vcs/jj)

- [official_tutorial](https://jj-vcs.github.io/jj/latest/tutorial/)

- [jj_init](https://v5.chriskrycho.com/essays/jj-init/)
