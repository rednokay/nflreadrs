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
    ///
    /// This method is used to construct a downloader for teams stats.
    ///
    /// # Arguments
    ///
    /// * `seasons` -   Current season if None. A vector of the desired season if Some.
    /// * `summary_level`   -   Summary level of the data to retrieve.
    ///
    /// # Panics
    ///
    /// Panics if a vector of length greater than one is passed. These vectors are not supported yet.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::{SummaryLevel, TeamStats};
    ///
    /// let seasons: Option<Vec<i32>> = Some(vec![2025]);
    ///
    /// let team_stats_dl = TeamStats::new(seasons, SummaryLevel::Reg);
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(team_stats_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/stats_team/stats_team_reg_2025.csv").unwrap())
    /// ```
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
    /// Returns a valid URL to the download destination.
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

    /// Return blocking client.
    fn client(&self) -> &blocking::Client {
        &self.client
    }
}
