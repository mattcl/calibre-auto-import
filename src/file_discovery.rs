use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use snafu::{ResultExt, Snafu};
use walkdir::{DirEntry, WalkDir};

use crate::marker::Marker;

#[derive(Debug, Snafu)]
pub enum DiscoverError {
    FetchEntry { source: walkdir::Error },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FileDiscoverer<'a> {
    search_dir: &'a Path,
    marker: Marker,
}

impl<'a> FileDiscoverer<'a> {
    pub fn new(search_dir: &'a Path, marker: Marker) -> Self {
        Self { search_dir, marker }
    }

    pub fn discover(&self) -> Result<Vec<FileInfo>, DiscoverError> {
        let mut out = Vec::default();
        let walker = WalkDir::new(self.search_dir).into_iter();
        for entry in walker.filter_entry(|e| newer_than(&self.marker, e)) {
            let entry = entry.context(FetchEntrySnafu)?;
            if entry.path().is_dir() {
                continue;
            }

            out.push(FileInfo {
                path: entry.path().to_path_buf(),
                name: entry.file_name().to_string_lossy().into_owned(),
            });
        }

        Ok(out)
    }
}

fn newer_than(marker: &Marker, entry: &DirEntry) -> bool {
    // no directories, no files that aren't epub
    if entry
        .metadata()
        .map(|metadata| metadata.is_dir())
        .unwrap_or_default()
    {
        return true;
    }

    if let Some(cutoff) = marker.cutoff_time {
        entry
            .metadata()
            .inspect_err(|error| {
                tracing::warn!(
                    ?error,
                    path = entry.path().to_string_lossy().to_string(),
                    "could not get metadata"
                )
            })
            .map(|meta| {
                meta.modified()
                    .inspect_err(|error| {
                        tracing::warn!(
                            ?error,
                            path = entry.path().to_string_lossy().to_string(),
                            "could not get modified"
                        )
                    })
                    .map(|modified| {
                        let modified_t: DateTime<Utc> = DateTime::from(modified);
                        modified_t > cutoff
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    } else {
        // it we didn't have a cutoff, always true
        true
    }
}
