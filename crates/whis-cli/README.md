<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/whis/main/crates/whis-desktop/icons/128x128.png" alt="whis" width="80" height="80" />

<h3>whis</h3>
<p>
  Your voice, piped to clipboard.
  <br />
  <a href="https://whis.ink">Website</a>
  ·
  <a href="https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop">Desktop</a>
  ·
  <a href="https://github.com/frankdierolf/whis/releases">Releases</a>
</p>
</div>

## Why?

- **Built for AI workflows** — speak your prompt, paste to Claude/Copilot
- **Cheap** — ~$0.006/minute via OpenAI Whisper or Mistral Voxtral (no local GPU)
- **Simple** — record → transcribe → clipboard
- **Multi-provider** — choose between OpenAI Whisper or Mistral Voxtral

## Quick Start

```bash
cargo install whis
whis config --openai-api-key sk-your-key-here  # or --mistral-api-key
whis
```

## Usage

**One-shot mode:**
```bash
whis    # Recording starts, press Enter to stop
```

**Hotkey mode (background service):**
```bash
whis listen                    # Global Ctrl+Shift+R anywhere
whis listen -k "super+space"   # Custom hotkey
whis status                    # Check if running
whis stop                      # Stop service
```

**Configuration:**
```bash
whis config --openai-api-key sk-...   # Save OpenAI API key
whis config --mistral-api-key ...     # Save Mistral API key
whis config --provider mistral        # Switch to Mistral Voxtral
whis config --language en             # Set language hint (ISO-639-1)
whis config --show                    # View current settings
```

## Requirements

- API key from [OpenAI](https://platform.openai.com/api-keys) or [Mistral](https://console.mistral.ai/api-keys)
- FFmpeg (`sudo apt install ffmpeg` or `brew install ffmpeg`)
- Linux (X11/Wayland) or macOS

**For hotkey mode** (one-time setup on Linux):
```bash
sudo usermod -aG input $USER
echo 'KERNEL=="uinput", GROUP="input", MODE="0660"' | sudo tee /etc/udev/rules.d/99-uinput.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
# Logout and login again
```

## Prefer a GUI?

See [whis-desktop](https://github.com/frankdierolf/whis/tree/main/crates/whis-desktop) — same functionality, with system tray.

## License

MIT
