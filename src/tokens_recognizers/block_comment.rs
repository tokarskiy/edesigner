use crate::interfaces::TokenRecognizer; 
use crate::entities::Token; 
use crate::entities::CodeChar; 
use crate::entities::TokenType;

pub struct BlockCommentTokenRecognizer {}
impl TokenRecognizer for BlockCommentTokenRecognizer
{
    fn recognize_token(&self, chars: &Vec<CodeChar>, position: usize) -> (Option<Token>, usize)
    {
        if position >= chars.len() - 1 {

            return (None, 0); 
        }

        let curr_char_data = chars[position].clone();
        let next_char_data = chars[position + 1].clone(); 

        let curr_char = curr_char_data.symbol; 
        let next_char = next_char_data.symbol; 

        if curr_char == '/' && next_char == '*' {
            let string: String = chars.iter()
                .skip(position)
                .map(|x| x.symbol)
                .collect(); 

            let addr = string.find("*/"); 
            let len = match addr {
                None => string.len(), 
                Some(ad) => ad + 2,
            };

            let comment_text: String = string.chars().take(len).collect();
            let next_position = position + len + 1;

            let token = Option::Some(Token{
                position: curr_char_data.position,
                value: comment_text,
                token_type: TokenType::Comment,
            });

            (token, next_position)
        }
        else {
            (None, 0)
        }
    } 
}
