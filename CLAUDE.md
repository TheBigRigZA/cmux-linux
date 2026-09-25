# cmux for Linux — agent notes

This is the Linux port of [manaflow-ai/cmux](https://github.com/manaflow-ai/cmux).
The upstream is a macOS Swift + AppKit terminal app embedding Ghostty; this
fork is a full **Rust + GTK4** rewrite that shadows the same v2 JSON-RPC
socket protocol. The Swift sources and Xcode project have been removed (see
the `fix(phaseA)` and `chore(phaseB)` commits on this branch); only the Rust
implementation is live on Linux.

## Initial setup

```bash
./scripts/setup-linux.sh              # initializes ghostty + agent-browser submodules
                                      # (the former with --force; see note below),
                                      # installs GTK4/clang/libc++ dev headers,
                                      # builds ghostty-internal.a
cargo build --release                 # builds cmux-app, cmux, cmux-generate, agent-browser
./scripts/install-cmuxd-remote.sh     # builds + installs the Go SSH daemon
```

The `--force` on the submodule update is required: the previously pinned
ghostty SHA (`4845e82d`) is no longer reachable on `manaflow-ai/ghostty`,
so an existing checkout will refuse to update without it.

Run the app from the build tree:

```bash
./target/release/cmux-app
```

The CLI talks to it over a Unix socket at `$XDG_RUNTIME_DIR/cmux/cmux.sock`
(override with `CMUX_SOCKET=` or `--socket`).

## Build system

| Component | Tool | Build command |
|---|---|---|
| ghostty-internal.a (static lib) | zig 0.15.2 | `scripts/setup-linux.sh` (wraps `zig build`) |
| `cmux-app`, `cmux`, `cmux-generate` | cargo / rustc | `cargo build [--release]` |
| `cmuxd-remote` (Go SSH helper) | go ≥1.22 | `scripts/install-cmuxd-remote.sh` |
| .deb / .rpm packages | shell wrappers | `packaging/scripts/build-deb.sh` / `build-rpm.sh` |

`build.rs` consumes `ghostty/zig-out/lib/ghostty-internal.a` (combined archive
that bundles simdutf, libhighway, glslang, dcimgui, SPIRV-Cross). The Linux
build links against **libc++ + libc++abi**, not libstdc++ — see the comment
block at the top of `build.rs` for the ABI rationale.

## Quick development loop

There is no `reload.sh` on Linux. The macOS-style tagged-build / DerivedData
workflow does not apply. Use cargo + binary execution:

```bash
cargo build && pkill -x cmux-app; ./target/debug/cmux-app &
```

To validate a release candidate:

```bash
./packaging/scripts/build-deb.sh && ./packaging/scripts/validate-deb.sh
./packaging/scripts/build-rpm.sh && ./packaging/scripts/validate-rpm.sh
```

## Testing policy

**Never run tests locally on the developer machine.** All tests run in CI
(GitHub Actions) or on a dedicated VM. The Python `tests_v2/` socket suite
requires a running cmux-app instance to talk to; launching one locally would
fight the developer's own session over the socket.

There is no Linux-equivalent of the macOS XCTest bundles; those were removed
with the Swift sources.

## Pitfalls

- **Submodule policy.** Only `ghostty` is needed for Linux. `vendor/bonsplit`
  and `homebrew-cmux` were removed in Phase B; do not re-add them.
- **Submodule safety.** When updating the ghostty submodule, always push the
  submodule commit to its remote `main` BEFORE updating the parent pointer.
  Detached HEAD commits will be orphaned. Verify with
  `cd ghostty && git merge-base --is-ancestor HEAD origin/main`.
- **C++ ABI.** Ghostty's bundled C++ deps link against libc++, not libstdc++.
  Adding new C++ link deps must follow the same convention. If you swap any
  link target back to `stdc++`, the build will surface "vtable / method not
  found" errors for `std::__1::*` symbols.
- **No separate simdutf.o / libhighway.a / stubs.o linking.** Those live
  inside `ghostty-internal.a`; pulling them in separately produces duplicate
  symbols. The historic build.rs lookup code is documented but disabled.
- **API drift TODOs.** Two surface-lifecycle exports
  (`ghostty_surface_display_realized` / `..._unrealized`) were available on
  the old fork pin but were not re-exported on current ghostty main. The Rust
  call sites in `src/ghostty/surface.rs` are stubbed with `TODO(cmux-linux)`
  comments — Phase C must restore them via fresh ghostty fork commits or
  switch to the new lifecycle API.
- **Socket commands must not steal focus.** Only the explicit focus-intent
  commands (`window.focus`, `workspace.select/next/previous/last`,
  `surface.focus`, `pane.focus/last`, browser focus commands) may mutate UI
  focus. All other commands preserve current focus context.
- **Socket telemetry handlers** (`report_*`, `ports_kick`, status/progress/log
  metadata) must run off the GTK main thread. Use the existing tokio + glib
  spawn_local bridge — never block the GTK main loop on JSON parsing.
- **Notifications.** `notify-rust` is unreliable on some Linux desktops;
  the app shells out to `notify-send` (`libnotify-bin` / `libnotify` on the
  package side). Make sure `notify-send` is in the runtime deps when editing
  packaging.
- **Localisation.** The macOS xcstrings system is gone. Linux UI strings are
  inline. Plan a `gettext`-based path if/when translation becomes a priority.

## Mycelium project tracking

This repo uses `myc` (Mycelium) for epic/task tracking. The `.mycelium/`
directory contains a SQLite DB committed to git. Standard cheatsheet lives
in the original macOS CLAUDE.md history — refer back to git history
(`git log --diff-filter=D -- CLAUDE.md`) if you need the full table.

## Release

The `bump-version.sh` script still bumps `Cargo.toml` and the packaging
versions; the old Xcode `MARKETING_VERSION` bump has been removed.

```bash
./scripts/bump-version.sh          # bump minor
./scripts/bump-version.sh patch    # 0.1.0 → 0.1.1
./scripts/bump-version.sh major    # 0.1.0 → 1.0.0
./scripts/bump-version.sh 1.0.0    # exact
```

Release workflow:

```bash
./scripts/bump-version.sh
./packaging/scripts/build-deb.sh
./packaging/scripts/build-rpm.sh
./packaging/scripts/validate-deb.sh
./packaging/scripts/validate-rpm.sh
git tag vX.Y.Z && git push fork vX.Y.Z
```

There is no Sparkle auto-update on Linux. Distribution is via .deb / .rpm
artefacts attached to GitHub release tags. AppImage and Flatpak are planned
for Phase 13 (post-port).

<!-- TEAM_MODE:START -->
## ⚡ Team Mode (conditional)
IMPORTANT: Before doing anything else, check whether `TEAM.md` exists in the project root.
- If `TEAM.md` exists: read it IN FULL before processing any task. You are operating as Tech
  Lead of a multi-agent team, not as a solo developer — `TEAM.md` contains all instructions.
- If `TEAM.md` does not exist: ignore this block entirely and proceed as a normal solo-developer
  session. `TEAM.md` is machine-local and gitignored (generated per-session by the fleet
  spawner); its absence means no team is currently active.
<!-- TEAM_MODE:END -->

## Agentic layer (TAC)

- Run `/prime` at session start for tiered, lane-specific context; this
  file stays rules + pointers.
- Validation for any change (respecting the testing policy above — no
  local test runs): `cargo build && cargo clippy --all-targets -- -D warnings`;
  for `daemon/remote/` add `go build ./... && go vet ./...`; for packaging
  run the build/validate-deb/rpm scripts. Close change reports with
  `git diff --stat`.
- Domain experts: `/experts:socket:question`, `/experts:ghostty-ffi:question`
  (maintained by their `self-improve` commands — never hand-edit
  `expertise.yaml`).
- Fleet: `just team <feature>` spawns the CMUX team defined in
  `cmux/team.config.json`; workers finish with `CMUX-LINUX-DONE: <role> | ...`.
- Plans go in `specs/`, fetched docs in `ai_docs/` (manifest only is
  committed), runtime team state in `.team/` (README only is committed).

## File index

When creating or modifying files, add/update a 1-2 sentence entry here.
DO NOT bloat this section with implementation details.

- `src/main.rs` — GTK4 application entry for `cmux-app`; wires window, state, socket server.
- `src/app_state.rs` — shared app state (windows, workspaces, surfaces) reached by UI and socket handlers.
- `src/split_engine.rs` — pane split tree layout and surface placement/teardown.
- `src/ghostty/` — libghostty FFI: surface lifecycle, runtime callbacks, key input translation.
- `src/socket/` — v2 JSON-RPC Unix-socket server: command table, handlers, auth.
- `src/cli/`, `src/bin/cmux.rs` — the `cmux` CLI client that talks to the socket.
- `src/ssh/`, `daemon/remote/` — SSH remote workspaces and the Go `cmuxd-remote` helper.
- `build.rs` — bindgen + static link of `ghostty-internal.a` against libc++.
- `packaging/` — .deb/.rpm build + validation scripts.
- `tests_v2/` — Python socket-protocol suite (CI/VM only).
- `cmux/team.config.json`, `cmux/roles/` — CMUX fleet roles and lanes.
- `scripts/spawn_team.py` — spawns the fleet (`just team <feature>`).
