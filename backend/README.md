# Torrust Web Backend

Torrust is an open source web based BitTorrent tracker developed in Rust.
It allows users to upload and download torrents on a web UI, and tracks peers with an UDP BitTorrent tracker.

[Documentation](https://torrust.github.io/torrust-documentation/torrust-web-backend/about/)

## Features
* [X] Login / Register
* [X] Authentication using JWT tokens
* [X] E-mail verification
* [X] Torrent Uploading / Downloading

## Getting started
The easiest way is to get built binaries from [Releases](https://github.com/torrust/torrust-web-backend/releases),
but building from sources is also possible:

```bash
git clone https://github.com/torrust/torrust.git
cd torrust
cargo build --release
```

__Notice:__ Skip the first step if you've downloaded the binaries directly.

1. After building __Torrust__, navigate to the folder.
```bash
cd torrust/target
```

2. Create a file called `config.toml` with the following contents and change the [configuration](https://torrust.github.io/torrust-tracker/CONFIG.html) according to your liking.


3. And run __Torrust__:
```bash
./torrust
```

#### Note : We recommond you to use GoBang to manage database (A cross-platform TUI database management tool written in Rust)

[GoBang](https://github.com/TaKO8Ki/gobang)

## Contributing
Please report any bugs you find to our issue tracker. Ideas and feature requests are welcome as well!
Any pull request targeting existing issues would be very much appreciated.
