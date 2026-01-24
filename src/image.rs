use std::fs;

use anyhow::Result;

use crate::{color::Color, stat::Statistics};

const WORKSPACE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const TEMPLATE_DIR: &str = "templates";
const OUTPUT_DIR: &str = "generated";

pub struct LanguageImage {
    pub progress: String,
    pub lang_list: String,
}

impl LanguageImage {
    const FILENAME: &str = "languages.svg";
    const ANIMATION_DELAY_MS: u64 = 100;

    pub fn new(color: &Color, stats: &Statistics) -> Self {
        let progress = stats.langs.iter().map(|(lang, percent) | {
            let color = color.get_color(lang);
            format!(
                r#"<span style="background-color: {color}; width: {percent:.3}%;" class="progress-item"></span>"#,
            )
        }).collect::<Vec<_>>().join("\n");
        let lang_list = stats
            .langs
            .iter()
            .enumerate()
            .map(|(i, (lang, percent))| {
                let color = color.get_color(lang);
                let delay = i as u64 * Self::ANIMATION_DELAY_MS;
                format!(
                    r#"<li style="animation-delay: {delay}ms;">
                    <svg xmlns="http://www.w3.org/2000/svg" class="octicon" style="fill:{color};"
                    viewBox="0 0 16 16" version="1.1" width="16" height="16"><path
                    fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
                    <span class="lang">{lang}</span>
                    <span class="percent">{percent:.2}%</span>
                    </li>"#
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        Self {
            progress,
            lang_list,
        }
    }

    pub fn save(&self) -> Result<()> {
        let template_path = format!("{WORKSPACE_ROOT}/{TEMPLATE_DIR}/{}", Self::FILENAME);
        let output_dir = format!("{WORKSPACE_ROOT}/{OUTPUT_DIR}");
        let output_path = format!("{output_dir}/{}", Self::FILENAME);
        fs::create_dir_all(&output_dir)?;
        let template = fs::read_to_string(&template_path)?;
        let output = template
            .replace("{{ progress }}", &self.progress)
            .replace("{{ lang_list }}", &self.lang_list);
        fs::write(&output_path, output)?;
        Ok(())
    }
}

pub struct OverviewImage {
    pub name: String,
    pub stars: u64,
    pub forks: u64,
    pub contributions: u64,
    pub followers: u64,
    pub views: u64,
    pub repos: u64,
}

impl OverviewImage {
    const FILENAME: &str = "overview.svg";

    pub fn new(stats: &Statistics) -> Self {
        Self {
            name: stats.name.clone(),
            stars: stats.stars,
            forks: stats.forks,
            contributions: stats.contributions,
            followers: stats.followers,
            views: stats.views,
            repos: stats.repos,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn fmt_number(num: u64) -> String {
        match num {
            1_000_000_000_000.. => format!("{:.3}T", num as f64 / 1_000_000_000_000.0),
            1_000_000_000.. => format!("{:.3}B", num as f64 / 1_000_000_000.0),
            1_000_000.. => format!("{:.3}M", num as f64 / 1_000_000.0),
            1_000.. => format!("{:.3}K", num as f64 / 1_000.0),
            _ => num.to_string(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let template_path = format!("{WORKSPACE_ROOT}/{TEMPLATE_DIR}/{}", Self::FILENAME);
        let output_dir = format!("{WORKSPACE_ROOT}/{OUTPUT_DIR}");
        let output_path = format!("{output_dir}/{}", Self::FILENAME);
        fs::create_dir_all(&output_dir)?;
        let template = fs::read_to_string(&template_path)?;
        let output = template
            .replace("{{ name }}", &self.name)
            .replace("{{ stars }}", &Self::fmt_number(self.stars))
            .replace("{{ forks }}", &Self::fmt_number(self.forks))
            .replace("{{ contributions }}", &Self::fmt_number(self.contributions))
            .replace("{{ followers }}", &Self::fmt_number(self.followers))
            .replace("{{ views }}", &Self::fmt_number(self.views))
            .replace("{{ repos }}", &Self::fmt_number(self.repos));
        fs::write(&output_path, output)?;
        Ok(())
    }
}
