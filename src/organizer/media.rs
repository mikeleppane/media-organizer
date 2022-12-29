use chrono::{DateTime, Local};

pub const IMAGE_FORMATS: [&str; 4] = ["jpeg", "jpg", "gif", "png"];
pub const VIDEO_FORMATS: [&str; 1] = ["mp4"];

#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug)]
struct Formats<'img> {
    image: Vec<&'img str>,
    video: Vec<&'img str>,
}

impl<'img> Formats<'img> {
    fn new() -> Self {
        Self {
            image: IMAGE_FORMATS.to_vec(),
            video: VIDEO_FORMATS.to_vec(),
        }
    }

    fn add_image(&mut self, format: &'img str) {
        self.image.push(format);
    }

    fn add_video(&mut self, format: &'img str) {
        self.video.push(format);
    }
}

#[derive(Debug)]
pub struct MediaFile<'f> {
    name: &'f str,
    created_at: DateTime<Local>,
    r#type: MediaType,
    size: u64,
}

impl<'f> MediaFile<'f> {
    pub fn new(name: &str, created_at: DateTime<Local>, r#type: MediaType, size: u64) -> Self {
        Self {
            name,
            created_at,
            r#type,
            size,
        }
    }
}
