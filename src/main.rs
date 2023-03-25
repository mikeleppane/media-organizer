use organizer::media_organizer::MediaOrganizer;

mod cli;
mod organizer;

fn main() -> color_eyre::Result<()> {
    color_eyre::install().expect("Failed to initialize color_eyre");
    let cli = cli::handle_command_line_arguments();
    let mut media_organizer = MediaOrganizer::new(cli);
    media_organizer.run()?;
    Ok(())
}
