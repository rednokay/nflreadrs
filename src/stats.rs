use crate::downloader::Downloader;
use crate::utils;
use anyhow::Result;
use reqwest::blocking;
use std::default::Default;
use strum::Display;
use url::Url;

/// Summary levels describing the scope of the data.
#[derive(Debug, Display)]
pub enum SummaryLevel {
    Week,
    Reg,
    Post,
    RegPost,
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
        let summary = self.summary_level.to_string().to_lowercase();

        let seasons = match &self.seasons {
            None => utils::get_current_season(None),
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

/// Downloader for schedules.
#[derive(Debug)]
pub struct Schedules {
    base_url: &'static str,
    client: blocking::Client,
}

impl Schedules {
    /// Create a new schedules downloader.
    ///
    /// This method is used to construct a downloader for schedules using the Default trait.
    /// The source does not provide any seasons or summary levels, all available schedules will be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::Schedules;
    ///
    /// let schedules_dl = Schedules::new();
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(schedules_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/schedules/games.csv").unwrap())
    /// ```
    pub fn new() -> Self {
        Schedules::default()
    }
}

impl Default for Schedules {
    // Default constructor for schedules downloader.
    fn default() -> Self {
        Self {
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/schedules/games.csv",
            client: blocking::Client::new(),
        }
    }
}

impl Downloader for Schedules {
    /// Returns a valid URL to the download destination.
    ///
    /// Here the download URL is the base url as the source does not provide seasons or summary levels
    fn url(&self) -> Result<Url> {
        Ok(Url::parse(self.base_url)?)
    }

    /// Return blocking client.
    fn client(&self) -> &blocking::Client {
        &self.client
    }
}

/// Downloader for play by play data.
#[derive(Debug)]
pub struct PlayByPlay {
    seasons: Option<i32>,
    base_url: &'static str,
    client: blocking::Client,
}

impl PlayByPlay {
    /// Create a new team play by play data downloader.
    ///
    /// This method is used to construct a downloader for play by play data.
    ///
    /// # Arguments
    ///
    /// * `seasons` -   Current season if None. Given season if Some.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::PlayByPlay;
    ///
    /// let seasons: Option<i32> = Some(2025);
    ///
    /// let play_by_play_dl = PlayByPlay::new(seasons);
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(play_by_play_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/pbp/play_by_play_2025.csv").unwrap())
    /// ```
    pub fn new(seasons: Option<i32>) -> Self {
        Self {
            seasons,
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/pbp/",
            client: blocking::Client::new(),
        }
    }
}

impl Downloader for PlayByPlay {
    /// Returns a valid URL to the download destination.
    fn url(&self) -> Result<Url> {
        let seasons = self.seasons.unwrap_or(utils::get_current_season(None));

        let url = format!("{}play_by_play_{}.csv", self.base_url, seasons);

        Ok(Url::parse(&url)?)
    }

    /// Return blocking client.
    fn client(&self) -> &blocking::Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod team_stats_downloader_tests {
        use super::*;

        #[test]
        fn test_correct_url_various_seasons_and_summary_levels() {
            let cases = [
                // (summary level, season, expected url ending)
                (SummaryLevel::Post, 2035, "post_2035"),
                (SummaryLevel::Reg, 2005, "reg_2005"),
                (SummaryLevel::Week, 2017, "week_2017"),
                (SummaryLevel::RegPost, 2011, "regpost_2011"),
            ];
            let base = "https://github.com/nflverse/nflverse-data/releases/download/stats_team/stats_team_";

            for (sum_lvl, season, exp) in cases {
                let team_stats = TeamStats::new(Some(vec![season]), sum_lvl);
                let expected_url = Url::parse(&format!("{}{}.csv", base, exp)).unwrap();
                assert_eq!(team_stats.url().unwrap(), expected_url);
            }
        }

        #[test]
        fn test_correct_url_seasons_none() {
            let base = "https://github.com/nflverse/nflverse-data/releases/download/stats_team/stats_team_";
            let team_stats = TeamStats::new(None, SummaryLevel::RegPost);
            let expected_url = Url::parse(&format!(
                "{}regpost_{}.csv",
                base,
                utils::get_current_season(None)
            ))
            .unwrap();
            assert_eq!(team_stats.url().unwrap(), expected_url);
        }

        // TODO: This behavior will be changed
        #[test]
        fn test_correct_url_season_vec() {
            let team_stats = TeamStats::new(Some(vec![2000, 2012]), SummaryLevel::Post);
            let url = team_stats.url();
            assert!(url.is_err());
        }
    }

    mod play_by_play_downloader_tests {
        use super::*;

        #[test]
        fn test_correct_url_various_seasons() {
            let cases = [(2025, "2025.csv"), (2006, "2006.csv")];

            for (season, exp) in cases {
                let play_by_play = PlayByPlay::new(Some(season));
                let expected =
                    Url::parse(&format!("{}play_by_play_{}", play_by_play.base_url, exp)).unwrap();
                assert_eq!(play_by_play.url().unwrap(), expected);
            }
        }

        #[test]
        fn test_correct_url_seasons_none() {
            let base = "https://github.com/nflverse/nflverse-data/releases/download/pbp/";
            let play_by_play = PlayByPlay::new(None);
            let expected_url = Url::parse(&format!(
                "{}play_by_play_{}.csv",
                base,
                utils::get_current_season(None)
            ))
            .unwrap();
            assert_eq!(play_by_play.url().unwrap(), expected_url);
        }
    }
}
