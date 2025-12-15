use anyhow::{Context, Result};
use arboard::Clipboard;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::verbose;

/// Check if running inside a Flatpak sandbox
fn is_flatpak() -> bool {
    std::path::Path::new("/.flatpak-info").exists()
}

/// Get the current session type (x11, wayland, or unknown)
fn session_type() -> &'static str {
    std::env::var("XDG_SESSION_TYPE")
        .map(|s| match s.as_str() {
            "x11" => "x11",
            "wayland" => "wayland",
            _ => "unknown",
        })
        .unwrap_or("unknown")
}

/// Copy to clipboard using bundled wl-copy
///
/// In Flatpak, we bundle wl-clipboard and call wl-copy directly.
/// This is required because GNOME/Mutter does not implement the wlr-data-control
/// Wayland protocol that arboard's wayland-data-control feature requires.
fn copy_via_wl_copy(text: &str) -> Result<()> {
    crate::verbose!("Using wl-copy for clipboard (Flatpak environment)");

    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to spawn wl-copy")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .context("Failed to write to wl-copy")?;
    }

    let status = child.wait().context("Failed to wait for wl-copy")?;
    if !status.success() {
        anyhow::bail!("wl-copy exited with non-zero status");
    }

    crate::verbose!("wl-copy succeeded");
    Ok(())
}

/// Copy to clipboard using xclip (for X11)
///
/// arboard has issues on some X11 setups where it reports success but
/// doesn't actually set the clipboard. xclip is more reliable.
fn copy_via_xclip(text: &str) -> Result<()> {
    crate::verbose!("Using xclip for clipboard");

    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to spawn xclip. Install it with: sudo apt install xclip")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .context("Failed to write to xclip")?;
    }

    let status = child.wait().context("Failed to wait for xclip")?;
    if !status.success() {
        anyhow::bail!("xclip exited with non-zero status");
    }

    crate::verbose!("xclip succeeded");
    Ok(())
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    crate::verbose!("Copying to clipboard: {} chars", text.len());
    crate::verbose!("Session type: {}", session_type());
    crate::verbose!("Is Flatpak: {}", is_flatpak());

    if verbose::is_verbose() && !text.is_empty() {
        // Show a preview of the text (first 100 chars)
        let preview: String = text.chars().take(100).collect();
        let suffix = if text.len() > 100 { "..." } else { "" };
        crate::verbose!("Text preview: \"{preview}{suffix}\"");
    }

    // In Flatpak, use bundled wl-copy directly.
    // This is necessary because GNOME doesn't support wlr-data-control protocol.
    if is_flatpak() {
        return copy_via_wl_copy(text);
    }

    // On X11, use xclip which is more reliable than arboard
    if session_type() == "x11" {
        return copy_via_xclip(text);
    }

    // Standard approach using arboard (Wayland non-Flatpak)
    crate::verbose!("Using arboard for clipboard");
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
    clipboard
        .set_text(text)
        .context("Failed to copy text to clipboard")?;

    crate::verbose!("Clipboard set successfully via arboard");

    Ok(())
}
