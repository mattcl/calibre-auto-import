use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum MarkerError {
    FileOpen { source: std::io::Error },
    FileCreate { source: std::io::Error },
    InvalidFormat { source: serde_json::Error },
    Serialization { source: serde_json::Error },
    Write { source: std::io::Error },
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Marker {
    pub cutoff_time: Option<DateTime<Utc>>,
}

impl Marker {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, MarkerError> {
        let file = File::open(path.as_ref()).context(FileOpenSnafu)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).context(InvalidFormatSnafu)
    }

    pub fn write_to_file<T: AsRef<Path>>(&self, path: T) -> Result<(), MarkerError> {
        let mut file = File::create(path.as_ref()).context(FileCreateSnafu)?;
        file.write_all(
            serde_json::to_string(&self)
                .context(SerializationSnafu)?
                .as_bytes(),
        )
        .context(WriteSnafu)?;

        Ok(())
    }
}

impl From<DateTime<Utc>> for Marker {
    fn from(value: DateTime<Utc>) -> Self {
        Self {
            cutoff_time: Some(value),
        }
    }
}

impl TryFrom<&Path> for Marker {
    type Error = MarkerError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::from_file(value)
    }
}

impl TryFrom<PathBuf> for Marker {
    type Error = MarkerError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::from_file(value)
    }
}
