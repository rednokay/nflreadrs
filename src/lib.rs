pub mod downloader {
    //! From web to DataFrame.

    use anyhow::Result;
    use polars::prelude::*;
    use reqwest::blocking;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;
    use url::Url;
    use uuid::Uuid;

    /// Trait that is implemented to download wanted stats.
    pub trait Downloader {
        /// Returns a temporary directory to download into.
        fn tmp_dir(&self) -> PathBuf {
            env::temp_dir()
        }

        /// Returns the URL path for this downloader.
        fn url(&self) -> Result<Url>;

        /// Returns the client for the downloader.
        fn client(&self) -> &blocking::Client;
    }

    /// Reads a downloaded CSV file to DataFrame.
    fn from_csv(path: PathBuf) -> Result<DataFrame> {
        Ok(CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path))?
            .finish()?)
    }

    /// Download the CSV file.
    ///
    /// Downloads the CSV file with the wanted data into a temporary directory.
    fn fetch_content<D>(downloader: &D) -> Result<PathBuf>
    where
        D: Downloader,
    {
        let mut response = downloader.client().get(downloader.url()?).send()?;

        let mut path = downloader.tmp_dir();
        let id = Uuid::new_v4().to_string();
        path.push(format!("nflreadrs-{}.csv", &id));

        let mut file = File::create(&path)?;

        response.copy_to(&mut file)?;

        Ok(path)
    }

    /// Called on a Downloader to pull the data to a DataFrame.
    pub fn pull<D>(downloader: &D) -> Result<DataFrame>
    where
        D: Downloader,
    {
        let path_to_file = fetch_content(downloader)?;
        from_csv(path_to_file)
    }
}

pub mod stats {
    use super::downloader::Downloader;
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     todo!();
// }
