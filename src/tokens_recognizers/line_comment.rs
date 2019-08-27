use crate::interfaces::TokenRecognizer; 
use crate::entities::Token; 
use crate::entities::CodeChar; 
use crate::entities::TokenType;

pub struct LineCommentTokenRecognizer {}
impl TokenRecognizer for LineCommentTokenRecognizer
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

        if curr_char == '/' && next_char == '/' {
            let comment_text: String = chars.iter()
                .skip(position)
                .take_while(|x| x.position.line == curr_char_data.position.line)
                .map(|x| x.symbol)
                .collect(); 
            
            let next_position = position + comment_text.len() + 1;
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