use std::{collections::HashSet, env};

#[derive(Clone, Debug)]
pub struct Config {
    /// GitHub Personal Access Token
    pub access_token: String,
    /// Repositories to exclude (case sensitive)
    ///
    /// Should be in the format "owner/repo"
    pub exclude_repos: HashSet<String>,
    /// Languages to exclude (case insensitive)
    ///
    /// This will match precisely, so "Jupyter Notebook" will not match
    /// "Jupyter". Use names as it appears in the output, or check
    /// <https://github.com/ozh/github-colors/blob/master/colors.json> for reference.
    pub exclude_langs: HashSet<String>,
    /// Whether to exclude forked repositories
    pub exclude_forks: bool,
    /// Whether to exclude private repositories
    pub exclude_private: bool,
    /// Whether to exclude archive repositories
    pub exclude_archive: bool,
    /// Log level (0-5, default: 2)
    ///
    /// - 0 - Off
    /// - 1 - Error
    /// - 2 - Warn
    /// - 3 - Info
    /// - 4 - Debug
    /// - 5 - Trace
    pub log_level: u8,
    /// Delay between requests in milliseconds
    ///
    /// Default is 1000 ms, used to avoid hitting rate limits
    pub delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        let access_token = env::var("ACCESS_TOKEN").unwrap_or_else(|_| "Not set".to_string());
        let exclude_repos = env::var("EXCLUDE_REPOS")
            .unwrap_or_else(|_| String::new())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let exclude_langs = env::var("EXCLUDE_LANGS")
            .unwrap_or_else(|_| String::new())
            .split(',')
            .map(|s| s.trim().to_string().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        let exclude_forks = env::var("EXCLUDE_FORKS")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let exclude_private = env::var("EXCLUDE_PRIVATE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let exclude_archive = env::var("EXCLUDE_ARCHIVE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u8>()
            .unwrap_or(2);
        let delay_ms = env::var("DELAY_MS")
            .unwrap_or_else(|_| "1000".to_string())
            .parse::<u64>()
            .unwrap_or(1000);
        Self {
            access_token,
            exclude_repos,
            exclude_langs,
            exclude_forks,
            exclude_private,
            exclude_archive,
            log_level,
            delay_ms,
        }
    }
}
