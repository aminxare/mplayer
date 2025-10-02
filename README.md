# 🎵 Terminal Music Player

A terminal-based music player written in **Rust** for education, using:
- [rodio](https://github.com/RustAudio/rodio) for audio playback
- [ratatui](https://github.com/ratatui-org/ratatui) for the terminal user interface

This project aims to provide a lightweight, keyboard-driven music player that runs entirely in the terminal.

---

## ✨ Features

- Play, pause, and stop music
- Display current track information (title, artist, album)
- Playlist navigation
- Minimal and responsive TUI with `ratatui`

---

## 📦 Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- A terminal that supports Unicode

### Build from Source
```bash
git clone https://github.com/aminxare/mplayer.git
cd mplayer
cargo build --release
```

The binary will be located at:
```bash
target/release/mplayer
```

## ▶️ Usage

Run the player with a folder:
```bash
cargo run -- path/to/music/folder
```

### Controls (default key bindings)
- c or p → Play/Pause
- Enter → Select track
- q → Quit

### 🔮 Roadmap

- Volume control
- Shuffle and repeat modes
- Metadata fetching for more formats
- Better error handling & logging

### 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to open a pull request or issue in the [GitHub repository](https://github.com/aminxare/mplayer)
.