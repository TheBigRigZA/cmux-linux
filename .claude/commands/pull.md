# Pull

Pull latest main and update all submodules to their latest remote branch. No commits, no
pushes.

## Steps

1. `git pull origin main`
2. For each submodule:
   - `ghostty` — tracks branch `cmux-fork-gtk4-platform` on `origin` (TheBigRigZA/ghostty.git,
     per `.gitmodules`)
     - `cd ghostty && git fetch origin`
     - Check if behind: `git rev-list HEAD..origin/cmux-fork-gtk4-platform --count`
     - If behind: `git merge origin/cmux-fork-gtk4-platform --no-edit`
   - `agent-browser` — tracks branch `main` on `origin` (vercel-labs/agent-browser.git)
     - `cd agent-browser && git fetch origin`
     - Check if behind: `git rev-list HEAD..origin/main --count`
     - If behind: `git merge origin/main --no-edit`
   - Do NOT push either submodule. We only land submodule changes via PRs.
   - Go back to repo root after each
3. `git submodule update --init --recursive`
4. Report: current commit, which submodules were updated and by how many commits
