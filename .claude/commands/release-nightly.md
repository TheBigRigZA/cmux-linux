# Release Nightly

Unattended end-to-end release: bump version, update changelog from raw commit log, build and
validate .deb/.rpm, tag, push to `fork`, and publish a GitHub release. No PR, no signing step.

## Steps

1. **Determine version**
   - Read `Cargo.toml`, bump minor: `./scripts/bump-version.sh`

2. **Gather changes**
   - `git describe --tags --abbrev=0` then `git log --oneline <last-tag>..HEAD --no-merges`
   - Filter to end-user-visible changes; categorize Added/Changed/Fixed/Removed
   - Contributors via `gh pr view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'` (best-effort; skip silently if a PR lookup fails — this is unattended)

3. **Update `CHANGELOG.md`**
   - Add a new section at the top with the new version and today's date
   - **Only include changes that affect the end-user experience**
   - Write clear, user-facing descriptions (not raw commit messages)
   - **Credit contributors inline** (see Contributor Credits below)

4. **Build and validate**
   - `cargo build --release`
   - `./packaging/scripts/build-deb.sh && ./packaging/scripts/validate-deb.sh`
   - `./packaging/scripts/build-rpm.sh && ./packaging/scripts/validate-rpm.sh`
   - Abort the run if either validate script exits non-zero

5. **Commit, tag, push**
   - Stage `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`; commit `Bump version to X.Y.Z`
   - `git push fork main`
   - `git tag vX.Y.Z && git push fork vX.Y.Z`

6. **Publish**
   - `gh release create vX.Y.Z dist/cmux_X.Y.Z_amd64.deb dist/cmux-X.Y.Z-*.rpm --repo TheBigRigZA/cmux-linux --title "vX.Y.Z (nightly)" --notes "<changelog summary>"`

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
