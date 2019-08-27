use crate::entities::Token;

#[derive(Clone)]
pub struct Statement {
    pub tokens: Vec<Token>,
    pub substatements: Vec<Statement>,
    pub comments: Vec<Token>,
}
