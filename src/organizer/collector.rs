use std::ffi::OsStr;
use std::path::PathBuf;

use colored::{Color, Colorize};
use walkdir::WalkDir;

use crate::organizer::media::{Formats, MediaFile, MediaType};
use crate::organizer::printer::print;

trait HumanReadable: Sized {
    fn to_human(self) -> String;
}

impl HumanReadable for u64 {
    fn to_human(self) -> String {
        match self {
            0..=999 => self.to_string(),
            1000..=999_999 => {
                format!("{:.1} KB", self as f64 / 1000f64)
            }
            1_000_000..=999_999_999 => {
                format!("{:.1} MB", self as f64 / 1_000_000f64)
            }
            1_000_000_000.. => {
                format!("{:.1} GB", self as f64 / 1_000_000_000f64)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stat {
    images: u64,
    images_size: u64,
    videos: u64,
    videos_size: u64,
    total_size: u64,
}

impl Stat {
    fn add_media(&mut self, media_type: &MediaType, size: u64) {
        match media_type {
            MediaType::Image => {
                self.images += 1;
                self.images_size += size;
                self.total_size += size;
            }
            MediaType::Video => {
                self.videos += 1;
                self.videos_size += size;
                self.total_size += size;
            }
        }
    }

    pub fn print_stat(&self) {
        print("Result: ", None);
        println!("{}{}", "Number of images: ".green(), self.images);
        print(
            format!("Size of images: {}", self.images_size.to_human()).as_str(),
            Some(Color::Green),
        );
        print(
            format!("Number of videos: {}", self.videos).as_str(),
            Some(Color::Green),
        );
        print(
            format!("Size of videos: {}", self.videos_size.to_human()).as_str(),
            Some(Color::Green),
        );
        print(
            format!("Total size: {}", self.total_size.to_human()).as_str(),
            Some(Color::Green),
        );
    }
}

#[derive(Debug)]
pub struct Collector {
    media: Vec<MediaFile>,
    pub formats: Formats,
    stat: Stat,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            media: Vec::new(),
            formats: Formats::new(),
            stat: Stat::default(),
        }
    }

    fn get_media_type(&mut self, suffix: &OsStr) -> Option<MediaType> {
        self.formats.get_media_type(suffix)
    }

    pub fn get_stats(&self) -> &Stat {
        &self.stat
    }

    #[allow(dead_code)]
    pub fn get_files(&self) -> &Vec<MediaFile> {
        &self.media
    }

    pub fn get_images(&self) -> Vec<&MediaFile> {
        self.media.iter().filter(|&f| f.is_image()).collect::<_>()
    }

    #[allow(dead_code)]
    pub fn get_videos(&self) -> Vec<&MediaFile> {
        self.media.iter().filter(|&f| !f.is_image()).collect::<_>()
    }

    pub fn collect(&mut self, path: &PathBuf, recursive: &bool, _verbose: &bool) {
        let walker = match recursive {
            true => WalkDir::new(path),
            false => WalkDir::new(path).min_depth(0).max_depth(1),
        };
        print(
            format!("Analysing source path ({:?}) for media files...", path).as_str(),
            Some(Color::Green),
        );
        self.media = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.metadata().is_ok() && entry.metadata().unwrap().is_file())
            .filter_map(|entry| {
                if let Some(entension) = entry.path().extension() {
                    if self.get_media_type(entension).is_some() {
                        if let Some(media) = self.convert_entry_to_media_file(entry) {
                            return Some(media);
                        }
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    }

    fn convert_entry_to_media_file(&mut self, entry: walkdir::DirEntry) -> Option<MediaFile> {
        if let Ok(name) = entry.file_name().to_os_string().into_string() {
            let created_at = if let Ok(time) = entry
                .metadata()
                .unwrap_or_else(|_| panic!("Cannot get size for a file: {}. Perhaps there's no permissions to access the file?", name))
                .modified()
            {
                time.into()
            } else {
                println!("Not supported on this platform or filesystem");
                return None;
            };
            let size = entry.metadata().unwrap_or_else(|_| panic!("Cannot get size for a file: {}. Perhaps there's no permissions to access the file?", name)).len();
            let media_type = if let Some(entension) = entry.path().extension() {
                if let Some(m_type) = self.formats.get_media_type(entension) {
                    m_type
                } else {
                    println!(
                        "No file extension found ({:?}). Cannot determine file type. Skipping...",
                        entry.path()
                    );
                    return None;
                }
            } else {
                println!(
                    "No file extension found ({:?}). Cannot determine file type. Skipping...",
                    entry.path()
                );
                return None;
            };
            self.stat.add_media(&media_type, size);
            Some(MediaFile::new(name, created_at, media_type, size))
        } else {
            println!(
                "Unable to convert file name {:?}. Possibly not valid UTF-8. Skipping...",
                entry.file_name()
            );
            None
        }
    }
}
