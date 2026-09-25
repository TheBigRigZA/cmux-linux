# Sync Branch

Get the current branch ready: update all submodules to their latest remote branch, merge from
main, and rebase.

**Important: Never push automatically. Always ask the user before any push.**

## Steps

1. **Update submodules to latest**
   - `ghostty` (branch `cmux-fork-gtk4-platform`):
     - `cd ghostty && git fetch origin`
     - Check if behind: `git rev-list HEAD..origin/cmux-fork-gtk4-platform --count`
     - If behind: `git merge origin/cmux-fork-gtk4-platform --no-edit`
   - `agent-browser` (branch `main`):
     - `cd agent-browser && git fetch origin`
     - Check if behind: `git rev-list HEAD..origin/main --count`
     - If behind: `git merge origin/main --no-edit`
   - Do NOT push submodules. We only land submodule changes via PRs.
   - Go back to repo root after each

2. **Commit submodule updates on main**
   - `git checkout main && git pull origin main`
   - Check if any submodules changed: `git diff --name-only` (look for `ghostty` / `agent-browser`
     in the output — only add paths that actually appear there)
   - If changed, stage only the submodule paths that actually appear in `git diff --name-only`,
     e.g. `git add ghostty agent-browser && git commit -m "Update submodules: <brief
     description>"` — never add a path that isn't listed in `.gitmodules`
     (`homebrew-cmux`, `vendor/bonsplit` no longer exist and must never be passed to `git add`)
   - **Do not push.** Ask the user if they want to push (target remote: `fork`).

3. **Rebase current branch on main**
   - `git checkout <original-branch>`
   - `git rebase main`
   - If conflicts, resolve them and continue
   - **Do not push.** Ask the user if they want to force-push the rebased branch (target remote:
     `fork`).

4. **Report status**
   - Show what submodules were updated and by how many commits
   - Show if rebase was clean or had conflicts
   - Show current branch and commit

## Notes

- Never commit a submodule pointer in the parent repo unless the submodule commit is reachable
  from the submodule's remote `origin` branch (`cmux-fork-gtk4-platform` for ghostty, `main` for
  agent-browser) — per CLAUDE.md's submodule-safety pitfall about orphaned commits. Verify with
  `git merge-base --is-ancestor HEAD origin/<branch>` inside the submodule before committing its
  pointer.
- If no submodules need updating and main has no new commits, just say "Already up to date"
- If on main already, skip step 3
