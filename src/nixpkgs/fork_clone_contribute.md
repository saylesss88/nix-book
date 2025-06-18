# Fork, Clone, Contribute

- In the [Nixpkgs](https://github.com/NixOS/nixpkgs) Repository.

- Click Fork, then Create a new Fork.

- Uncheck the box "Only fork the `master` branch", for development we will need
  more branches for development.

  - If you only fork master, you won't have the `nixos-XX.YY` release branches
    available on your fork when you later try to create a PR against them, or
    when you want to create a feature branch from them on your fork.

- Click `<> Code` and Clone the Repo. `saylesss88` is the name of my GitHub,
  yours will obviously be different.

```bash
git clone git@github.com:saylesss88/nixpkgs.git
```

Figure out the branch that should be used for this change by going through
[this section](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md#branch-conventions)

When in doubt use `master`, that's where most changes should go. This can be
changed later by
[rebasing](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md#rebasing-between-branches-ie-from-master-to-staging)

```bash
cd nixpkgs

git remote add upstream https://github.com/NixOS/nixpkgs.git
# Make sure you have the latest changes from upstream Nixpkgs
git fetch upstream
# Show currently configured remote repository
git remote -v
origin  git@github.com:saylesss88/nixpkgs.git (fetch)
origin  git@github.com:saylesss88/nixpkgs.git (push)
upstream        https://github.com/NixOS/nixpkgs.git (fetch)
upstream        https://github.com/NixOS/nixpkgs.git (push)
```

**Understanding Your Remotes**

This output confirms that:

- `origin` is your personal fork on GitHub (`saylesss88/nixpkgs.git`). When you
  `git push origin ...`, your changes go here.

- `upstream` is the official Nixpkgs repository (`NixOS/nixpkgs.git`). When you
  `git fetch upstream`, you're getting the latest updates from the main project.

This setup ensures you can easily pull updates from the original project and
push your contributions to your own fork.

```bash
# Shows a ton of branches
git branch -r | grep upstream
# Narrow it down
git branch -r | grep upstream | grep nixos-
```

Next Steps for Contributing

1. Ensure `master` is up to date with `upstream`

```bash
git checkout master
git fetch upstream
git pull upstream master
git push origin master
```

This keeps your fork in sync to avoid conflicts.

If targeting another branch, replace `master` with `nixos-24.11` for example.

2. Create a Feature Branch

```bash
git checkout master
git checkout -b my-feature-branch # name should represent the feature
```

3. Make and Test Changes

[Packaging Conventions](https://github.com/NixOS/nixpkgs/blob/master/pkgs/README.md#conventions)

New package: Add to
`pkgs/by-name/<first-two-letters>/<package-name>/default.nix`.

Example structure:

```nix
{ lib, stdenv, fetchFromGitHub }: stdenv.mkDerivation {
pname = "xyz"; version = "1.2.3"; src = fetchFromGitHub { ... }; ... }
```

Update package: Edit version and `sha256` in the package’s `default.nix`. Use
`nix-prefetch-url` to update hashes:

```bash
nix-prefetch-url <source-url>
```

Fix a bug: Modify files in `pkgs/`, `nixos/modules/`, or elsewhere.

Test locally:

Build:

```bash
nix-build -A <package-name>
```

Test in a shell:

```bash
 bash nix-shell -p <package-name>
```

For NixOS modules:

```bash
nixos-rebuild test
```

Follow the Nixpkgs Contributing Guide.

4. Commit and Push

Commit with a clear message, make sure to follow
[commit conventions](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md#commit-conventions):

**Commit Conventions**

- Create a commit for each logical unit.

- Check for unnecessary whitespace with `git diff --check` before committing.

- If you have commits `pkg-name: oh, forgot to insert whitespace`: squash
  commits in this case. Use `git rebase -i`. See
  [Squashing Commits](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History#_squashing)
  for additional information.

- For consistency, there should not be a period at the end of the commit
  message's summary line (the first line of the commit message).

- When adding yourself as maintainer in the same pull request, make a separate
  commit with the message maintainers: add <handle>. Add the commit before those
  making changes to the package or module. See
  [Nixpkgs Maintainers](https://github.com/NixOS/nixpkgs/blob/master/maintainers/README.md)
  for details.

Format the commit messages in the following way:

```bash
(pkg-name): (from -> to | init at version | refactor | etc)

(Motivation for change. Link to release notes. Additional information.)
```

a) For example, for the `airshipper` package:

```bash
git add pkgs/by-name/ai/airshipper/
git commit -m "airshipper: init at 0.1.0"

Adds the airshipper tool for managing game assets.
Upstream homepage: https://github.com/someuser/airshipper"
```

b) Updating `airshipper` to a new version

```bash
git add pkgs/by-name/ai/airshipper/
git commit -m "airshipper: 0.1.0 -> 0.2.0

Updated airshipper to version 0.2.0. This release includes:
- Improved asset fetching logic
- Bug fixes for network errors

Release notes: https://github.com/someuser/airshipper/releases/tag/v0.2.0"
```

c) Fixing a bug in `airshipper`'s package definition

```bash
git add pkgs/by-name/ai/airshipper/
git commit -m "airshipper: fix: build with latest glibc

Resolved build failures on unstable channel due to changes in glibc.
Patched source to use updated API calls.
"
```

Examples:

- `nginx: init at 2.0.1`

- `firefox: 122.0 -> 123.0`

- `vim: fix build with gcc13`

Push:

```bash
git push origin my-feature-branch
```

5. Create a Pull Request

Go to <https://github.com/saylesss88/nixpkgs>. (your fork) Click the PR prompt
for my-feature-branch. Set the base branch to `NixOS/nixpkgs:master` (or
`nixos-24.11`). Write a PR description: Purpose of the change. Related issues
(e.g., Fixes #1234). Testing steps (e.g., `nix-build -A <package-name>`). Submit
and respond to feedback.

6. Handle Updates

For reviewer feedback or upstream changes:

Edit, commit, and push:

```bash
git add . git commit -m "<package-name>: address feedback" git push origin my-feature-branch
```

Rebase if needed: bash

```bash
git fetch upstream
git rebase upstream/master  # or upstream/nixos-24.11
git push origin my-feature-branch --force
```

7. Cleanup

After PR merge:

Delete branch:

```bash
git push origin --delete my-feature-branch
```

Sync master:

```bash
git checkout master
git pull upstream master
git push origin master
```

Addressing the Many Branches

- No need to manage all branches: The `nixos-branches` are just metadata from
  upstream. You only check out the one you need (e.g., `master` or
  `nixos-24.11`).

- Focus on relevant branches: The filter (`grep nixos-`) shows the key release
  branches. Ignore -small branches and older releases unless specifically
  required. Confirm latest stable: If you’re targeting a stable branch,
  `nixos-24.11` is likely the latest (or `nixos-25.05` if it’s active). Verify
  via NixOS status.
