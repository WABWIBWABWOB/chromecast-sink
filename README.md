> **Fork of [`Nerahikada/chromecast-sink`](https://github.com/Nerahikada/chromecast-sink).**
>
> The speaker-group support that this branch was made for (Cast port from mDNS instead of the hardcoded
> `8009`) and the `pipewire-rs` 0.10 build are **upstream now**, so for Linux, use upstream.
>
> This fork's current work is on the branch
> [`luna/windows-wasapi`](https://github.com/WABWIBWABWOB/chromecast-sink/tree/luna/windows-wasapi):
> Windows support (offered upstream as [PR #5](https://github.com/Nerahikada/chromecast-sink/pull/5)),
> prebuilt binaries on the [Releases](https://github.com/WABWIBWABWOB/chromecast-sink/releases) page.
>
> The rest below is the upstream README as of this branch.

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

This tool does low-latency Cast Streaming to a single device **or a Google speaker group** (this fork adds the speaker-group part).

If you want HTTP-based streaming instead — broader device compatibility and a virtual sink per discovered device, at the cost of several seconds of latency — use [p-cast](https://github.com/GenessyX/p-cast), which serves HLS segments over HTTP.

## Known Limitations

- **No auto-reconnect**: if the Chromecast disconnects, the tool exits cleanly — restart to reconnect
