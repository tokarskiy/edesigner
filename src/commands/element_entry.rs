use crate::commands::Command;
use crate::entities::Statement;
use crate::entities::Expression;  
use crate::interfaces::CommandRecognizer; 
use crate::commands::variant_eq;
use crate::entities::TokenType; 
use crate::commands::find_token;
use crate::commands::get_args; 
use crate::commands::get_expressions_tokens; 
use crate::entities::get_expression;
use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::keywords::is_keyword;

#[derive(Clone)]
pub struct ElementEntryCommand {
    pub entry_name: String, // variable name 
    pub element_name: String, // type name
    pub nodes: Vec<String>, 
    pub param_expressions: Vec<Expression>,
    pub statement: Statement,
}

pub struct ElementEntryCommandRecognizer{}
impl CommandRecognizer for ElementEntryCommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command {
        let tokens = &statement.tokens;
        match find_token(tokens, "=", TokenType::Symbol) {
            Option::Some(_) => { }
            Option::None => {
                return Command::None; 
            } 
        };

        let len = tokens.len(); 
        let init_token = tokens[0].clone(); 
        if len < 7 { // a [ ] = T ( )
            // TODO: Add error (invalid command format)
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("elemName [node1, node2, ...] = ElementName(arg1, arg2, ...); ")])); 

            return Command::ElementEntry(ElementEntryCommand{
                entry_name: format!(""),
                element_name: format!(""),  
                nodes: vec![], 
                param_expressions: vec![], 
                statement: statement.clone(), 
            });
        }

        let entry_name_token = tokens[0].clone(); 
        if !variant_eq(&TokenType::Word, &entry_name_token.token_type) {
            errs_acc.push(ErrorEntry::new(
                errors::INVALID_NAME, 
                &entry_name_token.position, 
                &vec![format!("["), entry_name_token.value.clone()]));
        }

        let entry_name = entry_name_token.value.clone();
        let start_token = tokens[1].clone(); 
        if !(variant_eq(&TokenType::Symbol, &start_token.token_type) && start_token.value == "[") {
            errs_acc.push(ErrorEntry::new(
                errors::UNEXPECTED_SYMBOL, 
                &start_token.position, 
                &vec![format!("["), start_token.value]));
        }

        let start = 2; 
        let mut end: usize = 0; 
        match find_token(tokens, "]", TokenType::Symbol) {
            Option::Some(i) => {
                end = i;
            },
            Option::None => {
                errs_acc.push(ErrorEntry::new(
                    errors::SYMBOL_NOT_FOUND, 
                    &start_token.position, 
                    &vec![format!("]")]));
            }
        }

        let nodes = get_args(tokens, start, end, true); 

        // 4 because  '=', element_name, '(' and ')'
        if end > len - 4 {
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("elemName [node1, node2, ...] = ElementName(arg1, arg2, ...); ")])); 
        }

        let eq_index = end + 1;
        let eq_token = tokens[eq_index].clone(); 
        if !(variant_eq(&TokenType::Symbol, &eq_token.token_type) && eq_token.value == "=") {
            // TODO: Add error 
        }

        let name_index = end + 2;
        let name_token = tokens[name_index].clone(); 
        let mut element_name = format!(""); 
        element_name = match name_token.token_type {
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

                element_name 
            }
        }; 

        let opened = tokens[name_index + 1].clone(); 
        let end = len - 1; 
        let closed = tokens[end].clone(); 

        if !(variant_eq(&TokenType::Symbol, &opened.token_type) && opened.value == "(") {
            errs_acc.push(ErrorEntry::new(
                errors::UNEXPECTED_SYMBOL, 
                &opened.position, 
                &vec![format!("("), opened.value]));
        }

        if !(variant_eq(&TokenType::Symbol, &closed.token_type) && closed.value == ")") {
            errs_acc.push(ErrorEntry::new(
                errors::UNEXPECTED_SYMBOL, 
                &closed.position, 
                &vec![format!(")"), closed.value]));
        }

        let start = name_index + 2;
        let param_expressions_tokens = get_expressions_tokens(tokens, start, end); 
        let mut param_expressions: Vec<Expression> = vec![]; 

        for tokens in &param_expressions_tokens {
            let op_expr = get_expression(tokens, errs_acc); 
            match op_expr {
                Option::Some(expr) => {
                    param_expressions.push(expr); 
                },
                Option::None => {
                    // TODO: Add error 
                }
            }
        }

        Command::ElementEntry(ElementEntryCommand{
            entry_name: entry_name,
            element_name: element_name,  
            nodes: nodes, 
            param_expressions: param_expressions, 
            statement: statement.clone(), 
        })
    }
}
