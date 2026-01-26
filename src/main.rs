mod color;
mod config;
mod i18n;
mod icons;
mod image;
mod stat;

use anyhow::Result;
use log::debug;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::default();
    debug!("{config:?}");

    let statistics = stat::Statistics::fetch(&config).await?;
    let color = color::Color::fetch(&config).await?;

    let lang_image = image::LanguageImage::new(config.locale, &color, &statistics);
    lang_image.save()?;

    let overview_image = image::OverviewImage::new(config.locale, &statistics);
    overview_image.save()?;

    Ok(())
}
