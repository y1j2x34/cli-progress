#[macro_export]
macro_rules! format_any {
    ($($text: expr),+) => {
        (|| {
            let mut texts = Vec::new();
            $(
                texts.push(format!("{}", $text));
            )+
            texts.concat()
        })()
    }
}

#[macro_export]
macro_rules! coloured {
    ($text: tt, $bgcolor: expr, $fgcolor: expr) => {
        coloured!($text, $bgcolor, $fgcolor, "")
    };
    ($text: tt, $bgcolor: expr, $fgcolor: expr, $($style: expr),+) => {
        format!(
            "{reset_start}{bgc}{fgc}{style}{text}{reset_end}",
            reset_start = "\x1B[0m",
            bgc = termion::color::Bg($bgcolor),
            fgc = termion::color::Fg($fgcolor),
            style = format_any!($($style),+),
            text = stringify!($text),
            reset_end = termion::style::Reset
        )
    };
}