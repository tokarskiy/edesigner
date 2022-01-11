use crate::commands::Command;
use crate::entities::Statement; 
use crate::interfaces::CommandRecognizer; 
use crate::commands::ElementEntryCommand; 
use crate::keywords::keyword; 
use crate::commands::is_first_token_specific_word; 
use crate::commands::variant_eq; 
use crate::commands::find_token; 
use crate::entities::TokenType; 
use crate::commands::get_args; 
use crate::commands::get_commands; 
use crate::commands::ElementEntryCommandRecognizer; 
use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::keywords::is_keyword;

#[derive(Clone)]
pub struct ElementCommand {
    pub name: String, 
    pub param_names: Vec<String>,
    pub nodes: Vec<String>, 
    pub element_entries: Vec<ElementEntryCommand>, 
    pub statement: Statement, 
}

pub struct ElementCommandRecognizer{}
impl CommandRecognizer for ElementCommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command {
        let tokens = &statement.tokens;
        if !is_first_token_specific_word(tokens, keyword::ELEMENT) {
            return Command::None; 
        }
        
        let mut name: String = format!(""); 
        let len = tokens.len(); 
        let init_token = tokens[0].clone(); 
        if len < 6 { // element [ ] T ( )
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("{} [node1, node2, ...] ElementName(arg1, arg2, ...) {{ ... }} ", keyword::ELEMENT)])); 

            return Command::Element(ElementCommand{
                name: name, 
                nodes: vec![], 
                param_names: vec![], 
                element_entries: vec![], 
                statement: statement.clone(), 
            });
        }

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

        let nodes = get_args(tokens, start, end, false); 

        // 3 because name, '(' and ')'
        if end > len - 3 {
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("{} [node1, node2, ...] ElementName(arg1, arg2, ...) {{ ... }} ", keyword::ELEMENT)])); 
        }

        let name_index = end + 1;
        let name_token = tokens[name_index].clone(); 
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

        Command::Element(ElementCommand{
            name: name, 
            nodes: nodes, 
            param_names: params, 
            element_entries: element_entries, 
            statement: statement.clone(), 
        })
    }
}
