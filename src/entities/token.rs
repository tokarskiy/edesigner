use crate::entities::Position; 
use crate::entities::TokenType; 

#[derive(Clone)]
pub struct Token {
    pub position: Position,
    pub value: String,
    pub token_type: TokenType,
}
