use clap::Parser;

use chromecast_sink::pipeline;

#[derive(Parser)]
#[command(
    name = "chromecast-sink",
    version,
    about = "Use a Google Chromecast / Nest device as a Linux speaker output. Creates a PipeWire virtual sink that streams audio to the Chromecast."
)]
struct Cli {
    /// Connect to a specific device by name.
    #[arg(short, long, value_name = "NAME")]
    device: Option<String>,

    /// Skip mDNS discovery and connect to this IP/hostname directly. Use with --port for speaker groups
    /// (the group's Cast port is in its mDNS SRV record or in the leader's eureka_info `multizone.groups[].cast_port`).
    /// --device then only names the sink; the target is not verified against it.
    #[arg(long, value_name = "HOST", requires = "device")]
    host: Option<String>,

    /// Cast v2 TCP port for --host (devices: 8009; speaker groups: leader-specific, e.g. 32012).
    #[arg(long, value_name = "PORT", default_value_t = 8009, requires = "host")]
    port: u16,

    /// Enable verbose (debug) logging.
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let level = if cli.verbose { "debug" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level))
        .format_timestamp_millis()
        .init();

    let result = match cli.host {
        Some(host) => pipeline::run_with_host(cli.device.as_deref().unwrap_or(&host), &host, cli.port),
        None => pipeline::run(cli.device.as_deref()),
    };
    if let Err(e) = result {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
