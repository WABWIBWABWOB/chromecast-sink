> **Fork of [`Nerahikada/chromecast-sink`](https://github.com/Nerahikada/chromecast-sink) with Windows builds.**
>
> - **Speaker groups** (Cast port from mDNS) and the `pipewire-rs` 0.10 build are upstream now.
> - **Windows support** (WASAPI loopback, see [Windows](#windows) below) is offered upstream as
>   [PR #5](https://github.com/Nerahikada/chromecast-sink/pull/5). Prebuilt `.exe` on this fork's Releases page.
>   Two differences to the PR as it stands: this build captures **CABLE Input** by default (the PR captures
>   the default playback device), and it has a [stop event](#stopping-a-hidden-instance-windows) for launchers
>   (dropped from the PR on review).
> - **`--host <IP> --port <PORT>`**: upstream has its own `--host`/`--port` now. This fork's version additionally
>   accepts `--device` together with `--host`, for a speaker-group leader that stops answering multicast mDNS while idle.
>
> The rest below is the upstream README plus the Windows section.

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

```bash
# Skip mDNS and connect to a speaker group's leader directly (fork only)
chromecast-sink --device "Living Room group" --host 192.0.2.10 --port 32012
```

The group's port is in its mDNS SRV record, or in the leader's `eureka_info` under `multizone.groups[].cast_port`.

## Windows

Windows can't create a virtual output device from a normal app, so on Windows chromecast-sink captures an existing playback device via WASAPI loopback instead. Install [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) (free) and use its **CABLE Input** device, which is the default:

```powershell
chromecast-sink.exe --list-devices                       # playback devices it can capture
chromecast-sink.exe --device "Living Room speaker"       # casts whatever plays on "CABLE Input"
chromecast-sink.exe --device "Living Room speaker" --capture-device "Speakers"
```

Then select **CABLE Input** as your output in Settings > Sound. Nothing plays locally on that device, so it behaves like the Linux sink. Windows may ask for a firewall exception on the first run; mDNS discovery needs inbound UDP on private networks.

### Stopping a hidden instance (Windows)

A process started without a console window can't receive Ctrl+C, and killing it skips the receiver teardown, so the speaker keeps waiting for a stream that is gone. This build therefore listens on the named event `Local\chromecast-sink-stop`. Signalling it stops the cast the same way Ctrl+C does:

```powershell
([System.Threading.EventWaitHandle]::OpenExisting('Local\chromecast-sink-stop')).Set()
```

The event is per session (`Local\`), so only processes of the same user session can stop it. Fork only; it was dropped from the upstream PR because upstream has no launcher that uses it.

**Build**: Rust 1.85+, Visual Studio Build Tools (C++), and CMake.

## Scope

This tool does low-latency Cast Streaming to a single device **or a Google speaker group**.

If you want HTTP-based streaming instead — broader device compatibility and a virtual sink per discovered device, at the cost of several seconds of latency — use [p-cast](https://github.com/GenessyX/p-cast), which serves HLS segments over HTTP.

## Known Limitations

- **No auto-reconnect**: if the Chromecast disconnects, the tool exits cleanly — restart to reconnect
