//! Generate configuration for a wanted download.
use crate::downloader::Downloader;
use crate::utils::{self, get_current_season};
use anyhow::Result;
use std::default::Default;
use strum::Display;
use url::Url;

/// Summary levels describing the scope of the data.
///
/// Some stat downloaders need a specified scope.
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
}

/// Downloader for schedules.
#[derive(Debug)]
pub struct Schedules {
    base_url: &'static str,
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
}

/// Downloader for play by play data.
#[derive(Debug)]
pub struct PlayByPlay {
    seasons: Option<i32>,
    base_url: &'static str,
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
}

/// Downloader for player stats.
#[derive(Debug)]
pub struct PlayerStats {
    seasons: Option<i32>,
    summary_level: SummaryLevel,
    base_url: &'static str,
}

impl PlayerStats {
    /// Create a new player stats downloader.
    ///
    /// This method is used to construct a downloader for teams stats.
    ///
    /// # Arguments
    ///
    /// * `seasons` -   Current season if None. Given season if Some.
    /// * `summary_level`   -   Summary level of the data to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::{SummaryLevel, PlayerStats};
    ///
    /// let seasons: Option<i32> = Some(2025);
    ///
    /// let player_stats_dl = PlayerStats::new(seasons, SummaryLevel::Reg);
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(player_stats_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/stats_player/stats_player_reg_2025.csv").unwrap())
    /// ```
    pub fn new(seasons: Option<i32>, summary_level: SummaryLevel) -> Self {
        Self {
            seasons,
            summary_level,
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/stats_player/",
        }
    }
}

impl Downloader for PlayerStats {
    /// Returns a valid URL to the download destination.
    fn url(&self) -> Result<Url> {
        let summary = self.summary_level.to_string().to_lowercase();

        let seasons = self.seasons.unwrap_or(get_current_season(None));
        let url = format!("{}stats_player_{}_{}.csv", self.base_url, summary, seasons);

        Ok(Url::parse(&url)?)
    }
}

/// Downloader for teams.
#[derive(Debug)]
pub struct Teams {
    base_url: &'static str,
}

impl Teams {
    /// Create a new teams downloader.
    ///
    /// This method is used to construct a downloader for teams using the Default trait.
    /// The source does not provide any seasons or summary levels, all available teams will be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::Teams;
    ///
    /// let teams_dl = Teams::new();
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(teams_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/teams/teams_colors_logos.csv").unwrap())
    /// ```
    pub fn new() -> Self {
        Teams::default()
    }
}

impl Default for Teams {
    // Default constructor for Teams downloader.
    fn default() -> Self {
        Self {
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/teams/teams_colors_logos.csv",
        }
    }
}

impl Downloader for Teams {
    /// Returns a valid URL to the download destination.
    ///
    /// Here the download URL is the base url as the source does not provide seasons or summary levels
    fn url(&self) -> Result<Url> {
        Ok(Url::parse(self.base_url)?)
    }
}

/// Downloader for Players.
#[derive(Debug)]
pub struct Players {
    base_url: &'static str,
}

impl Players {
    /// Create a new Players downloader.
    ///
    /// This method is used to construct a downloader for players using the Default trait.
    /// The source does not provide any seasons or summary levels, all available players will be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use nflreadrs::stats::Players;
    ///
    /// let players_dl = Players::new();
    ///
    /// # use url::Url;
    /// # use nflreadrs::downloader::Downloader;
    /// # assert_eq!(players_dl.url().unwrap(), Url::parse("https://github.com/nflverse/nflverse-data/releases/download/players/players.csv").unwrap())
    /// ```
    pub fn new() -> Self {
        Players::default()
    }
}

impl Default for Players {
    // Default constructor for Players downloader.
    fn default() -> Self {
        Self {
            base_url: "https://github.com/nflverse/nflverse-data/releases/download/players/players.csv",
        }
    }
}

impl Downloader for Players {
    /// Returns a valid URL to the download destination.
    ///
    /// Here the download URL is the base url as the source does not provide seasons or summary levels
    fn url(&self) -> Result<Url> {
        Ok(Url::parse(self.base_url)?)
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

    mod player_stats_downloader_tests {
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
            let base = "https://github.com/nflverse/nflverse-data/releases/download/stats_player/stats_player_";

            for (sum_lvl, season, exp) in cases {
                let team_stats = PlayerStats::new(Some(season), sum_lvl);
                let expected_url = Url::parse(&format!("{}{}.csv", base, exp)).unwrap();
                assert_eq!(team_stats.url().unwrap(), expected_url);
            }
        }

        #[test]
        fn test_correct_url_seasons_none() {
            let base = "https://github.com/nflverse/nflverse-data/releases/download/stats_player/stats_player_";
            let team_stats = PlayerStats::new(None, SummaryLevel::RegPost);
            let expected_url = Url::parse(&format!(
                "{}regpost_{}.csv",
                base,
                utils::get_current_season(None)
            ))
            .unwrap();
            assert_eq!(team_stats.url().unwrap(), expected_url);
        }
    }
}
