use crate::commands::Command;
use crate::entities::Statement; 
use crate::interfaces::CommandRecognizer; 
use crate::commands::is_first_token_specific_word; 
use crate::entities::TokenType; 
use crate::keywords::keyword; 
use crate::keywords::is_keyword; 
use crate::entities::ErrorEntry;
use crate::entities::errors;

#[derive(Clone)]
pub struct ImportCommand {
    pub statement: Statement, 
    pub package_name: String, 
}

pub struct ImportCommandRecognizer{}
impl CommandRecognizer for ImportCommandRecognizer {
    fn from_statement(&self, statement: &Statement, errs_acc: &mut Vec<ErrorEntry>) -> Command {
        let tokens = &statement.tokens;
        if !is_first_token_specific_word(&tokens, keyword::IMPORT) {
            return Command::None; 
        }

        let init_token = tokens[0].clone();
        if tokens.len() != 2 {
            errs_acc.push(ErrorEntry::new(
                errors::WROND_COMMAND_SEMANTIC, 
                &init_token.position, 
                &vec![format!("{} packageName; ", keyword::IMPORT)])); 
        }
        
        if statement.substatements.len() > 0 {
            errs_acc.push(ErrorEntry::new(
                errors::ARE_SUBSTATEMENTS, 
                &init_token.position, 
                &vec![])); 
        }

        let mut name: String = format!(""); 
        if tokens.len() > 1 {
            let name_token = tokens[1].clone(); 

            match name_token.token_type {
                TokenType::Word => { 
                    name = name_token.value.clone(); 
                    if is_keyword(&name) {
                        errs_acc.push(ErrorEntry::new(
                            errors::NAME_KEYWORD, 
                            &init_token.position, 
                            &vec![name.clone()])); 
                    }
                }
                _ => { 
                    errs_acc.push(ErrorEntry::new(
                        errors::INVALID_NAME, 
                        &init_token.position, 
                        &vec![name.clone()]));
                }
            }
        }
        
        Command::Import(ImportCommand {
            package_name: name, 
            statement: statement.clone(),
        })
    }
}
