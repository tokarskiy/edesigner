use std::fmt::Formatter;
use std::fmt::Display; 

#[derive(Clone)]
pub enum TokenType {
    Undefined = 0,
    Word = 1,
    Number = 2,
    Comment = 3,
    Symbol = 4,
    Empty = 5,
    _Test = 6,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TokenType::Undefined => write!(f, "Undefined"),
            TokenType::Word => write!(f, "Word"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Comment => write!(f, "Comment"),
            TokenType::Symbol => write!(f, "Symbol"),
            TokenType::Empty => write!(f, "Empty"), 
            TokenType::_Test => write!(f, "TEST"),
        }
    }
}