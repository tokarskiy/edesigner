use std::collections::HashMap; 
use crate::entities::Token; 
use crate::entities::Statement; 
use crate::entities::TokenType; 
use crate::entities::Position; 
use crate::entities::ErrorEntry; 
use crate::entities::errors; 

pub fn get_statements(tokens: &Vec<Token>, errs_acc: &mut Vec<ErrorEntry>) -> Vec<Statement> {
    let (groups, separators) = divide_tokens(tokens, errs_acc); 
    let braces_map = get_braces_map(&separators, errs_acc);

    get_statements_rec(&groups, &separators, &braces_map, 0, groups.len())
}

fn get_statements_rec(groups: &Vec<Vec<Token>>, 
                      separators: &Vec<Token>, 
                      map: &HashMap<usize, usize>, 
                      start: usize, 
                      end: usize) -> Vec<Statement> {

    if start >= groups.len() {
        return vec![]; 
    }

    let mut i = start; 
    let mut statements: Vec<Statement> = Vec::new();

    while i < end {
        let all_tokens = groups[i].to_vec();
        let tokens: Vec<_> = all_tokens.iter()
            .filter(|x| match x.token_type {
                TokenType::Comment => false,
                _ => true,
            })
            .map(|x| x.clone())
            .collect(); 
        
        let comments: Vec<_> = all_tokens.iter()
            .filter(|x| match x.token_type {
                TokenType::Comment => true,
                _ => false,
            })
            .map(|x| x.clone())
            .collect(); 

        let mut substatements: Vec<Statement> = Vec::new(); 
        if map.contains_key(&i) {
            let new_start = i + 1;
            let new_end = map[&i] + 1;

            substatements = get_statements_rec(groups, separators, map, new_start, new_end);
            i = new_end; 
        }
        else {
            i += 1; 
        }

        if tokens.len() > 0 || substatements.len() > 0 {
            let statement = Statement{
                tokens: tokens,
                comments: comments,
                substatements: substatements,
            };

            statements.push(statement); 
        }
    }

    return statements;
}

fn divide_tokens(tokens: &Vec<Token>, errs_acc: &mut Vec<ErrorEntry>) -> (Vec<Vec<Token>>, Vec<Token>) {
    let mut groups: Vec<Vec<Token>> = Vec::new(); 
    let mut separators: Vec<Token> = Vec::new(); 
    let mut curr_group: Vec<Token> = Vec::new(); 

    for token in tokens {
        if is_separator(token) {
            groups.push(curr_group.to_vec()); 
            separators.push(token.clone()); 
            curr_group.clear();
        }
        else {
            curr_group.push(token.clone()); 
        }
    }

    if curr_group.len() > 0 {
        // If statement doesn't finish with separator, 
        // throw an error
        let last = curr_group.last().unwrap(); 
        errs_acc.push(ErrorEntry::new(
            errors::UNFINISHED_COMMAND, 
            &last.position, 
            &vec![]));

        groups.push(curr_group); 
        
        separators.push(Token{
            token_type: TokenType::Empty,
            value: "".to_string(), // TODO: String.Empty
            position: Position{line: 0, line_position: 0}
        });
    }

    (groups, separators)
}

fn is_semicolon(token: &Token) -> bool {
    match token.token_type{
        TokenType::Symbol => token.value == ";",
        _ => false,
    }
}

fn is_opened_brace(token: &Token) -> bool {
    match token.token_type{
        TokenType::Symbol => token.value == "{",
        _ => false,
    }
}

fn is_closed_brace(token: &Token) -> bool {
    match token.token_type{
        TokenType::Symbol => token.value == "}",
        _ => false,
    }
}

fn is_separator(token: &Token) -> bool {
    is_semicolon(token) || is_opened_brace(token) || is_closed_brace(token)
}

fn get_braces_map(separators: &Vec<Token>, errs_acc: &mut Vec<ErrorEntry>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new(); 
    let mut stack: Vec<usize> = Vec::new(); 
    let mut i = 0;

    for separator in separators {
        let ch = separator.clone().value;
        if ch == "{" {
            stack.push(i); 
        }

        if ch == "}" {
            match stack.pop() {
                None => {  
                    // Throw an error if amount of opened brackets 
                    // not equal to amount of closed
                    errs_acc.push(ErrorEntry::new(errors::OPEN_CLOSED_BRACKETS, &separator.position, &vec![]));
                } 
                Some(start) => {
                    map.insert(start, i); 
                }
            }
        }

        i += 1; 
    }

    map
}


