# Release Local

Bump version, update changelog, build and validate the .deb/.rpm locally, tag, and push to
`fork`. Does not publish a GitHub release (use `/release` for that).

## Steps

### 1. Determine the new version number

- Get the current version from `Cargo.toml`
- Bump the minor version unless the user specifies otherwise

### 2. Gather changes and contributors since the last release

- Find the most recent git tag: `git describe --tags --abbrev=0`
- Get commits since that tag: `git log --oneline <last-tag>..HEAD --no-merges`
- **Filter for end-user visible changes only** — ignore developer tooling, CI, docs, tests
- Categorize changes into: Added, Changed, Fixed, Removed
- If there are no user-facing changes, ask the user if they still want to release
- **Collect contributors:** For each PR referenced in the commits, get the author:
  ```bash
  gh pr view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'
  ```
- Also check for linked issue reporters (the person who filed the bug):
  ```bash
  gh issue view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'
  ```
- Build a deduplicated list of all contributor `@handle`s for the release

### 3. Update the changelog

- Add a new section at the top of `CHANGELOG.md` with the new version and today's date
- **Only include changes that affect the end-user experience**
- Write clear, user-facing descriptions (not raw commit messages)
- **Credit contributors inline** (see Contributor Credits below)

### 4. Bump the version

- Run: `./scripts/bump-version.sh` (bumps minor by default; pass `patch`/`major`/`X.Y.Z` to override)

### 5. Build and validate packages

- `cargo build --release`
- `./packaging/scripts/build-deb.sh && ./packaging/scripts/validate-deb.sh`
- `./packaging/scripts/build-rpm.sh && ./packaging/scripts/validate-rpm.sh`

### 6. Commit, tag, and push to `fork`

- Stage: `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`
- Commit message: `Bump version to X.Y.Z`
- `git push fork main`
- `git tag vX.Y.Z && git push fork vX.Y.Z`

### 7. Report

- Print the paths of the built `dist/cmux_*.deb` and `dist/cmux-*.rpm` for the user to
  inspect/install locally

## Changelog Guidelines

**Include only end-user visible changes:**
- New features users can see or interact with
- Bug fixes users would notice (crashes, UI glitches, incorrect behavior)
- Performance improvements users would feel
- UI/UX changes
- Breaking changes or removed features

**Exclude internal/developer changes:**
- Setup scripts, build scripts, reload scripts
- CI/workflow changes
- Documentation updates (README, CONTRIBUTING, CLAUDE.md)
- Test additions or fixes
- Internal refactoring with no user-visible effect
- Dependency updates (unless they fix a user-facing bug)

**Writing style:**
- Use present tense ("Add feature" not "Added feature")
- Group by category: Added, Changed, Fixed, Removed
- Be concise but descriptive
- Focus on what the user experiences, not how it was implemented

## Contributor Credits

Credit the people who made each release happen. This builds community and encourages contributions.

**Per-entry attribution** — append contributor credit after each changelog bullet:
- For code contributions (PR author): `— thanks @user!`
- For bug reports (issue reporter, if different from PR author): `— thanks @reporter for the report!`
- Core team (`lawrencecchen`, `austinywang`) contributions get no per-entry callout — core work is the baseline

**Summary section** — add a "Thanks to N contributors!" section at the bottom of each release:
```markdown
### Thanks to N contributors!

- [@user1](https://github.com/user1)
- [@user2](https://github.com/user2)
```
- List all contributors alphabetically by GitHub handle (including core team)
- Link each handle to their GitHub profile
- Include everyone: PR authors, issue reporters, anyone whose work is in the release

**GitHub Release body** — when the release is published, the GitHub Release should also include the "Thanks to N contributors!" section with linked handles.
