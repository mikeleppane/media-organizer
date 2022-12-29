mod cli;
mod organizer;

use organizer::media_organizer::MediaOrganizer;

fn main() {
    let cli = cli::handle_command_line_arguments();
    let media_organizer = MediaOrganizer::new(cli);
    media_organizer.run();
}
