use std::fs;

use anyhow::Result;
use maud::{Markup, html};

use crate::{color::Color, icons, stat::Statistics};

const WORKSPACE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const OUTPUT_DIR: &str = "generated";

pub struct LanguageImage(Markup);

impl LanguageImage {
    const FILENAME: &str = "languages.svg";
    const ANIMATION_DELAY_MS: u64 = 100;

    pub fn new(color: &Color, stats: &Statistics) -> Self {
        let (style_sheet, class_name) = turf::style_sheet_values!("styles/languages.scss");

        let progress = html! {
            @for (lang, percent) in &stats.langs {
                @let color = color.get_color(lang);
                span
                    .(class_name.progress_item)
                    style={ "background-color:" (color) ";width:" (percent) "%" } {}
            }
        };
        let lang_list = html! {
            ul {
                @for (i, (lang, percent)) in stats.langs.iter().enumerate() {
                    @let color = color.get_color(lang);
                    @let delay = i as u64 * Self::ANIMATION_DELAY_MS;
                    @let style = format!("fill:{color}");
                    li style={ "animation-delay:" (delay) "ms" } {
                        ({
                            icons::DOT
                                .with_style(style)
                                .with_class(class_name.octicon)
                                .finalize()
                        })
                        span.(class_name.lang) { (lang) }
                        span.(class_name.percent) { (format!("{:.2}%", percent)) }
                    }
                }
            }
        };
        let svg = html! {
            svg #gh-dark-mode-only xmlns="http://www.w3.org/2000/svg" width="360" height="210" {
                style { (style_sheet) }
                g {
                    rect #background x="5" y="5" {}
                    g {
                        foreignObject x="21" y="17" width="318" height="176" {
                            .(class_name.ellipsis) xmlns="http://www.w3.org/1999/xhtml" {
                                h2 { "Languages Used (By File Size)" }
                                {
                                    span.(class_name.progress) { (progress) }
                                }
                                (lang_list)
                            }
                        }
                    }
                }
            }
        };

        Self(svg)
    }

    pub fn save(self) -> Result<()> {
        let output_dir = format!("{WORKSPACE_ROOT}/{OUTPUT_DIR}");
        let output_path = format!("{output_dir}/{}", Self::FILENAME);
        fs::create_dir_all(&output_dir)?;
        fs::write(&output_path, self.0.into_string())?;
        Ok(())
    }
}

pub struct OverviewImage(Markup);

impl OverviewImage {
    const FILENAME: &str = "overview.svg";
    const ANIMATION_DELAY_MS: u64 = 100;

    pub fn new(stats: &Statistics) -> Self {
        let entries = [
            ("Stars", icons::STAR, stats.stars),
            ("Forks", icons::REPO_FORKED, stats.forks),
            ("Followers", icons::PERSON, stats.followers),
            (
                "All-time contributions",
                icons::REPO_PUSH,
                stats.contributions,
            ),
            ("Repository views (past two weeks)", icons::EYE, stats.views),
            ("Repositories with contributions", icons::REPO, stats.repos),
        ];

        let (style_sheet, style_class) = turf::style_sheet_values!("styles/overview.scss");
        let table = html! {
            table {
                thead {
                    tr style="transform: translateX(0)" {
                        th colspan="2" { (stats.name) "'s GitHub Statistics" }
                    }
                }
                tbody {
                    @for (i, (label, icon, value)) in entries.into_iter().enumerate() {
                        @let delay = i as u64 * Self::ANIMATION_DELAY_MS;
                        tr style={ "animation-delay:" (delay) "ms" } {
                            td { (icon.with_class(style_class.octicon).finalize()) (label) }
                            td { (Self::fmt_number(value)) }
                        }
                    }
                }
            }
        };
        let svg = html! {
            svg #gh-dark-mode-only xmlns="http://www.w3.org/2000/svg" width="360" height="210" {
                style { (style_sheet) }
                g {
                    rect #background x="5" y="5" {}
                    g {
                        foreignObject x="21" y="21" width="318" height="168" {
                            div xmlns="http://www.w3.org/1999/xhtml" { (table) }
                        }
                    }
                }
            }
        };
        Self(svg)
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

    pub fn save(self) -> Result<()> {
        let output_dir = format!("{WORKSPACE_ROOT}/{OUTPUT_DIR}");
        let output_path = format!("{output_dir}/{}", Self::FILENAME);
        fs::create_dir_all(&output_dir)?;
        fs::write(&output_path, self.0.into_string())?;
        Ok(())
    }
}
