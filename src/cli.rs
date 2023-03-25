use std::path::{Path, PathBuf};

use clap::Parser;

const VALID_STYLES: [&str; 3] = ["year", "month", "flat"];

/// About Media Organizer
#[derive(Parser, Debug)]
#[clap(
    author = "Mikko Leppänen <mleppan23@gmail.com>",
    version = "0.1",
    about
)]
pub struct Args {
    /// Destination path
    #[arg(
    value_parser = verify_path,
    short,
    long,
    value_name = "PATH",
    default_value = ".",
    )]
    pub dest: PathBuf,
    /// Target path
    #[arg(value_parser = verify_path, short, long, value_name = "PATH", default_value = ".")]
    pub target: PathBuf,
    /// How to organize images: possible options are: "year" | "month" | "flat"
    #[arg(value_parser = verify_style, short, long, default_value = "year")]
    pub style: String,
    /// Do you want to keep videos in a separate folder (videos)?
    #[arg(long, default_value_t = true)]
    pub separate_videos: bool,
    /// Find files recursively
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,
    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// Include additional image format(s) as a comma separated list: "jpeg,png". Defaults are: "jpeg", "jpg", "gif", "png", "svg", "tiff", "tif", "webp"
    #[arg(long, default_value = "")]
    pub include_image_format: String,
    /// Include additional video format(s) as a comma separated list: "webm,mkv". Defaults are: "webm", "mkv", "flv", "ogg", "ogv", "avi", "m2v", "m4v", "mpg", "mpeg", "mp4", "asf", "rmvb", "wmv"
    #[arg(long, default_value = "")]
    pub include_video_format: String,
    /// Move content to target instead of copying
    #[arg(short, long, default_value_t = false)]
    pub use_move: bool,
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    if Path::new(path).exists() {
        Ok(PathBuf::from(path))
    } else {
        Err(format!("Given path {:?} does not exists.", path))
    }
}

fn verify_style(style: &str) -> Result<String, String> {
    if VALID_STYLES.contains(&style.to_lowercase().as_ref()) {
        Ok(style.to_string())
    } else {
        Err(format!(
            "Given style is not defined. Possible options are: {:?}",
            VALID_STYLES
        ))
    }
}

pub fn handle_command_line_arguments() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use clap::Parser;
    use pretty_assertions::assert_eq;

    use crate::cli::Args;

    struct Inputs {
        pub cli: String,
    }

    impl Inputs {
        pub fn new(cli: &str) -> Self {
            Self {
                cli: cli.to_string(),
            }
        }
    }

    #[test]
    fn should_parse_cli_correctly_with_valid_input() {
        let inputs = vec![
            Inputs::new("media-organizer --dest . --style year"),
            Inputs::new("media-organizer --target . --style month"),
            Inputs::new("media-organizer -t . -s FLAT"),
        ];

        for input in inputs {
            Args::try_parse_from(input.cli.split_whitespace()).unwrap();
        }
    }

    #[test]
    fn should_parse_cli_correctly_with_default_input() {
        let args = Args::try_parse_from("program".split_whitespace()).unwrap();
        assert_eq!(args.dest, PathBuf::from("."));
        assert_eq!(args.target, PathBuf::from("."));
        assert_eq!(args.style, "year");
        assert_eq!(args.separate_videos, true);
        assert_eq!(args.recursive, false);
        assert_eq!(args.verbose, false);
        assert_eq!(args.include_video_format, "");
        assert_eq!(args.include_image_format, "");
        assert_eq!(args.use_move, false);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_trying_to_parse_invalid_dest_path() {
        let inputs = vec![Inputs::new("media-organizer --dest invalid")];

        for input in inputs {
            Args::try_parse_from(input.cli.split_whitespace()).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn should_panic_if_trying_to_parse_invalid_target_path() {
        let inputs = vec![Inputs::new("media-organizer --target invalid")];

        for input in inputs {
            Args::try_parse_from(input.cli.split_whitespace()).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn should_panic_if_trying_to_parse_invalid_style() {
        let inputs = vec![Inputs::new("media-orgnaizer --style invalid-style")];

        for input in inputs {
            Args::try_parse_from(input.cli.split_whitespace()).unwrap();
        }
    }
}
