use crate::interfaces::TokenRecognizer; 
use crate::entities::Token; 
use crate::entities::CodeChar; 
use crate::entities::TokenType;

pub struct SymbolTokenRecognizer {}
const COMBINED_OPERATORS: [&'static str; 5] = [
    ">=",
    "<=",
    "==",
    "!=",
    "=>", 
];

impl TokenRecognizer for SymbolTokenRecognizer
{
    fn recognize_token(&self, chars: &Vec<CodeChar>, position: usize) -> (Option<Token>, usize)
    {
        let symbols = "{}[]();=,+-=*/<>!".to_string(); 
        let char_data = chars[position].clone();
        let curr_char = char_data.symbol;
        let pos = char_data.position; 

        if symbols.contains(curr_char)
        {
            let mut next_pos = position + 1;
            let mut token = Option::Some(Token{
                position: pos.clone(),
                value: curr_char.to_string(),
                token_type: TokenType::Symbol,
            });

            match chars.get(next_pos) {
                Option::None => { },
                Option::Some(next_char_data) => {
                    let next_char = next_char_data.symbol;
                    let two_chars_str: String = vec![curr_char, next_char].iter().collect();
                    for &operator in &COMBINED_OPERATORS {
                        if operator.to_string() == two_chars_str {
                            token = Option::Some(Token{
                                position: pos.clone(),
                                value: two_chars_str.clone(),
                                token_type: TokenType::Symbol,
                            });

                            next_pos = position + 2; 
                        }
                    }
                }
            }

            (token, next_pos)
        }
        else 
        {
            (None, 0)
        }

    } 
}
