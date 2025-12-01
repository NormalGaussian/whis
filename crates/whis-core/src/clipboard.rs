use anyhow::{Context, Result};
use arboard::Clipboard;
use std::io::Write;
use std::process::{Command, Stdio};

/// Check if running inside a Flatpak sandbox
fn is_flatpak() -> bool {
    std::path::Path::new("/.flatpak-info").exists()
}

/// Copy to clipboard using flatpak-spawn to access host's wl-copy
///
/// This is required because GNOME/Mutter does not implement the wlr-data-control
/// Wayland protocol that arboard's wayland-data-control feature requires.
fn copy_via_flatpak_spawn(text: &str) -> Result<()> {
    let mut child = Command::new("flatpak-spawn")
        .args(["--host", "wl-copy"])
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to spawn flatpak-spawn")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .context("Failed to write to wl-copy")?;
    }

    let status = child.wait().context("Failed to wait for wl-copy")?;
    if !status.success() {
        anyhow::bail!("wl-copy exited with non-zero status");
    }

    Ok(())
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    // In Flatpak, use flatpak-spawn to call host's wl-copy.
    // This is necessary because GNOME doesn't support wlr-data-control protocol.
    if is_flatpak() {
        return copy_via_flatpak_spawn(text);
    }

    // Standard approach for non-Flatpak environments
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
    clipboard
        .set_text(text)
        .context("Failed to copy text to clipboard")?;

    Ok(())
}
