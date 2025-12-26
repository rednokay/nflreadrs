//! Download data with specified configuration.
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
fn from_csv(path: PathBuf, infer_rows: Option<usize>) -> Result<DataFrame> {
    Ok(CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(infer_rows)
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
///
/// This fetches the desired data by downloading it into the temporary directory,
/// loads it into memory and returns it as a polars::DataFrame.
///
/// # Arguments
///
/// * `downloader`  -   The struct relating to the desired stats. Needs to implement Downloader.
pub fn pull<D>(downloader: &D) -> Result<DataFrame>
where
    D: Downloader,
{
    let path_to_file = fetch_content(downloader)?;
    from_csv(path_to_file, None)
}
