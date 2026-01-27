use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use log::{debug, error, info, trace, warn};
use octocrab::models;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
struct Traffic {
    pub count: u32,
    pub uniques: u32,
}

#[derive(Serialize)]
struct TrafficParams<'a> {
    per: &'a str,
}

pub struct Statistics {
    pub name: String,
    pub followers: u64,
    pub stars: u64,
    pub forks: u64,
    pub contributions: u64,
    // pub lines_changed: u64,
    pub views: u64,
    pub repos: u64,
    pub langs: Vec<(String, f64)>,
}

impl Statistics {
    /// Number of decimal digits for language percentage
    const PRECISION_DIGITS: i32 = 4;

    #[allow(clippy::too_many_lines)]
    pub async fn fetch(config: &Config) -> Result<Self> {
        let client = octocrab::instance().user_access_token(config.access_token.clone())?;
        let user = client.current().user().await?;

        let followers = client
            .get::<Vec<models::Author>, _, _>(&user.followers_url, None::<&()>)
            .await?
            .len() as u64;

        let mut stars = 0;
        let mut forks = 0;
        let mut contributions = 0;
        // let mut lines_changed = 0;
        let mut views = 0;
        let mut repos_count = 0;
        let mut langs = HashMap::new();
        let page = client
            .current()
            .list_repos_for_authenticated_user()
            .send()
            .await?;
        let repos = client.all_pages(page).await?;
        // Throttle to avoid hitting rate limits
        let mut repos =
            Box::pin(tokio_stream::iter(&repos).throttle(Duration::from_millis(config.delay_ms)));
        while let Some(repo) = repos.next().await {
            let Some(name) = &repo.full_name else {
                error!("Repository without a name found, skipping...");
                continue;
            };
            if config.exclude_repos.contains(name) {
                debug!("{name} is excluded, skipping...");
                continue;
            }

            let &Some(size) = &repo.size else {
                warn!("{name} has no size information, skipping...");
                continue;
            };
            if size == 0 {
                info!("{name} is empty, skipping...");
                continue;
            }

            if config.exclude_forks {
                match repo.fork {
                    Some(true) => {
                        debug!("{name} is a fork, skipping...");
                        continue;
                    }
                    None => {
                        warn!("{name} has no fork information, ignoring...");
                    }
                    _ => {}
                }
            }

            if config.exclude_private {
                match repo.private {
                    Some(true) => {
                        debug!("{name} is private, skipping...");
                        continue;
                    }
                    None => {
                        warn!("{name} has no private information, ignoring...");
                    }
                    _ => {}
                }
            }

            if config.exclude_archive {
                match repo.archived {
                    Some(true) => {
                        debug!("{name} is archived, skipping...");
                        continue;
                    }
                    None => {
                        warn!("{name} has no archive information, ignoring...");
                    }
                    _ => {}
                }
            }

            let contributors = client
                .repos_by_id(repo.id)
                .list_contributors()
                .send()
                .await?;
            let contributors = client.all_pages(contributors).await?;
            if let Some(contributor) = contributors.iter().find(|c| c.author.login == user.login) {
                trace!(
                    "{name}: {} contributions by user",
                    contributor.contributions
                );
                contributions += u64::from(contributor.contributions);
            } else {
                info!("{name} has no contributions by {}, skipping...", user.login);
                continue;
            }

            repos_count += 1;

            match repo.forks_count {
                Some(fork_count) => {
                    trace!("{name}: {fork_count} forks");
                    forks += u64::from(fork_count);
                }
                None => warn!("Failed to get fork count for {name}, skipping..."),
            }

            match repo.stargazers_count {
                Some(star_count) => {
                    trace!("{name}: {star_count} stars");
                    stars += u64::from(star_count);
                }
                None => warn!("Failed to get star count for {name}, skipping..."),
            }

            match client
                .get::<Traffic, _, _>(
                    &format!("https://api.github.com/repos/{name}/traffic/views"),
                    Some(&TrafficParams { per: "week" }),
                )
                .await
            {
                Ok(traffic) => {
                    trace!("{name}: {traffic:?}");
                    views += u64::from(traffic.uniques);
                }
                Err(e) => {
                    error!("Error when fetching traffic views for {name}: {e}");
                    warn!("Failed to get traffic views for {name}, ignoring...");
                }
            }

            let Some(langs_url) = &repo.languages_url else {
                warn!("{name} has no languages URL, ignoring...");
                continue;
            };
            client
                .get::<models::repos::Languages, _, _>(langs_url, None::<&()>)
                .await?
                .into_iter()
                .for_each(|(lang, bytes)| {
                    if config.exclude_langs.contains(&lang.to_lowercase()) {
                        debug!("{lang} in {name} is excluded, skipping...");
                        return;
                    }
                    trace!("{name}: {bytes} bytes in {lang}");

                    langs
                        .entry(lang)
                        .and_modify(|cnt| *cnt += bytes.unsigned_abs())
                        .or_insert_with(|| bytes.unsigned_abs());
                });
        }

        let mut langs = langs.into_iter().collect::<Vec<_>>();
        langs.sort_unstable_by_key(|&(_, size)| std::cmp::Reverse(size));
        #[allow(clippy::cast_precision_loss)]
        let total = langs.iter().map(|&(_, size)| size).sum::<u64>() as f64;
        #[allow(clippy::cast_precision_loss)]
        let langs = langs
            .into_iter()
            .map(|(lang, size)| {
                let percent = (size as f64 / total) * 100.0;
                let base = 10.0_f64.powi(Self::PRECISION_DIGITS);
                (lang, (percent * base).round() / base)
            })
            .collect::<_>();

        Ok(Self {
            name: user.name.unwrap_or(user.login),
            followers,
            stars,
            forks,
            contributions,
            views,
            repos: repos_count,
            langs,
        })
    }
}
