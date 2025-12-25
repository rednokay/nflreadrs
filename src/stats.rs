use crate::downloader::Downloader;
use anyhow::Result;
use reqwest::blocking;
use strum::Display;
use url::Url;

/// Summary levels describing the scope of the data.
#[derive(Debug, Display)]
pub enum SummaryLevel {
    Week,
    Reg,
    Post,
    RegNPost,
}

/// Downloader for team stats.
#[derive(Debug)]
pub struct TeamStats {
    seasons: Option<Vec<i32>>,
    summary_level: SummaryLevel,
    base_url: &'static str,
    client: blocking::Client,
}

impl TeamStats {
    /// Create a new team stats downloader.
    pub fn new(seasons: Option<Vec<i32>>, summary_level: SummaryLevel) -> Self {
        Self {
            seasons,
            summary_level,
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/stats_team/",
            client: blocking::Client::new(),
        }
    }
}

impl Downloader for TeamStats {
    fn url(&self) -> Result<Url> {
        // TODO: handle RegNPost probalby using strum
        let summary = self.summary_level.to_string().to_lowercase();

        // TODO: Not hardcode year
        let seasons = match &self.seasons {
            None => 2025,
            Some(v) => match v.len() {
                1 => v[0],
                _ => anyhow::bail!("Unhandled season case {:?}", self.seasons),
            },
        };

        let url = format!("{}stats_team_{}_{}.csv", self.base_url, summary, seasons);

        Ok(Url::parse(&url)?)
    }

    fn client(&self) -> &blocking::Client {
        &self.client
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     todo!();
// }
