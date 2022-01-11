use crate::entities::Token; 
use crate::entities::TokenType; 
use crate::interfaces::CommandRecognizer;
use crate::entities::Statement; 
use crate::commands::Command; 
use crate::entities::ErrorEntry;
use crate::entities::errors;

pub fn is_first_token_specific_word(tokens: &Vec<Token>, word: &str) -> bool {
    match tokens.first() {
        Option::None => false,
        Option::Some(token) => token.value == word.to_string() && 
                               variant_eq(&token.token_type, &TokenType::Word),
    }
}

pub fn find_token(tokens: &Vec<Token>, word: &str, token_type: TokenType) -> Option<usize> {
    let mut i: usize = 0; 
    
    for token in tokens {
        if token.value == word.to_string() && variant_eq(&token.token_type, &token_type) {
            return Option::Some(i); 
        }

        i += 1;
    }

    return Option::None; 
}

pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn get_args(tokens: &Vec<Token>, start: usize, end: usize, allow_nums: bool) -> Vec<String> {
    let mut cntr: usize = 0; 
    let mut args: Vec<String> = vec![]; 

    for i in start..end {
        let token = tokens[i].clone();
        match cntr % 2 {
            0 => {
                if variant_eq(&TokenType::Word, &token.token_type) {
                    args.push(token.value); 
                }
                else if variant_eq(&TokenType::Number, &token.token_type) && allow_nums {
                    args.push(token.value); 
                }
                else {
                    // TODO: Add error 
                }
            },
            1 => {
                if !(variant_eq(&TokenType::Symbol, &token.token_type) && token.value == ",") {
                    // TODO: Add error 
                }
            },
            _ => { } // this code is unreachable
        } 

        cntr += 1; 
    }

    args
}

pub fn get_expressions_tokens(tokens: &Vec<Token>, start: usize, end: usize) -> Vec<Vec<Token>> {
    let mut expressions: Vec<Vec<Token>> = vec![]; 
    let mut current_exp: Vec<Token> = vec![]; 
    let mut level = 0; 

    for i in start..end {
        let token = tokens[i].clone();
        let is_comma = variant_eq(&TokenType::Symbol, &token.token_type) && token.value == ","; 

        match is_comma {
            true if level == 0 => {
                expressions.push(current_exp.clone());
                current_exp = vec![];  
            },
            _ => {
                if variant_eq(&TokenType::Symbol, &token.token_type) {
                    if token.value == "(" {
                        level += 1; 
                    }

                    if token.value == ")" {
                        if level == 0 {
                            // TODO: Add error 
                        }

                        level -= 1; 
                    }
                }

                current_exp.push(token.clone()); 
            },
        }
    }

    if current_exp.len() > 0 {
        expressions.push(current_exp.clone()); 
    }

    if level != 0 {
        // TODO: Add error 
    }

    expressions
}

pub fn get_commands(statements: &Vec<Statement>, recognizers: &Vec<&dyn CommandRecognizer>, errs_acc: &mut Vec<ErrorEntry>) -> Vec<Command> {
    let mut cmds: Vec<Command> = vec![];

    'outer: for statement in statements {
        'inner: for recognizer in recognizers {
            let command = recognizer.from_statement(statement, errs_acc);
            match command {
                Command::None => { 
                    continue 'inner; 
                },
                _ => {
                    cmds.push(command); 
                    continue 'outer; 
                },
            }
        }

        // Throwing an error (unrecognized statement)
        let first_token = statement.tokens[0].clone();
        let mut stat_text: String = format!(""); 
        for token in &statement.tokens {
            stat_text.push_str(&token.value); 
        }

        errs_acc.push(ErrorEntry::new(
            errors::UNDEFINED_STATEMENT,
            &first_token.position, 
            &vec![stat_text])); 
    } 

    cmds
}