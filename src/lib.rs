#![allow(dead_code)]

pub mod spinner;
pub mod progress;
mod coloured;

extern crate termion;

pub use termion::color;

#[cfg(test)]
mod tests {
    use std::io::repeat;
    use super::*;
    #[test]
    fn test1() {
        println!("{} {} {} Blue", color::Bg(color::LightYellow), termion::style::Italic, color::Fg(color::Blue));
        println!("Hello world");
        println!(
            "{}, {}",
            coloured!("AAAA", color::LightGreen, color::Blue),
            coloured!("BBBB", color::Red, color::Magenta, termion::style::Underline, termion::style::Bold)
        );
    }
}
