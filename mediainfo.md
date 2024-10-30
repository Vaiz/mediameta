# MediaInfo

To extract metadata from a wide range of media formats, this library primarily uses native Rust
libraries. In cases where these libraries do not support a specific format or fail, the library
falls back on a third-party tool called `MediaInfo`. This tool needs to be installed separately.

- MediaInfo official website: [mediaarea.net/en/MediaInfo](https://mediaarea.net/en/MediaInfo)

## Installing MediaInfo

The `mediainfo` tool is required to use the [`extract_metadata`] function. Follow the instructions
below to install `mediainfo` on your system.

### Windows

To install `mediainfo` on Windows, you can use the `winget` package manager:

```
winget install MediaArea.MediaInfo
```

> **Note:** It might be required to restart the shell to make the alias available.

### macOS

For macOS users, `mediainfo` can be installed using Homebrew:

```
brew install mediainfo
```

### Linux

On most Linux distributions, `mediainfo` can be installed via the package manager. For example:

```
sudo apt-get install mediainfo  # Debian/Ubuntu
sudo yum install mediainfo      # CentOS/RHEL
sudo pacman -S mediainfo        # Arch Linux
```

### Verifying Installation

After installation, verify that `mediainfo` is available by running:

```
mediainfo --version
```

If correctly installed, this should display the version of `mediainfo`.
