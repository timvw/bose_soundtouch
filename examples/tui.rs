// #![cfg(feature = "websocket")]

// Only include imports when websocket feature is enabled
#[cfg(feature = "websocket")]
use {
    bose_soundtouch::{BoseClient, KeyValue, Preset},
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    image::load_from_memory,
    ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, List, ListItem, Paragraph},
        Frame, Terminal,
    },
    std::{collections::HashMap, io, time::Duration},
    tokio::time::sleep,
    tracing_appender::rolling::{RollingFileAppender, Rotation},
    tracing_subscriber::{fmt, prelude::*},
    viuer::Config as ViuerConfig,
};

#[cfg(feature = "websocket")]
struct App {
    client: BoseClient,
    device_info: Option<String>,
    now_playing: Option<String>,
    volume: Option<String>,
    presets: Option<Vec<Preset>>,
    preset_art_urls: HashMap<i32, String>,
    image_cache: HashMap<String, Vec<u8>>,
    current_art_url: Option<String>,
    last_drawn_art_url: Option<String>,
    last_drawn_preset_urls: HashMap<i32, String>,
}

#[cfg(feature = "websocket")]
impl App {
    fn new(hostname: &str) -> Self {
        let client = BoseClient::new_from_string(hostname);
        Self {
            client,
            device_info: None,
            now_playing: None,
            volume: None,
            presets: None,
            preset_art_urls: HashMap::new(),
            image_cache: HashMap::new(),
            current_art_url: None,
            last_drawn_art_url: None,
            last_drawn_preset_urls: HashMap::new(),
        }
    }

    async fn fetch_image(&mut self, url: &str) -> Option<&Vec<u8>> {
        if !self.image_cache.contains_key(url) {
            match reqwest::get(url).await {
                Ok(response) => match response.bytes().await {
                    Ok(bytes) => {
                        self.image_cache.insert(url.to_string(), bytes.to_vec());
                    }
                    Err(e) => {
                        tracing::error!("Failed to get image bytes: {}", e);
                        return None;
                    }
                },
                Err(e) => {
                    tracing::error!("Failed to download image: {}", e);
                    return None;
                }
            }
        }
        self.image_cache.get(url)
    }

