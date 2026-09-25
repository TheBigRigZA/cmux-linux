# Release

Prepare a new release for cmux-linux. This command updates the changelog, bumps the version,
builds the .deb/.rpm, validates them, tags, and publishes a GitHub release.

## Steps

1. **Determine the new version number**
   - Get the current version from `Cargo.toml` (`version = "X.Y.Z"`)
   - Bump the minor version unless the user specifies otherwise: `./scripts/bump-version.sh`
     (or `patch`/`major`/an explicit `X.Y.Z`)

2. **Gather changes and contributors since the last release**
   - Find the most recent git tag: `git describe --tags --abbrev=0`
   - Get commits since that tag: `git log --oneline <last-tag>..HEAD --no-merges`
   - **Filter for end-user visible changes only** - ignore developer tooling, CI, docs, tests
   - Categorize changes into: Added, Changed, Fixed, Removed
   - **Collect contributors:** For each PR referenced in the commits, get the author:
     ```bash
     gh pr view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'
     ```
   - Also check for linked issue reporters (the person who filed the bug):
     ```bash
     gh issue view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'
     ```
   - Build a deduplicated list of all contributor `@handle`s for the release

3. **Update `CHANGELOG.md`**
   - Add a new section at the top with the new version and today's date
   - **Only include changes that affect the end-user experience** - things users will see, feel, or interact with
   - Write clear, user-facing descriptions (not raw commit messages)
   - **Credit contributors inline** (see Contributor Credits below)
   - If there are no user-facing changes, ask the user if they still want to release

4. **Bump the version**
   - Run `./scripts/bump-version.sh` (bumps `Cargo.toml`; pass `patch`/`major`/`X.Y.Z` to
     override the default minor bump)

5. **Build the release artifacts**
   - `cargo build --release`
   - `./packaging/scripts/build-deb.sh && ./packaging/scripts/validate-deb.sh`
   - `./packaging/scripts/build-rpm.sh && ./packaging/scripts/validate-rpm.sh`

6. **Commit, tag, and push to `fork`**
   - Stage: `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`
   - Commit message: `Bump version to X.Y.Z`
   - `git push fork main`
   - `git tag vX.Y.Z`
   - `git push fork vX.Y.Z`

7. **Publish the GitHub release**
   - `gh release create vX.Y.Z dist/cmux_X.Y.Z_amd64.deb dist/cmux-X.Y.Z-*.rpm --repo TheBigRigZA/cmux-linux --title "vX.Y.Z" --notes "<changelog summary>"`

8. **Verify**
   - `gh release view vX.Y.Z --repo TheBigRigZA/cmux-linux` and confirm both `.deb` and `.rpm`
     assets are attached

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
- Link to issues/PRs if relevant

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

## Example Changelog Entry

```markdown
## [0.13.0] - 2025-01-30

### Added
- New keyboard shortcut for quick tab switching ([#42](https://github.com/TheBigRigZA/cmux-linux/pull/42)) — thanks @contributor!

### Fixed
- Memory leak when closing split panes ([#38](https://github.com/TheBigRigZA/cmux-linux/pull/38)) — thanks @fixer!
- Notification badges not clearing properly ([#35](https://github.com/TheBigRigZA/cmux-linux/pull/35)) — thanks @reporter for the report!

### Changed
- Improved terminal rendering performance ([#40](https://github.com/TheBigRigZA/cmux-linux/pull/40))

### Thanks to 4 contributors!

- [@contributor](https://github.com/contributor)
- [@fixer](https://github.com/fixer)
- [@lawrencechen](https://github.com/lawrencechen)
- [@reporter](https://github.com/reporter)
```
