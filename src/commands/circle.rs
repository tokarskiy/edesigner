use crate::commands::Command;
use crate::interfaces::CommandRecognizer;
use crate::entities::Statement; 
use crate::keywords::keyword;
use crate::commands::is_first_token_specific_word; 
use crate::commands::variant_eq; 
use crate::entities::TokenType; 
use crate::commands::get_args; 
use crate::commands::ElementEntryCommand; 
use crate::commands::ElementEntryCommandRecognizer; 
use crate::commands::get_commands; 
use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::keywords::is_keyword;

#[derive(Clone)]
pub struct CircleCommand {
    pub name: String, 
    pub param_names: Vec<String>, 
    pub statement: Statement, 
    pub element_entries: Vec<ElementEntryCommand>, 
}

pub struct CircleCommandRecognizer{}
impl CommandRecognizer for CircleCommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command {
        let tokens = &statement.tokens;
        if !is_first_token_specific_word(tokens, keyword::CIRCLE) {
            return Command::None; 
        }
        
        let mut name: String = format!(""); 
        let len = tokens.len(); 
        let init_token = tokens[0].clone(); 
        if len < 4 { // circle T ( )
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("{} <CIRCLE_NAME>(<ARGS_LIST>) {{ ... }} ", keyword::PACKAGE)])); 

            return Command::Circle(CircleCommand{
                name: name, 
                param_names: vec![], 
                element_entries: vec![], 
                statement: statement.clone(), 
            });
        }

        let name_token = tokens[1].clone(); 
        name = match name_token.token_type {
            TokenType::Word => {
                if is_keyword(&name) {
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
        let closed = tokens[len - 1].clone(); 
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

        let start = 3;
        let end = len - 1; 
        let params = get_args(tokens, start, end, false); 
        if statement.substatements.len() == 0 {
            errs_acc.push(ErrorEntry::new(
                errors::NO_SUBSTATEMENTS, 
                &init_token.position, 
                &vec![])); 
        }

        let mut element_entries: Vec<ElementEntryCommand> = vec![]; 
        let recognizers: Vec<&dyn CommandRecognizer> = vec![
            &ElementEntryCommandRecognizer{},
        ];

        let cmd_entries = get_commands(&statement.substatements, &recognizers, errs_acc); 
        for cmd_entry in &cmd_entries {
            match cmd_entry {
                Command::ElementEntry(c) => {
                    element_entries.push(c.clone()); 
                },  
                _ => {
                    // Adding error not nesessary because 
                    // function get_commands threw it
                },
            }
        }

        Command::Circle(CircleCommand{
            name: name, 
            param_names: params, 
            element_entries: element_entries, 
            statement: statement.clone(), 
        })
    }
}