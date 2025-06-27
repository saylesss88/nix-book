# Version Control with JJ

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![JJ Logo](../images/jujutsu.png)

- You can use jujutsu (jj) with existing Git repositories with one command.
  `jj git init --colocate` or `jj git init --git-repo /path/to/git_repository`.
  The native repository format for jj is still a work in progress so people
  typically use a `git` repository for backend.

- Unlike `git`, `jj` has no index "staging area". It treats the working copy as
  an actual commit. When you make changes to files, these changes are
  automatically recorded to the working commit. There's no need to explicitly
  stage changes because they are already part of the commit that represents your
  current working state.

  - This means that you don't need to worry about making a change, running
    `git add .`, running `git commit -m "commit message"` because it's already
    done for you. This is handy with flakes by preventing a "dirty working tree"
    and can instantly be rebuilt after making a change.

## Here's an example

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
JJ, there's no concept of a "current branch" -- commits are first-class, and
bookmarks are optional pointers.

**Remote bookmarks** are bookmarks that exist on a remote (like `origin`). jj
keeps track of the last-seen position of each remote bookmark (e.g.,
`main@origin`), similar to Git's remote-tracking branches

> NOTE: JJ is designed for a "branchless" workflow, so bookmarks are more
> lightweight and flexible than Git branches.

**Or** to do this in a directory that isn't already a git repo you can do
something like:

```bash
cargo new hello-world --vcs=none
cd hello-world
jj git init
Initialized repo in "."
```

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
git branch -D update-test
git merge update-test
sudo nixos-rebuild switch --flake .
```

With JJ a similar workflow could be:

```bash
jj new  # Create a new child commit/start working on a new change
nix flake update
sudo nixos-rebuild test --flake .
jj squash #  equivalent to `git commit -a --amend`
jj describe -m "update" # Similar to git commit -m
jj commit # Only needed if finalizing an explicit commit
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
@  lnmmxwko saylesss87@proton.me 2025-06-27 10:14:57 1eac6aa0
│  (empty) (no description set)
○  qnknltto saylesss87@proton.me 2025-06-27 09:04:08 git_head() 5358483a
│  (empty) jj
```

The above log output shows that running `jj git init` creates an empty working
commit (`@`) on top of the `git_head()`

```bash
jj desc -m "Switch from nixVim to NVF"
jj new  # Create a new empty change
jj log
@  nmnmznmm saylesss87@proton.me 2025-06-27 10:16:30 52dd7ee0
│  (empty) (no description set)
○  lnmmxwko saylesss87@proton.me 2025-06-27 10:16:24 git_head() 3e8f9f3a
│  (empty) Switch from nixVim to NVF
○  qnknltto saylesss87@proton.me 2025-06-27 09:04:08 5358483a
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
@  zsxsolsq saylesss87@proton.me 2025-06-27 10:18:01 2c35d83f
│  (empty) (no description set)
○  lnmmxwko saylesss87@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
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
@  zsxsolsq saylesss87@proton.me 2025-06-27 10:18:47 606abaa7
│  (empty) Switch from NVF to nixVim
○  lnmmxwko saylesss87@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
│  (empty) Switch from nixVim to NVF
○  qnknltto saylesss87@proton.me 2025-06-27 09:04:08 5358483a
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
○  zsxsolsq saylesss87@proton.me 2025-06-27 10:22:03 ad0713b6
│  (empty) Switch from NVF to nixVim
@  lpnxxxpo saylesss87@proton.me 2025-06-27 10:22:03 bf929946
│  (empty) Adding LSP to nixVim
○  lnmmxwko saylesss87@proton.me 2025-06-27 10:18:01 git_head() 485eaee9
│  (empty) Switch from nixVim to NVF
○  qnknltto saylesss87@proton.me 2025-06-27 09:04:08 5358483a
│  (empty) jj
○  qnknltto sayls87@proton.me 2025-06-27 09:04:08 git_head()
```

The "Adding LSP to nixVim" commit is directly above "Switch from nixVim to NVF"
(the old `git_head()`)

The "Switch from NVF to nixVim" commit (which was your `@` before `jj new -B`)
is now above "Adding LSP to nixVim" in the log output, meaning "Adding LSP to
nixVim" is its new parent.

`@` has moved to "Adding LSP to nixVim"

## Operation Log and Undo

JJ records every operation (commits, merges, rebases, etc.) in an operation log.
You can view and undo previous operations, making it easy to recover from
mistakes, a feature not present in Git’s core CLI

## Conflict Resolution

In JJ, conflicts live inside commits and can be resolved at any time, not just
during a merge. This makes rebasing and history editing safer and more flexible

## Revsets

JJ includes a powerful query language for selecting commits. For example:

```bash
jj log -r "author(alice) & file(*.py)"
```

This command lists all commits by Alice that touch Python files.

### Resources

- [jj_github](https://github.com/jj-vcs/jj)

- [official_tutorial](https://jj-vcs.github.io/jj/latest/tutorial/)

- [jj_init](https://v5.chriskrycho.com/essays/jj-init/) # very good article

- [steves_jj_tutorial](https://steveklabnik.github.io/jujutsu-tutorial/)
