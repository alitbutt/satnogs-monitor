use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct StationConfig {
    #[serde(default)]
    pub local: bool,
    pub satnogs_id: u64,
    pub rt_ip: Option<String>,
    pub rt_port: Option<u32>,
}

impl StationConfig {
    pub fn new(id: u64) -> Self {
        StationConfig {
            local: false,
            satnogs_id: id,
            rt_ip: None,
            rt_port: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UiConfig {
    pub db_min: f32,
    pub db_max: f32,
    pub ground_track_num: u8,
    pub rotator_warn: f64,
    pub rotator_error: f64,
    pub sat_footprint: bool,
    pub spectrum_plot: bool,
    pub waterfall: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api_endpoint: String,
    pub job_update_interval: u64,
    pub log_level: Option<u64>,
    pub ui: UiConfig,
    pub rotctld_address: Option<String>,
    pub rotctld_interval: u64,
    pub stations: Vec<StationConfig>,
    pub data_path: Option<String>,
    pub waterfall_zoom: f32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.set_default("api_endpoint", "https://network.satnogs.org/api/")?;
        settings.set_default("job_update_interval", 600)?;
        settings.set_default("log_level", 0)?;
        settings.set_default("ui.db_min", -100.0)?;
        settings.set_default("ui.db_max", 0)?;
        settings.set_default("ui.ground_track_num", 3)?;
        settings.set_default("ui.rotator_warn", 5.0)?;
        settings.set_default("ui.rotator_error", 15.0)?;
        settings.set_default("ui.sat_footprint", true)?;
        settings.set_default("ui.spectrum_plot", false)?;
        settings.set_default("ui.waterfall", false)?;
        settings.set_default("rotctld_interval", 5)?;
        settings.set_default("stations", Vec::<config::Value>::new())?;
        settings.set_default("waterfall_zoom", 1.0)?;

        if let Some(project_dirs) = ProjectDirs::from("org", "SatNOGS", "satnogs-monitor") {
            let file = File::with_name(
                project_dirs
                    .config_dir()
                    .join("config.toml")
                    .to_str()
                    .ok_or_else(|| ConfigError::Message("Invalid project dir".to_string()))?,
            );
            settings.merge(file.required(false))?;
        }

        settings.try_into()
    }

    pub fn from_file(file: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.merge(File::with_name(file))?;
        settings.try_into()
    }
}
