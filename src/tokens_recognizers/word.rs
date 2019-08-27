use crate::interfaces::TokenRecognizer; 
use crate::entities::Token; 
use crate::entities::CodeChar; 
use crate::entities::TokenType;

pub struct WordTokenRecognizer {}
impl WordTokenRecognizer {
    fn is_allowed_char(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '.'
    }

    fn is_initial_char(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
}

impl TokenRecognizer for WordTokenRecognizer
{
    fn recognize_token(&self, chars: &Vec<CodeChar>, position: usize) -> (Option<Token>, usize)
    {
        let curr_char_data = chars[position].clone();
        let curr_char = curr_char_data.symbol; 
        
        if !self.is_initial_char(curr_char) {
            return (None, 0); 
        }

        let mut result: String = chars.iter()
            .skip(position)
            .take_while(|x| x.position.line == curr_char_data.position.line)
            .take_while(|x| self.is_allowed_char(x.symbol))
            .map(|x| x.symbol)
            .collect(); 

        let len = result.len(); 
        let mut next_position = position + len; 
        let last = result.chars().last().unwrap(); 
        if last == '.' {
            result = result.chars().take(len - 1).collect(); 
            next_position = next_position - 1; 
        }

        let token = Option::Some(Token{
            value: result, 
            position: curr_char_data.position,
            token_type: TokenType::Word,
        });
        
        (token, next_position)
    } 
}
