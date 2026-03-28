# 👁 vigil

> A lightweight, systemd-native file watcher for Linux.

Vigil monitors a directory recursively and logs file system events — creates, modifications, and deletions — straight to your system journal via `journald`. It integrates natively with systemd's watchdog protocol, making it a well-behaved, production-ready service.

---

## Features

- 📁 **Recursive directory watching** — monitors an entire directory tree for `.rs` file changes
- 📓 **journald integration** — logs events directly to the system journal, no log files to manage
- 🐕 **Systemd watchdog support** — emits keepalive pings so systemd can restart the service if it hangs
- 🔊 **Dynamic log levels** — toggle between `TRACE` and `INFO` at runtime via Unix signals, no restart needed
- 📦 **Debian package** — ships as a `.deb` for easy installation on Debian/Ubuntu systems

---

## Installation

### From a `.deb` package

```sh
sudo dpkg -i vigil_*.deb
```

After installation, you **must** configure the watch path before starting the service (see [Configuration](#configuration)).

---

## Configuration

Vigil is configured entirely via environment variables, which you set through a systemd drop-in override.

Run:

```sh
sudo systemctl edit vigil
```

And add:

```ini
[Service]
Environment=VIGIL_MONITOR_ROOT=/path/to/watch
```

Replace `/path/to/watch` with the directory you want vigil to monitor.

---

## Usage

### Start the service

```sh
sudo systemctl start vigil
```

### Enable it on boot

```sh
sudo systemctl enable vigil
```

### View logs

```sh
journalctl -u vigil -f
```

---

## Dynamic Log Levels

You can change the log verbosity at runtime without restarting the service:

| Signal | Effect |
|--------|--------|
| `SIGUSR1` | Switch to `TRACE` (verbose) |
| `SIGUSR2` | Switch back to `INFO` (normal) |

```sh
# Enable verbose logging
sudo kill -USR1 $(pidof vigil)

# Back to normal
sudo kill -USR2 $(pidof vigil)
```

---

## Building from Source

```sh
git clone https://github.com/clement-bramy/vigil
cd vigil
cargo build --release
```

To build a `.deb` package:

```sh
cargo install cargo-deb
cargo deb
```

The package will be output to `target/debian/`.

---

## License

MIT — see [LICENSE](LICENSE) for details.
