//! What a newly created terminal should start with.
//!
//! Ghostty's surface config (`ghostty_surface_config_s` in ghostty.h) has
//! always carried `working_directory` and `command`. Nothing on the Rust side
//! ever set them, so `cmux split --cwd ...` had nowhere to put the value and
//! the flag did not exist. This is the value that fills them in.

/// Overrides applied to a new terminal surface at creation time.
///
/// `None` means inherit: for a split that is the parent surface's config
/// (D-08 CWD inheritance), for a new workspace it is the app default. An
/// override never becomes an empty string — the socket layer filters those
/// out, because `--cwd ""` meaning "/" would be a nasty surprise.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpawnSpec {
    /// Directory the new shell starts in.
    pub cwd: Option<String>,
    /// Command to run instead of the default shell.
    pub command: Option<String>,
}
