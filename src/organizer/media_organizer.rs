use crate::cli::Args;

#[derive(Debug)]
pub struct MediaOrganizer {
    options: Args,
}

impl MediaOrganizer {
    pub fn new(args: Args) -> Self {
        Self { options: args }
    }

    pub fn run(&self) {
        todo!()
    }
}
