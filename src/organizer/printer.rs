use colored::{Color, ColoredString, Colorize};

pub fn print(message: &str, color: Option<Color>) {
    let colored = ColoredString::from(message);
    match color {
        Some(color) => println!("{}", colored.color(color)),
        None => println!("{}", colored),
    }
}
