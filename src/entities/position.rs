use std::fmt::Formatter;
use std::fmt::Display; 

#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub line_position: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.line + 1, self.line_position + 1)
    }
}