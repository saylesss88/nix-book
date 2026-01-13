---
title: Packaging a Rust crate for Nixpkgs
date: 2025-12-31
author: saylesss88
collection: "blog"
tags: ["packaging", "rust", "nixpkgs"]
draft: false
---

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

# Packaging a Rust crate for Nixpkgs

> NOTE: This example assumes you’re packaging a crate that’s already on
> crates.io, or you’re packaging an existing Rust project for nixpkgs.

Nixpkgs is a big repository, so it helps to start with a focused workflow:
create a branch, add a package under `pkgs/by-name/`, build it, then open a PR.

---

## Clone nixpkgs

1. Fork and clone `NixOS/nixpkgs`:

```bash
git clone git@github.com:your-user/nixpkgs.git
cd nixpkgs
git remote add upstream git@github.com:NixOS/nixpkgs.git
```

(SSH avoids HTTPS helper issues)

2. If your clone is shallow, convert it to full history (doesn't lose work):

```bash
git fetch --unshallow --tags
```

---

## Create a branch and add package:

1. Create a branch before changes preferably:

```bash
git switch -c mdbook-rss-feed
```

---

## Add the package under pkgs/by-name

New top-level packages should generally go under
`pkgs/by-name/<2 letters>/<name>/package.nix` (e.g.
`pkgs/by-name/md/mdbook-rss-feed/package.nix`). Packages in `pkgs/by-name` are
picked up automatically and usually don’t require edits to `all-packages.nix`.

---

## Write package.nix (Rust crate example)

Start with `rustPlatform.buildRustPackage` and `fetchCrate`:

```nix
{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
}:
rustPlatform.buildRustPackage rec {
  pname = "mdbook-rss-feed";
  version = "1.3.0";

  src = fetchCrate {
    inherit pname version;
    hash = "output of `nix hash convert` shown below";
  };

  cargoHash = lib.fakeHash;

  nativeInstallCheckInuts = [
    versionCheckHook
  ];
  doInstallCheck = true;

  meta = {
    description = "mdBook preprocessor that generates RSS, Atom, and JSON feeds";
    mainProgram = "mdbook-rss-feed";
    homePage = "https://crates.io/crates/mdbook-rss-feed";
    license = lib.licenses.asl20;
    maintainers = [ lib.maintainers.sayls88 ];
  };
}
```

---

## Prefetch the crate hash:

Use `fetchCrate` / `crate2nix` style workflow, or just prefetch the `crates.io`
tarball:

```bash
nix-prefetch-url \
  --unpack \
  https://crates.io/api/v1/crates/mdbook-rss-feed/1.3.0/download
```

That prints a base32 hash: `0932843lknasdlfkm2lkdnflaknldvdsvser`

Convert it to sri format:

```bash
nix hash convert --hash-algo sha256 --from nix32 --to sri 0932843lknasdlfkm2lkdnflaknldvdsvser
```

The above commands output looks like: `sha256-...=`

Put the resulting `sha256-...` into `src.hash`:

```nix
  src = fetchCrate {
    inherit pname version;
    hash = "sha256-...";
  };
```

---

## Get cargoHash via a failing build

In the `nixpkgs` root (i.e., the `nixpkgs` directory), run:

```bash
nix-build -A mdbook-rss-feed
# OR nix3 format
nix build .#mdbook-rss-feed
```

Nix will fail with a message like:

```text
hash mismatch
specified: sha256-....
got: sha256-1...
```

Copy the `got` value into `cargoHash`, rebuild, and it should succeed.

Sanity check: from `nixpkgs` root :

```bash
./result/bin/mdbook-rss-feed --version
```

---

## Adding yourself as maintainer

Edit `nixpkgs/maintainers/maintainer-list.nix` add your user in alphabetical
order:

```nix
your-handle = {
  email = "you@example.com";
  name = "Your Name";
  github = "your-gh-handle";
  githubId = 12345678;
};
```

If you specify `github`, nixpkgs expects `githubId` too. You can get it from:
`https://api.github.com/users/<user>`.

The nixpkgs maintainers prefer if you add the `maintainer-list.nix` as a
separate commit.

```bash
git commit -m "maintainers: add <user>"
```

---

## Treefmt

Run treefmt the nixpkgs way, from the repo root. Run this right before you push,
my editors formatter does something different with single entry lists than what
Nixpkgs wants:

```bash
nix develop --command treefmt
nix fmt
```

---

## Rebase and push safely

From the `mdbook-rss-feed` branch:

```bash
git fetch upstream --tags
git rebase upstream/master
```

Commit and push your PR branch:

**Then commit and push**

```bash
git commit -m "mdbook-rss-feed: init at 1.3.0"
# First push
git push origin mdbook-rss-feed
# Use `--force-with-lease` only if you rebased/amended and need to rewrite the PR branch.
# git push --force-with-lease origin mdbook-rss-feed
```

`--force-with-lease` is the recommended safe force-push for PR branches.

If `--force-with-lease` says "stale info", fetch the remote branch ref first,
then retry.

**Then commit and push**

```bash
git commit -m "mdbook-rss-feed: init at 1.3.0"
git push -u origin mdbook-rss-feed
```

Then:

1. Go to GitHub -> your fork

2. Click "Compare & pull request" on the `mdbook-rss-feed` branch

3. Fill out the PR template (why useful, tested on x86_64-linux, etc.)

4. Submit!

The package will go through CI checks, and once green + approved by a
maintainer, it'll land in nixpkgs.

---

## Recovering from Mistakes

You're bound to make mistakes, if you learn some Git basics it will help you
quite a bit.

You should avoid adding new commits for small fixes like typos, formatting, or
minor adjustments requested in review. For substantial changes that add
functionality, a new commit may be more appropriate.

Say that we pushed our PR and one of the maintainers gave us a suggested change,
(they want us to follow conventions and remove a trailing period from our
packages description for this example).

1. Make the edit locally (remove the trailing period in the file)

2. Stage the change: `git add pkgs/by-name/xx/your-package/package.nix` (avoid
   `git add -A` as it stages everything, which can accidentally include
   unrelated files)

3. Amend the commit: `git commit --amend --no-edit`(this preserves your original
   commit message, if you want to change the commit message use
   `git commit --amend`)

4. Force push: `git push --force-with-lease` (This is safer than just using
   `--force` because it will fail if someone else has pushed commits to your
   branch that you don't have locally)

**Alternative for Multiple Commits**

Interactive rebase is useful when your PR has several “WIP” commits (or you
added a small review fix as a separate commit) and you want to present a cleaner
history before merge.

You can use interactive rebase to squash all your would be small fix commits
into a single commit they belong to.

Avoid squashing if the commits represent distinct, reviewable changes that stand
on their own.

**Basic Workflow (squash/fixup)**

1. Decide how many commits back you want to edit (example: last 3 commits):

```bash
git rebase -i HEAD~3
```

2. Your editor opens with a "todo" list (oldest at top). Change later commits
   from `pick` to `fixup` or `squash`:

- `fixup` = combine into the previous commit, discard this commit message.

- `squash` = combine, but keep/edit commit messages.

Example todo:

```text
pick 1111111 mdbook-rss-feed: init at 0.1.0
pick 2222222 mdbook-rss-feed: fix trailing period
pick 3333333 mdbook-rss-feed: formatting
```

Change to:

```text
pick 1111111 mdbook-rss-feed: init at 0.1.0
fixup 2222222 mdbook-rss-feed: fix trailing period
fixup 3333333 mdbook-rss-feed: formatting
```

3. Save/close: if you squashed, Git will prompt you to edit the final combined
   message.

**Push updated history**

You have to force-push because `rebase` rewrites commit SHAs:

```bash
git push --force-with-lease
# If something goes wrong
# git rebase --abort
# If you hit conflicts, fix the files, then:
# git add <files>
# git rebase --continue
```

If you have an unrelated change accidentally included (for example: you staged
an extra file), it’s usually better to fix it via rebase/splitting before
reviewers spend time re-reviewing noise.

> As of 01-13-26 I have been waiting for 2 weeks for the darwin checks to complete,
I guess this [bottleneck](https://discourse.nixos.org/t/ofborg-aarch64-darwin-builds-causing-bottleneck/55290)
has gotten worse. I guess most PRs take about 6 weeks to resolve FYI.
