use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::entities::CodeChar;
use crate::entities::Token; 
use crate::entities::TokenType; 
use crate::entities::Position; 
use crate::interfaces::TokenRecognizer; 

fn collect_code_chars(code: String) -> Vec<CodeChar> {
    let lines_iter = code.split("\n"); // TODO: I dont know is it true way
    let lines = lines_iter.collect::<Vec<&str>>(); 

    let mut code_chars: Vec<CodeChar> = Vec::new(); 
    let mut i: usize = 0;
    for line in lines 
    {
        let text = format!("{} ", line);
        let mut j: usize = 0; 
        for ch in text.chars()
        {
            code_chars.push(CodeChar{
                position: Position{
                    line: i,
                    line_position: j,
                },

                symbol: ch,
            });

            j += 1; 
        }

        i += 1;
    }

    code_chars
}

pub fn tokenize(code: String, recognizers: &Vec<&dyn TokenRecognizer>, errs_acc: &mut Vec<ErrorEntry>) -> Vec<Token> {
    let code_chars = collect_code_chars(code);
    let code_chars = &code_chars; 
    let mut tokens: Vec<Token> = Vec::new(); 
    let mut pos = 0; 
    let len = code_chars.len(); 
    //let len = 0; 
    'outer: loop 
    {
        if pos >= len
        {
            break 'outer; 
        }

        let curr_char_data = code_chars[pos].clone();
        if curr_char_data.symbol == ' '
        {
            pos += 1; 
            continue 'outer; 
        }

        let mut status = false; 
        'inner: for recognizer in recognizers 
        {
            let (token, next) = recognizer.recognize_token(code_chars, pos); 
            match token {
                None => continue 'inner,
                Some(x) => {
                    status = true; 
                    pos = next; 
                    tokens.push(x);
                }
            };
        }

        if !status {
            // If recognizers failed, throw the undefined 
            // symbol error
            if curr_char_data.symbol != '\r' { // TODO: Don't know is it good approach
                errs_acc.push(ErrorEntry::new(
                    errors::UNDEFINED_SYMBOL, 
                    &curr_char_data.position, 
                    &vec![curr_char_data.symbol.to_string()]));           
                
                tokens.push(Token{
                    token_type: TokenType::Undefined, 
                    position: curr_char_data.position.clone(),
                    value: curr_char_data.symbol.to_string(), 
                });
            }

            pos += 1;
        }
    }

    tokens
}

