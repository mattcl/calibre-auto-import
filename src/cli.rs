use std::path::PathBuf;

use clap::Parser;

/// Copies epup files form one location to another.
///
/// Will only copy files that were created after a persisted timestamp.
#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(short, long, value_name = "DIR", env = "CAI_MARKER_DIR")]
    pub marker_dir: PathBuf,

    #[arg(
        long,
        value_name = "NAME",
        env = "CAI_MARKER_FILE_NAME",
        default_value = "cai_marker.json"
    )]
    pub marker_file_name: String,

    #[arg(short, long, value_name = "DIR", env = "CAI_WATCH_DIR")]
    pub watch_dir: PathBuf,

    #[arg(short, long, value_name = "DIR", env = "CAI_OUTPUT_DIR")]
    pub output_dir: PathBuf,

    #[arg(long)]
    pub dry_run: bool,
}

impl Args {
    pub fn marker_path(&self) -> PathBuf {
        self.marker_dir.join(&self.marker_file_name)
    }
}
