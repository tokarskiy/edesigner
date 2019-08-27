use crate::commands::Command;
use crate::interfaces::CommandRecognizer;
use crate::entities::Statement; 
use crate::keywords::keyword;
use crate::commands::is_first_token_specific_word; 
use crate::commands::variant_eq; 
use crate::entities::TokenType; 
use crate::commands::get_args; 
use crate::entities::Expression; 
use crate::entities::get_expression;
use crate::commands::find_token; 
use crate::entities::Token;
use crate::entities::Position; 
use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::keywords::is_keyword;

#[derive(Clone)]
pub struct FunctionCommand {
    pub name: String, 
    pub param_names: Vec<String>, 
    pub statement: Statement, 
    pub expression: Expression,
}

pub struct FunctionCommandRecognizer{}
impl CommandRecognizer for FunctionCommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command {
        let tokens = &statement.tokens;
        if !is_first_token_specific_word(tokens, keyword::FUNCTION) {
            return Command::None; 
        }
        
        let mut name: String = format!(""); 
        let len = tokens.len(); 
        let null_exp = get_expression(&vec![Token{
            token_type: TokenType::Number, 
            value: format!("0"),
            position: Position{ line: 0, line_position: 0 }
        }], &mut vec![]).unwrap(); 

        let init_token = tokens[0].clone(); 
        if len < 4 { // circle T ( )
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("{} FunctionName(arg1, arg2, ...) => arg1 + arg2; ", keyword::FUNCTION)])); 

            return Command::Function(FunctionCommand{
                name: name, 
                param_names: vec![], 
                statement: statement.clone(), 
                expression: null_exp,
            });
        }

        let name_token = tokens[1].clone(); 
        name = match name_token.token_type {
            TokenType::Word => {
                if is_keyword(&name_token.value) {
                    errs_acc.push(ErrorEntry::new(
                        errors::NAME_KEYWORD, 
                        &name_token.position, 
                        &vec![name_token.value.clone()])); 
                }

                name_token.value
            }, 
            _ => {
                errs_acc.push(ErrorEntry::new(
                    errors::INVALID_NAME, 
                    &name_token.position, 
                    &vec![name_token.value]));

                name 
            }
        }; 

        let opened = tokens[2].clone(); 
        let mut end: usize = 0; 
        match find_token(tokens, ")", TokenType::Symbol) {
            Option::Some(i) => {
                end = i;
            },
            Option::None => {
                errs_acc.push(ErrorEntry::new(
                    errors::SYMBOL_NOT_FOUND, 
                    &init_token.position, 
                    &vec![format!(")")]));
            }
        }

        if end == 0 {
            return Command::Function(FunctionCommand{
                name: name, 
                param_names: vec![], 
                statement: statement.clone(), 
                expression: null_exp,
            });
        }

        if !(variant_eq(&TokenType::Symbol, &opened.token_type) && opened.value == "(") {
            errs_acc.push(ErrorEntry::new(
                errors::UNEXPECTED_SYMBOL, 
                &opened.position, 
                &vec![format!("("), opened.value]));
        }

        let start = 3;
        let params = get_args(tokens, start, end, false); 
        if statement.substatements.len() != 0 {
            errs_acc.push(ErrorEntry::new(
                errors::ARE_SUBSTATEMENTS, 
                &init_token.position, 
                &vec![]));     
        }

        let rocket_sign_index = end + 1; 
        let rocket_sign_token = tokens[rocket_sign_index].clone(); 
        if !(variant_eq(&TokenType::Symbol, &rocket_sign_token.token_type) && rocket_sign_token.value == "=>") {
            errs_acc.push(ErrorEntry::new(
                errors::UNEXPECTED_SYMBOL, 
                &rocket_sign_token.position, 
                &vec![format!("=>"), rocket_sign_token.value]));
        }

        let expr_tokens: Vec<_> = tokens.iter()
            .skip(rocket_sign_index + 1)
            .map(|x| x.clone())
            .collect();  

        let expr = match get_expression(&expr_tokens, errs_acc) {
            Option::Some(x) => x,
            Option::None => null_exp,
        }; 

        Command::Function(FunctionCommand{
            name: name, 
            param_names: params, 
            expression: expr, 
            statement: statement.clone(), 
        })
    }
}