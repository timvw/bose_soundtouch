use anyhow::{Context, Result};
use bose_soundtouch::BoseClient;
use clap::{Args, Parser, Subcommand};

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
    let client = BoseClient::new_from_string(&app_args.global_opts.hostname);
    let result = match app_args.command {
        Command::Status => print_status(&client).await,
        Command::Power => client
            .power()
            .await
            .with_context(|| "Failed to switch power".to_string()),
        Command::Play => client
            .play()
            .await
            .with_context(|| "Failed to send play".to_string()),
        Command::Pause => client
            .pause()
            .await
            .with_context(|| "Failed to send pause".to_string()),
        Command::Volume(volume_args) => match volume_args.value {
            Some(volume) => client
                .set_volume(volume)
                .await
                .with_context(|| "Failed set volume".to_string()),
            None => print_volume(&client).await,
        },
        Command::Preset(preset_args) => match preset_args.value {
            Some(preset) => client
                .set_preset(preset)
                .await
                .with_context(|| "Failed to change preset".to_string()),
            None => print_presets(&client).await,
        },
    };

    if let Err(e) = result {
        println!("Failed to execute command because {}", e);
    }
}

async fn print_status(client: &BoseClient) -> Result<()> {
    let status = client.get_status().await?;
    println!("Status: {:?}", status);
    Ok(())
}

async fn print_volume(client: &BoseClient) -> Result<()> {
    let volume = client.get_volume().await?;
    println!("Volume: {:?}", volume.actual);
    Ok(())
}

async fn print_presets(client: &BoseClient) -> Result<()> {
    let presets = client.get_presets().await?;
    println!("the presets are: ");
    for preset in presets.items {
        println!(
            "{} - {} ({})",
            preset.id, preset.content_item.name, preset.content_item.source
        )
    }
    Ok(())
}
