# Nflreadrs

> [!NOTE]
> This crate is not yet fully functional!

Nflreadrs is a port of [nflverse's](https://nflverse.nflverse.com/)
[nflreadpy](https://github.com/nflverse/nflreadpy) (Python) and
[nflreadr](https://github.com/nflverse/nflreadr) \(R)
libraries to the Rust programming language.

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
