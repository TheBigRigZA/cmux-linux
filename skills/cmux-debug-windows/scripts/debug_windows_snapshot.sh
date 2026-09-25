#!/usr/bin/env bash
set -euo pipefail

echo "debug_windows_snapshot.sh is INACTIVE on cmux-linux." >&2
echo "It read Sidebar/Background/Menu Bar Extra debug values via macOS 'defaults' and" >&2
echo "'pbcopy', both of which are macOS-only tooling that removed with the Swift sources" >&2
echo "during the Rust + GTK4 port (see CLAUDE.md, skills/cmux-debug-windows/SKILL.md)." >&2
echo "There is no GTK4 equivalent to port this to. Use 'cmux identify' / 'cmux list-windows' /" >&2
echo "'cmux list-workspaces' / 'cmux list-panes' / 'cmux list-surfaces' / 'cmux health' /" >&2
echo "'cmux read-text' for pane/window introspection, or run cmux-app with" >&2
echo "GTK_DEBUG=interactive for the GTK inspector." >&2
exit 1
