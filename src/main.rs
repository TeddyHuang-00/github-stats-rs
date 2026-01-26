mod color;
mod config;
mod icons;
mod image;
mod stat;

use anyhow::Result;
use log::{LevelFilter, debug};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::default();
    SimpleLogger::new()
        .with_level(match config.log_level {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .with_module_level("octocrab", LevelFilter::Warn)
        .init()?;
    debug!("{config:?}");

    let statistics = stat::Statistics::fetch(&config).await?;
    let color = color::Color::fetch(&config).await?;

    let lang_image = image::LanguageImage::new(&color, &statistics);
    lang_image.save()?;

    let overview_image = image::OverviewImage::new(&statistics);
    overview_image.save()?;

    Ok(())
}
