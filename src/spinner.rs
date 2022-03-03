use std::fmt::{Display, Formatter};

pub struct SpinnerStyle<'a> {
    interval: i32,
    frames: [&'a str]
}

pub struct Spinner<'a> {
    style: Box<SpinnerStyle<'a>>,
    total: i32,
    start: i32
}


impl Spinner<'_> {
    fn to_string(&self) -> String {
        return String::from("");
    }
}

impl Display for Spinner<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}