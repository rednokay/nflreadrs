# Nflreadrs

[![Crates.io Version](https://img.shields.io/crates/v/nflreadrs.svg)](https://crates.io/crates/nflreadrs)
[![License](https://img.shields.io/crates/l/nflreadrs.svg)](https://crates.io/crates/nflreadrs)
[![Docs.rs](https://img.shields.io/docsrs/nflreadrs/latest)](https://docs.rs/nflreadrs)


> [!NOTE]
> This crate is not yet fully functional!

Nflreadrs is a port of [nflverse's](https://nflverse.nflverse.com/)
[nflreadpy](https://github.com/nflverse/nflreadpy) (Python) and
[nflreadr](https://github.com/nflverse/nflreadr) \(R)
libraries to the Rust programming language. Other than liking their work,
I am not affiliated with them in any way, shape or form.


## Installation

Use cargo to add the crate to your project.

```bash
cargo add nflreadrs
```

## Quick Start

This is a simple setup to download team stats of the current season.
The resulting data will be a Polars DataFrame.

```rust
use nflreadrs::downloader::pull;
use nflreadrs::stats::{SummaryLevel, TeamStats};

fn main() {
    // Download settings
    let teams_stats_dl = TeamStats::new(None, SummaryLevel::Week);

    let data = pull(&teams_stats_dl);
}
```


## Major TODOs

* Improve errors with thiserror
* Support for parquet
* Caching/using already downloaded files when feasible
