use clap::Parser;
use std::path::{Path, PathBuf};

/// About Media Organizer
/// asdsdfsdsfsdf
/// sdgergergergergerrge
#[derive(Parser, Debug)]
#[clap(
    author = "Mikko Leppänen <mleppan23@gmail.com>",
    version = "0.1",
    about
)]
pub struct Args {
    /// Destination path
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub dest: PathBuf,
    /// Target path
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub target: PathBuf,
    /// How to organize images: possible options are: "year" | "month" | "flat"
    #[arg(short, long, default_value = "year")]
    pub style: String,
    /// Do you want to keep videos in separate folder?
    #[arg(short, long, default_value_t = true)]
    pub separate_videos: bool,
    /// Find files recursively
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,
    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// Include some additional image format(s) as a comma separated list: "jpeg,png"
    #[arg(short, long, default_value = "")]
    pub include_image_format: String,
    /// Include some extra format not
    #[arg(short, long, default_value = "")]
    pub include_video_format: String,
    /// Move content to target instead of copying
    #[arg(short, long, default_value_t = false)]
    pub use_move: bool,
}

fn validate_paths(paths: [&PathBuf; 2]) {
    let _ = paths.iter().map(|&p| {
        if !Path::new(p).exists() {
            panic!("Given path does not exist: {:?}", p)
        }
    });
}

pub fn handle_command_line_arguments() -> Args {
    let args = Args::parse();
    validate_paths([&args.dest, &args.target]);
    args
}
