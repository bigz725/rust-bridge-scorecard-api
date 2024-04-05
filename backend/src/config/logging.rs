use std::str::FromStr;
use std::{fs::File, io, path::PathBuf};
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, registry::LookupSpan, Layer};

pub enum LogConfig {
    File(PathBuf),
    JsonFile(PathBuf),
    Stdout,
    //Stderr
}

impl LogConfig {
    pub fn layer<S>(self) -> Box<dyn Layer<S> + Send + Sync + 'static>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let filter = EnvFilter::builder()
            .with_default_directive(env_level().into()) //LevelFilter::INFO.into())
            .from_env_lossy();
        // Shared configuration regardless of where logs are output to.
        let fmt = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_names(true);

        // Configure the writer based on the desired log target:
        match self {
            LogConfig::File(path) => {
                let file = File::create(path).expect("failed to create text log file");
                Box::new(fmt.with_writer(file).with_filter(filter))
            }
            LogConfig::JsonFile(path) => {
                let file = File::create(path).expect("failed to create json log file");
                Box::new(fmt.json().with_writer(file).with_filter(filter))
            }
            LogConfig::Stdout => Box::new(fmt.with_writer(io::stdout).pretty().with_filter(filter)),
            //LogConfig::Stderr => Box::new(fmt.with_writer(io::stderr).pretty()),
        }
    }
}

pub fn init_logging() {
    let std_out_config = LogConfig::Stdout;
    let file_config = LogConfig::File("logs/log.ansi".into());
    let json_config = LogConfig::JsonFile("logs/log.json".into());
    //let std_err_config = LogConfig::Stderr;
    tracing_subscriber::registry()
        .with(std_out_config.layer())
        .with(json_config.layer())
        .with(file_config.layer())
        //.with(std_err_config.layer())
        .init();
}

pub fn env_level() -> Level {
    let log_level_str = std::env::var("LOG_LEVEL").unwrap_or("debug".to_string());
    Level::from_str(&log_level_str).unwrap_or(Level::DEBUG)
}