    async fn update(&mut self) -> bose_soundtouch::Result<()> {
        // Update device info
        match self.client.get_info().await {
            Ok(info) => {
                // Get the first network info entry (SCM type)
                let network = info
                    .network_info
                    .iter()
                    .find(|n| n.network_type == "SCM")
                    .unwrap_or(&info.network_info[0]);

                self.device_info = Some(format!(
                    "Device: {} ({})\nIP: {}\nType: {}",
                    info.name, info.device_id, network.ip_address, info.device_type
                ));
            }
            Err(e) => {
                self.device_info = Some(format!("Error getting device info: {}", e));
            }
        }

        // Update now playing and fetch artwork
        match self.client.get_status().await {
            Ok(status) => {
                self.now_playing = Some(if status.source == "STANDBY" {
                    "Device is in standby mode".to_string()
                } else {
                    format!(
                        "Source: {}\nTrack: {}\nArtist: {}\nAlbum: {}",
                        status.source,
                        status.track.unwrap_or_default(),
                        status.artist.unwrap_or_default(),
                        status.album.unwrap_or_default()
                    )
                });

                // Update artwork URL
                if let Some(art) = status.art {
                    if let Some(url) = art.url {
                        self.current_art_url = Some(url);
                    }
                }
            }
            Err(e) => {
                self.now_playing = Some(format!("Error getting playback status: {}", e));
            }
        }

        // Update volume
        match self.client.get_volume().await {
            Ok(volume) => {
                tracing::debug!(
                    "Current volume: {}, muted: {}",
                    volume.actual,
                    volume.mute_enabled.unwrap_or(false)
                );
                self.volume = Some(format!(
                    "Volume: {}/100 ({})",
                    volume.actual,
                    if volume.mute_enabled.unwrap_or(false) {
                        "Muted"
                    } else {
                        "Unmuted"
                    }
                ));
            }
            Err(e) => {
                tracing::error!("Error getting volume: {}", e);
                self.volume = Some(format!("Error getting volume: {}", e));
            }
        }

        // Update presets and their artwork URLs
        match self.client.get_presets().await {
            Ok(presets) => {
                // Clear old preset art URLs
                self.preset_art_urls.clear();

                // Update preset art URLs
                for preset in &presets.items {
                    if !preset.content_item.container_art.is_empty() {
                        self.preset_art_urls
                            .insert(preset.id, preset.content_item.container_art.clone());
                    }
                }

                self.presets = Some(presets.items);
            }
            Err(e) => {
                tracing::error!("Error getting presets: {}", e);
            }
        }

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(5),  // Device info
                    Constraint::Length(20), // Now playing + artwork
                    Constraint::Length(3),  // Volume
                    Constraint::Length(8),  // Presets
                    Constraint::Length(8),  // Controls
                ]
                .as_ref(),
            )
            .split(f.area());

        // Device info
        let device_info = Paragraph::new(self.device_info.as_deref().unwrap_or("Loading..."))
            .block(Block::default().title("Device Info").borders(Borders::ALL));
        f.render_widget(device_info, chunks[0]);

        // Now playing with artwork
        let now_playing_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(chunks[1]);

        let now_playing = Paragraph::new(self.now_playing.as_deref().unwrap_or("Loading..."))
            .block(Block::default().title("Now Playing").borders(Borders::ALL));
        f.render_widget(now_playing, now_playing_chunks[0]);

        // Artwork placeholder
        let artwork =
            Paragraph::new("").block(Block::default().title("Artwork").borders(Borders::ALL));
        f.render_widget(artwork, now_playing_chunks[1]);

        // Volume
        let volume = Paragraph::new(self.volume.as_deref().unwrap_or("Loading..."))
            .block(Block::default().title("Volume").borders(Borders::ALL));
        f.render_widget(volume, chunks[2]);

        // Presets
        let preset_items: Vec<ListItem> = self
            .presets
            .as_ref()
            .map(|presets| {
                presets
                    .iter()
                    .map(|preset| {
                        ListItem::new(vec![Line::from(vec![
                            Span::raw(format!("#{} ", preset.id)),
                            Span::styled(
                                &preset.content_item.name,
                                Style::default().fg(Color::Yellow),
                            ),
                        ])])
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec![ListItem::new("Loading presets...")]);

        let presets =
            List::new(preset_items).block(Block::default().title("Presets").borders(Borders::ALL));
        f.render_widget(presets, chunks[3]);

        // Controls
        let controls = List::new(vec![
            ListItem::new("Space - Play/Pause"),
            ListItem::new("←/→ - Previous/Next Track"),
            ListItem::new("↑/↓ - Volume Up/Down"),
            ListItem::new("m - Mute"),
            ListItem::new("p - Power"),
            ListItem::new("1-6 - Select Preset"),
            ListItem::new("q - Quit"),
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL));
        f.render_widget(controls, chunks[4]);
    }
}

#[tokio::main]
#[cfg(feature = "websocket")]
async fn main() -> io::Result<()> {
    // Setup logging
    let file_appender = RollingFileAppender::new(Rotation::NEVER, "logs", "bose_soundtouch.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Create terminal after raw mode is enabled
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new("192.168.1.143");

    // Start WebSocket listener in background
    let client = BoseClient::new_from_string(app.client.hostname());
    tokio::spawn(async move {
        if let Err(e) = client.connect_and_listen().await {
            tracing::error!("WebSocket error: {}", e);
        }
    });

    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Return u16 for x coordinates and i16 for y coordinates
#[cfg(feature = "websocket")]
fn rect_to_screen_coords(rect: ratatui::layout::Rect) -> (u16, i16, u16, u16) {
    // Add 1 to account for borders
    let x = rect.x.saturating_add(1);
    let y = (rect.y.saturating_add(1)) as i16; // Convert y to i16
                                               // Subtract 2 to account for borders
    let width = rect.width.saturating_sub(2);
    let height = rect.height.saturating_sub(2);
    (x, y, width, height)
}

#[cfg(feature = "websocket")]
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    // Initial terminal clear
    terminal.clear()?;

    loop {
        if let Err(e) = app.update().await {
            app.device_info = Some(format!("Error: {}", e));
        }

        // Draw the TUI and store layout info
        let mut layout_info = None;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(5),  // Device info
                    Constraint::Length(20), // Now playing + artwork
                    Constraint::Length(3),  // Volume
                    Constraint::Length(8),  // Presets (single row)
                    Constraint::Length(8),  // Controls
                ])
                .split(f.area());

            let now_playing_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[1]);

            // Store layout info for later use
            layout_info = Some((now_playing_chunks[1], chunks[3]));

            // Draw the UI
            app.draw(f);
        })?;

        // Get layout info
        if let Some((now_playing_rect, presets_rect)) = layout_info {
            let (art_x, art_y, art_width, art_height) = rect_to_screen_coords(now_playing_rect);

            // Only redraw now playing artwork if URL changed
            if app.current_art_url != app.last_drawn_art_url {
                let now_playing_config = ViuerConfig {
                    width: Some(art_width as u32),
                    height: Some(art_height as u32),
                    x: art_x, // Keep as u16
                    y: art_y, // Already i16
                    use_kitty: false,
                    ..ViuerConfig::default()
                };

                if let Some(url) = &app.current_art_url {
                    let url = url.clone();
                    if let Some(image_data) = app.fetch_image(&url).await {
                        if let Ok(image) = load_from_memory(image_data) {
                            let _ = viuer::print(&image, &now_playing_config);
                        }
                    }
                }
                app.last_drawn_art_url = app.current_art_url.clone();
            }

            // Calculate preset artwork positions
            let (preset_base_x, preset_base_y, preset_total_width, _) =
                rect_to_screen_coords(presets_rect);
            let preset_width = preset_total_width / 6; // Divide width by 6 for all presets in one row

            // Only redraw preset artwork if URLs changed
            let current_preset_urls: HashMap<i32, (String, String)> =
                if let Some(presets) = &app.presets {
                    presets
                        .iter()
                        .filter_map(|preset| {
                            app.preset_art_urls.get(&preset.id).map(|url| {
                                (preset.id, (url.clone(), preset.content_item.name.clone()))
                            })
                        })
                        .collect()
                } else {
                    HashMap::new()
                };

            if current_preset_urls.keys().collect::<Vec<_>>()
                != app.last_drawn_preset_urls.keys().collect::<Vec<_>>()
            {
                for (preset_id, (url, _)) in &current_preset_urls {
                    let preset_x = preset_base_x + ((preset_id - 1) as u16 * preset_width);

                    // Draw the preset artwork first
                    if let Some(image_data) = app.fetch_image(url).await {
                        if let Ok(image) = load_from_memory(image_data) {
                            let config = ViuerConfig {
                                width: Some(preset_width as u32),
                                height: Some(5), // Full height for artwork
                                x: preset_x,
                                y: preset_base_y,
                                use_kitty: false,
                                ..ViuerConfig::default()
                            };
                            let _ = viuer::print(&image, &config);
                        }
                    }
                }
                app.last_drawn_preset_urls = current_preset_urls
                    .iter()
                    .map(|(id, (url, _))| (*id, url.clone()))
                    .collect();
            }
        }

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    tracing::debug!("Raw key event received: {:?}", key);
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Up => {
                                tracing::debug!("Up arrow pressed - sending volume up command");
                                if let Ok(volume) = app.client.get_volume().await {
                                    let new_volume = (volume.actual + 1).min(100);
                                    tracing::debug!(
                                        "Setting volume from {} to {}",
                                        volume.actual,
                                        new_volume
                                    );
                                    if let Err(e) = app.client.set_volume(new_volume).await {
                                        tracing::error!("Error setting volume: {}", e);
                                    }
                                }
                                sleep(Duration::from_millis(50)).await;
                            }
                            KeyCode::Down => {
                                tracing::debug!("Down arrow pressed - sending volume down command");
                                if let Ok(volume) = app.client.get_volume().await {
                                    let new_volume = volume.actual.saturating_sub(1);
                                    tracing::debug!(
                                        "Setting volume from {} to {}",
                                        volume.actual,
                                        new_volume
                                    );
                                    if let Err(e) = app.client.set_volume(new_volume).await {
                                        tracing::error!("Error setting volume: {}", e);
                                    }
                                }
                                sleep(Duration::from_millis(50)).await;
                            }
                            KeyCode::Char(' ') => {
                                let _ = app.client.play_pause().await;
                            }
                            KeyCode::Left => {
                                let _ = app.client.prev_track().await;
                            }
                            KeyCode::Right => {
                                let _ = app.client.next_track().await;
                            }
                            KeyCode::Char('m') => {
                                let _ = app.client.mute().await;
                            }
                            KeyCode::Char('p') => {
                                let _ = app.client.press_and_release_key(&KeyValue::Power).await;
                            }
                            KeyCode::Char(c) => {
                                if let Some(digit) = c.to_digit(10) {
                                    if digit >= 1 && digit <= 6 {
                                        let _ = app.client.set_preset(digit as i32).await;
                                    }
                                }
                            }
                            _ => tracing::debug!("Unhandled key: {:?}", key.code),
                        }
                    }
                }
                _ => {}
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}

// Fallback main for when websocket is disabled
#[cfg(not(feature = "websocket"))]
fn main() {
    println!("This example requires the 'websocket' feature to be enabled.");
    println!("Try running with: cargo run --example tui --features websocket");
}
