use std::collections::HashMap;

use anyhow::Result;

use crate::config::Config;

pub struct Color(pub HashMap<String, String>);

impl Color {
    pub async fn fetch(config: &Config) -> Result<Self> {
        let client = octocrab::instance().user_access_token(config.access_token.clone())?;
        let colors = client
            .get::<HashMap<String, HashMap<String, Option<String>>>, _, _>(
                "https://raw.githubusercontent.com/ozh/github-colors/master/colors.json",
                None::<&()>,
            )
            .await?;
        let colors = colors
            .into_iter()
            .filter_map(|(lang, data)| match data.get("color") {
                Some(Some(color)) => Some((lang, color.clone())),
                _ => None,
            })
            .collect();
        Ok(Self(colors))
    }

    pub fn get_color<S: AsRef<str>>(&self, lang: S) -> &str {
        self.0.get(lang.as_ref()).map_or("#000000", String::as_str)
    }
}
