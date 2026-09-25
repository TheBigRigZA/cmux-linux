---
name: cmux-debug-windows
description: "INACTIVE on cmux-linux. This skill managed macOS SwiftUI debug windows (Sidebar/Background/Menu Bar Extra Debug) that were removed with the Swift sources during the Linux GTK4 port. Do not invoke; no GTK4 equivalent exists yet."
---

# cmux Debug Windows (inactive on Linux)

This skill is a leftover from the macOS Swift + AppKit build. It tuned NSUserDefaults-backed
debug panes in `Sources/cmuxApp.swift` and `Sources/AppDelegate.swift` — both files were
removed when this repo was rewritten as a Rust + GTK4 app (see CLAUDE.md). There is no
`xcodebuild -project GhosttyTabs.xcodeproj` step and no `./scripts/reload.sh` /
`reloadp.sh` / `reloads.sh` on Linux; none of those exist in this repo.

The Linux port currently has no equivalent debug-window surface: no GTK inspector wiring
of its own, no `RUST_LOG`/logging crate integration here, no custom `GTK_DEBUG` handling in
`src/`. What actually exists today:

- `cmux identify`, `cmux list-windows`, `cmux list-workspaces`, `cmux list-panes`,
  `cmux list-surfaces`, `cmux health`, `cmux read-text` — the CLI's real window/workspace/
  pane/surface introspection commands (`src/cli/mod.rs`, `src/socket/mod.rs`). `cmux identify`
  flashes a window/pane for visual identification; it is unrelated to this skill's original
  scope.
- `GTK_DEBUG=interactive ./target/debug/cmux-app` — GTK4's own built-in interactive
  inspector, launched via the standard `GTK_DEBUG` environment variable, not anything this
  skill wires up.

Do not invoke this skill. If GTK4-native debug tooling (inspector toggle, log-level control,
etc.) is added in the future, replace this file with a real workflow at that time; until then
`scripts/debug_windows_snapshot.sh` (macOS `defaults`/`pbcopy`-based) is dead code kept for
reference only and exits non-zero if run.
