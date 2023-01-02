use crate::cli::Args;
use crate::organizer::collector::Collector;
use crate::organizer::printer::print;
use colored::Color;

#[derive(Debug)]
pub struct MediaOrganizer {
    options: Args,
}

impl MediaOrganizer {
    pub fn new(args: Args) -> Self {
        Self { options: args }
    }

    pub fn run(&self) {
        print("", None);
        print(
            "*************** Media Organizer ***************",
            Some(Color::Blue),
        );
        let mut collector = Collector::new();
        let media_files = collector.collect(
            &self.options.dest,
            &self.options.recursive,
            &self.options.verbose,
        );
        print("Analysis done!", None);
        collector.get_stats().print_stat();
    }
}
