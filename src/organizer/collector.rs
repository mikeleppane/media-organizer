use crate::organizer::media::{MediaFile, MediaType, IMAGE_FORMATS, VIDEO_FORMATS};
use chrono::{DateTime, Local};
use color_eyre::Result;
use std::ffi::OsStr;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Default)]
struct Stat {
    images: usize,
    image_size: usize,
    videos: usize,
    video_size: usize,
    total_size: usize,
}

#[derive(Debug)]
struct Collector<'f> {
    media: Vec<MediaFile<'f>>,
    stat: Stat,
}

fn get_media_type(suffix: &OsStr) -> Option<MediaType> {
    if let Some(suffix) = suffix.to_str() {
        if IMAGE_FORMATS.contains(&suffix) {
            return Some(MediaType::Image);
        }
        if VIDEO_FORMATS.contains(&suffix) {
            return Some(MediaType::Video);
        }
    }
    None
}

impl<'f> Collector<'f> {
    fn new() -> Self {
        Self {
            media: Vec::new(),
            stat: Stat::default(),
        }
    }

    fn collect(&mut self, path: PathBuf, recursive: bool, verbose: bool) -> Result<()> {
        let walker = match recursive {
            true => WalkDir::new(path),
            false => WalkDir::new(path).min_depth(0).max_depth(0),
        };
        let collected_files = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.metadata().is_ok() && entry.metadata().unwrap().is_file())
            .collect::<Vec<_>>();
    }

    fn convert_entries_to_media_files(entries: Vec<DirEntry>) -> Vec<MediaFile<'f>> {
        let mut media_files = Vec::<MediaFile>::new();
        for entry in entries {
            if entry.file_name().to_str().is_some() {
                let name = entry.file_name().to_str().unwrap();
                let mut created_at = Local::now();
                if let Ok(time) = entry.metadata().unwrap().created() {
                    created_at = time.into();
                } else {
                    println!("Not supported on this platform or filesystem");
                    continue;
                }
                let size = entry.metadata().unwrap().len();
                let mut media_type = MediaType::Image;
                if let Some(entension) = entry.path().extension() {
                    if let Some(m_type) = get_media_type(entension) {
                        media_type = m_type
                    } else {
                        println!(
                            "No file extension found ({:?}). Cannot determine file type. Skipping...",
                            entry.path()
                        );
                        continue;
                    }
                } else {
                    println!(
                        "No file extension found ({:?}). Cannot determine file type. Skipping...",
                        entry.path()
                    );
                    continue;
                }
                media_files.push(MediaFile::new(name, created_at, media_type, size));
            } else {
                println!(
                    "Unable to convert file name {:?}. Possibly not valid UTF-8. Skipping...",
                    entry.file_name()
                );
                continue;
            }
        }
        media_files
    }
}
