use clap::{Args, Parser, Subcommand};
use bose_soundtouch::BoseClient;

/// Control your Bose SoundTouch 20
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the current status
    Status,
    /// Press (and release) the power button
    Power,
    /// Press (and release) the play button
    Play,
    /// Press (and release) the pause button
    Pause,
    /// Get and set the volume
    Volume(VolumeArgs),
    /// Get and set a preset
    Preset(PresetArgs),
}

#[derive(Debug, Args)]
struct VolumeArgs {
    /// The new value for volume (0-100)
    value: Option<i32>,
}

#[derive(Debug, Args)]
struct PresetArgs {
    /// The new preset (1-6)
    value: Option<i32>,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    /// Hostname of the Bose system
    #[arg(long, env, default_value = "bose-woonkamer.local")]
    hostname: String,
}

#[tokio::main]
async fn main() {
    let app_args = AppArgs::parse();
    let client = BoseClient::new(&app_args.global_opts.hostname);
    let result = match app_args.command {
        Command::Status => Ok(()), //no-op for now client.print_status(),
        Command::Power => client.power().await,
        Command::Play => client.play().await,
        Command::Pause => client.pause().await,
        Command::Volume(volume_args) => match volume_args.value {
            Some(volume) => client.set_volume(volume).await,
            None => Ok(()), //no-op for now client.print_volume(),
        },
        Command::Preset(preset_args) => match preset_args.value {
            Some(preset) => client.set_preset(preset).await,
            None => Ok(()), // no-op for now client.print_presets(),
        },
    };

    match result {
        Err(e) => println!("Failed to execute command because {}", e),
        Ok(_) => {}
    }
}