use chrono::{DateTime, Datelike, Local};

pub const IMAGE_FORMATS: [&str; 4] = ["jpeg", "jpg", "gif", "png"];
pub const VIDEO_FORMATS: [&str; 1] = ["mp4"];

#[derive(Debug, PartialEq, Eq, Default)]
pub enum MediaType {
    #[default]
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

#[derive(Debug, Default)]
pub struct MediaFile {
    pub name: String,
    created_at: DateTime<Local>,
    r#type: MediaType,
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
