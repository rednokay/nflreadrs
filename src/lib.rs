//! Crate to pull data from [nflverse's](https://nflverse.nflverse.com/) NFL database.
//!
//! This project is heavily inspired by the original modules by nflverse written in [R](https://github.com/nflverse/nflreadr) and
//! [Python](https://github.com/nflverse/nflreadpy). Currently, we do not support the complete scope of the references due to this crate
//! being work in process.
pub mod downloader;
pub mod stats;
pub mod utils;
