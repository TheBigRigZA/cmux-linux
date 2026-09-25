---
name: release
description: "Prepare and ship a cmux release end-to-end: choose the next version, curate user-facing changelog entries, bump versions, build and validate .deb/.rpm packages, tag, push to fork, and publish a GitHub release. Use when asked to cut, prepare, publish, or tag a new release."
---

# Release

Run this workflow to prepare and publish a cmux-linux release.

## Workflow

1. Determine the version:
- Read `version` from `Cargo.toml`.
- Default to a minor bump unless the user explicitly requests patch/major/specific version.

2. Gather user-facing changes and contributors since the last tag:
- `git describe --tags --abbrev=0`
- `git log --oneline <last-tag>..HEAD --no-merges`
- Keep only end-user visible changes (features, bug fixes, UX/perf behavior).
- **Collect contributors:** For each PR, get the author with `gh pr view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'`. Also check linked issue reporters with `gh issue view <N> --repo TheBigRigZA/cmux-linux --json author --jq '.author.login'`.
- Build a deduplicated list of all contributor `@handle`s.

3. Update `CHANGELOG.md`:
- Use categories `Added`, `Changed`, `Fixed`, `Removed`.
- **Credit contributors inline** (see Contributor Credits below).
- If no user-facing changes exist, confirm with the user before continuing.

4. Bump version and build packages:
- `./scripts/bump-version.sh` (minor) or `./scripts/bump-version.sh patch|major|X.Y.Z`
- `cargo build --release`
- `./packaging/scripts/build-deb.sh && ./packaging/scripts/validate-deb.sh`
- `./packaging/scripts/build-rpm.sh && ./packaging/scripts/validate-rpm.sh`

5. Commit and push to `fork`:
- Stage `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`.
- Commit with `Bump version to X.Y.Z`.
- `git push fork main`.

6. Tag and push:
- `git tag vX.Y.Z`
- `git push fork vX.Y.Z`

7. Publish the GitHub release:
- `gh release create vX.Y.Z dist/cmux_X.Y.Z_amd64.deb dist/cmux-X.Y.Z-*.rpm --repo TheBigRigZA/cmux-linux --title "vX.Y.Z" --notes "..."`

8. Verify:
- `gh release view vX.Y.Z --repo TheBigRigZA/cmux-linux` and confirm both `.deb` and `.rpm` assets are attached.

## Changelog Rules

- Include only user-visible changes.
- Exclude internal-only changes (CI, tests, docs-only edits, refactors without behavior changes).
- Write concise user-facing bullets in present tense.

## Contributor Credits

Credit the people who made each release happen:

- **Per-entry:** Append `— thanks @user!` for community code contributions. Use `— thanks @user for the report!` for bug reporters (when different from PR author). No callout for core team (`lawrencecchen`, `austinywang`) — core work is the baseline.
- **Summary:** Add a `### Thanks to N contributors!` section at the bottom of each release with an alphabetical list of all `[@handle](https://github.com/handle)` links (including core team).
- **GitHub Release body:** Include the same "Thanks to N contributors!" section with linked handles.
