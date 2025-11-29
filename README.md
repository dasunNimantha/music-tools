# Music Metadata Editor

A native Rust GUI application for editing music file metadata using Iced framework.

## Features

- Select multiple audio files or entire folders
- Remove all existing metadata (including album art)
- Set custom artist and album names
- Supports: MP3, FLAC, M4A, OGG, WMA, AAC, MP4, OPUS

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- System dependencies for Iced (usually pre-installed on Linux)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

Or run the release binary:

```bash
./target/release/music-metadata-editor
```

## Usage

1. Click "Select Files" to choose individual audio files, or "Select Folder" to select a directory
2. Enter the desired Artist and Album names
3. Click "Process Files" to remove all metadata and set the new values
