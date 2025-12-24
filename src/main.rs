use std::collections::HashSet;

use chrono::Utc;
use clap::Parser as _;
use snafu::{ResultExt, Snafu};

mod cli;
mod file_discovery;
mod marker;

use crate::{
    file_discovery::{DiscoverError, FileDiscoverer},
    marker::{Marker, MarkerError},
};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(transparent)]
    Marker {
        source: MarkerError,
    },
    #[snafu(transparent)]
    Discover {
        source: DiscoverError,
    },
    Copy {
        source: std::io::Error,
    },
}

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();

    tracing_subscriber::fmt().init();

    let marker_path = args.marker_path();
    let marker = Marker::from_file(&marker_path)
        .inspect_err(|error| tracing::warn!(?error, "Could not get marker from file"))
        .unwrap_or_default();

    let discoverer = FileDiscoverer::new(&args.watch_dir, marker);

    let candidates = discoverer.discover()?;

    if !candidates.is_empty() {
        tracing::info!(num = candidates.len(), "found files to copy");
        let mut seen = HashSet::with_capacity(candidates.len());

        for candidate in candidates {
            // if we already wrote a file that would have the same name, skip
            // for now. This will be fine since readarr should be renaming files
            // with author names anyway
            if seen.insert(candidate.name.clone()) {
                // copy this to the output dir
                let output_path = args.output_dir.join(&candidate.name);
                if args.dry_run {
                    tracing::warn!(name = candidate.name, "dry run would copy");
                } else {
                    std::fs::copy(&candidate.path, &output_path).context(CopySnafu)?;
                    tracing::info!(name = candidate.name, "copied");
                }
            } else {
                tracing::warn!(
                    name = candidate.name,
                    "skipping what would be a duplicate file name"
                );
            }
        }

        if !args.dry_run {
            // persist a new marker
            tracing::info!("writing new marker");
            let marker = Marker::from(Utc::now());
            marker.write_to_file(&marker_path)?;
        }
    } else {
        tracing::info!("no newer files to copy");
    }

    Ok(())
}
