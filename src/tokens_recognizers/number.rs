use crate::interfaces::TokenRecognizer; 
use crate::entities::Token; 
use crate::entities::CodeChar; 
use crate::entities::TokenType;

pub struct NumberTokenRecognizer {}
impl NumberTokenRecognizer {
    fn is_allowed_char(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '.'
    }

    fn is_initial_char(&self, ch: char) -> bool {
        ch.is_numeric()
    }
}

impl TokenRecognizer for NumberTokenRecognizer
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
        let next_position = position + len; 
        let extend: String = match chars.get(next_position) {
            Option::None => format!(""),
            Option::Some(ch) if ch.symbol == '-' || ch.symbol == '+' => {
                let last = chars[next_position - 1].clone(); 
                if last.symbol == 'E' || last.symbol == 'e' {
                    let mut after_e = ch.symbol.to_string(); 
                    let append: String = chars.iter()
                        .skip(next_position + 1)
                        .take_while(|x| x.position.line == curr_char_data.position.line)
                        .take_while(|x| self.is_allowed_char(x.symbol))
                        .map(|x| x.symbol)
                        .collect();
                    
                    after_e.push_str(&append);
                    after_e
                }
                else {
                    format!("")
                }
            },
            _ => format!(""),
        };

        result.push_str(&extend); 
        let len = result.len(); 
        let next_position = position + len; 
        
        let token = Option::Some(Token{
            value: result, 
            position: curr_char_data.position,
            token_type: TokenType::Number,
        });
        
        (token, next_position)
    } 
}