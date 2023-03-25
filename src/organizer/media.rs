use std::ffi::OsStr;

use chrono::{DateTime, Datelike, Local};

pub const IMAGE_FORMATS: [&str; 8] = ["jpeg", "jpg", "gif", "png", "svg", "tiff", "tif", "webp"];
pub const VIDEO_FORMATS: [&str; 14] = [
    "webm", "mkv", "flv", "ogg", "ogv", "avi", "m2v", "m4v", "mpg", "mpeg", "mp4", "asf", "rmvb",
    "wmv",
];

#[derive(Debug, PartialEq, Eq, Default)]
pub enum MediaType {
    #[default]
    Image,
    Video,
}

#[derive(Debug)]
pub struct Formats {
    image: Vec<String>,
    video: Vec<String>,
}

impl Formats {
    pub fn new() -> Self {
        Self {
            image: IMAGE_FORMATS.iter().map(|&img| img.to_string()).collect(),
            video: VIDEO_FORMATS.iter().map(|&img| img.to_string()).collect(),
        }
    }

    pub fn get_media_type(&self, suffix: &OsStr) -> Option<MediaType> {
        if let Some(suffix) = suffix.to_str() {
            if self.image.iter().any(|img| img == suffix) {
                return Some(MediaType::Image);
            }
            if self.video.iter().any(|video| video == suffix) {
                return Some(MediaType::Video);
            }
        }
        None
    }

    pub fn add_image(&mut self, format: String) {
        self.image.push(format);
    }

    pub fn add_video(&mut self, format: String) {
        self.video.push(format);
    }
}

#[derive(Debug, Default)]
pub struct MediaFile {
    pub name: String,
    created_at: DateTime<Local>,
    r#type: MediaType,
    #[allow(dead_code)]
    size: u64,
}

impl MediaFile {
    pub fn new(name: String, created_at: DateTime<Local>, r#type: MediaType, size: u64) -> Self {
        Self {
            name,
            created_at,
            r#type,
            size,
        }
    }

    pub fn is_image(&self) -> bool {
        match self.r#type {
            MediaType::Image => true,
            MediaType::Video => false,
        }
    }

    pub fn year(&self) -> String {
        self.created_at.year().to_string()
    }
    pub fn month(&self) -> &str {
        match self.created_at.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("Not a month"),
        }
    }
}
