use color_eyre::Result;
use colored::Color;

use crate::cli::Args;
use crate::organizer::collector::Collector;
use crate::organizer::media::{MediaFile, MediaType};
use crate::organizer::printer::print;

#[derive(Debug)]
pub struct MediaOrganizer {
    collector: Collector,
    options: Args,
}

impl MediaOrganizer {
    pub fn new(args: Args) -> Self {
        Self {
            collector: Collector::new(),
            options: args,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        print("", None);
        print(
            "*************** Media Organizer ***************",
            Some(Color::Blue),
        );
        self.collector.collect(
            &self.options.dest,
            &self.options.recursive,
            &self.options.verbose,
        );
        print("Analysis done!", None);
        self.collector.get_stats().print_stat();
        print("Starting to organize files to target folder...", None);
        self.organize_files()?;
        print("Done.", None);
        Ok(())
    }

    pub fn organize_files(&self) -> Result<()> {
        let images: Vec<&MediaFile> = self.collector.get_images();
        self.organize_images(images)?;
        Ok(())
    }

    pub fn organize_images(&self, images: Vec<&MediaFile>) -> Result<()> {
        for image in images {
            match self.options.style.as_str() {
                "year" => {
                    let path = self.options.target.join(image.year());
                    if !path.exists() {
                        std::fs::create_dir(&path)?;
                    }
                    if self.options.use_move {
                        std::fs::copy(self.options.dest.join(&image.name), &path.join(&image.name))?;
                        std::fs::remove_file(self.options.dest.join(&image.name))?;
                    } else {
                        std::fs::copy(self.options.dest.join(&image.name), &path.join(&image.name))?;
                    }
                }
                "month" => {
                    let path = self.options.target.join(image.year()).join(image.month());
                    if !path.exists() {
                        std::fs::create_dir_all(&path)?;
                    }

                    std::fs::copy(self.options.dest.join(&image.name), &path.join(&image.name))?;
                }
                "flat" => {
                    let path = &self.options.target;
                    if !path.exists() {
                        std::fs::create_dir_all(path)?;
                    }

                    std::fs::copy(self.options.dest.join(&image.name), &path.join(&image.name))?;
                }
                _ => panic!("Unrecognized style for the organizing media files: Possible options are: 'year', 'month', 'flat'")
            }
        }
        Ok(())
    }
}
