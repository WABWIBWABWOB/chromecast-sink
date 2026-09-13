> **Fork with Google speaker-group support.**
>
> The upstream project [`Nerahikada/chromecast-sink`](https://github.com/Nerahikada/chromecast-sink)
> streams to a single Cast device. This fork adds one thing: `--device "<group name>"` now streams to a
> whole **Google speaker group** (all members in sync), by using the Cast port advertised over mDNS
> instead of the hardcoded `8009` (groups run on the leader on a different port, e.g. `32012`).
> It also builds on current toolchains (`pipewire-rs` 0.10).
>
> The build fix was offered back upstream; the speaker-group change lives here because the upstream
> project intentionally targets a single device (and points to
> [`p-cast`](https://github.com/GenessyX/p-cast) for groups over HLS/HTTP, which has ~3 s of latency).
>
> Everything below is the original README, unchanged.

---

# chromecast-sink

Use Google Chromecast / Nest devices as a Linux speaker output.

## Requirements

**OS**: Linux with PipeWire (tested on Ubuntu 24.04, 26.04).

22.04 works too, but you'll need to switch its audio stack from PulseAudio to PipeWire first.

**Build**: Rust 1.85+ and:

```bash
sudo apt install build-essential libpipewire-0.3-dev libopus-dev libclang-dev pkg-config
```

## Installation

```bash
git clone https://github.com/Nerahikada/chromecast-sink.git
cd chromecast-sink
cargo install --path .
```

## Usage

```bash
# Stream to the only device found, or list them and ask for --device
chromecast-sink

# Specify device by name
chromecast-sink --device "Living Room speaker"
```

After starting, select **"Chromecast - \<device name\>"** as your audio output in Settings > Sound.

## Scope

This tool does one thing: low-latency Cast Streaming to a single device.

If you want HTTP-based streaming instead — broader device compatibility and a virtual sink per discovered device, at the cost of several seconds of latency — use [p-cast](https://github.com/GenessyX/p-cast), which serves HLS segments over HTTP.

## Known Limitations

- **No auto-reconnect**: if the Chromecast disconnects, the tool exits cleanly — restart to reconnect
