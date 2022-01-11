mod entities;
mod tokenizer; 
mod tokens_recognizers;
mod interfaces; 
mod statement_divider; 
mod commands; 
mod keywords; 

use commands::Command; 
use entities::Statement; 
use interfaces::TokenRecognizer;
use interfaces::CommandRecognizer; 
use tokens_recognizers::BlockCommentTokenRecognizer;
use tokens_recognizers::LineCommentTokenRecognizer;
use tokens_recognizers::NumberTokenRecognizer;
use tokens_recognizers::SymbolTokenRecognizer;
use tokens_recognizers::WordTokenRecognizer; 
use tokenizer::tokenize; 
use statement_divider::get_statements; 
use commands::get_commands; 
use entities::ErrorEntry; 
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("Please write a file name. "); 
        return; 
    }

    let file_name: &String = &args[1].clone(); 
    let code: String = match fs::read_to_string(file_name) {
        Result::Ok(text) => text,
        Result::Err(_) => {
            println!("Error reading file. "); 
            return; 
        }
    };

    let token_recognizers: Vec<&dyn TokenRecognizer> = vec!(
        &LineCommentTokenRecognizer{},
        &BlockCommentTokenRecognizer{},
        &SymbolTokenRecognizer{},
        &WordTokenRecognizer{},
        &NumberTokenRecognizer{},
    ); 

    let mut errors: Vec<ErrorEntry> = vec![]; 
    let tokens = tokenize(code, &token_recognizers, &mut errors); 
    let stats = get_statements(&tokens, &mut errors); 

    for error in &errors {
        error.console_print(); 
    }

    if errors.len() > 0 {
        return; 
    }

    //print_statements(&stats);

    let command_recognizers: Vec<&dyn CommandRecognizer> = vec![
        &commands::CircleCommandRecognizer{},
        &commands::ElementCommandRecognizer{},
        &commands::PackageCommandRecognizer{},
        &commands::ImportCommandRecognizer{}, 
        &commands::FunctionCommandRecognizer{}, 
    ]; 

    let mut errors: Vec<ErrorEntry> = vec![]; 
    let commands = get_commands(&stats, &command_recognizers, &mut errors);

    for error in &errors {
        error.console_print(); 
    }

    if errors.len() > 0 {
        return; 
    }

    for command in commands {
        match command {
            Command::Package(c) => { 
                println!("Package name: {}", c.name); 
            },
            Command::Import(c) => { 
                println!("Imports {}", c.package_name);
            }, 
            Command::Circle(c) => {
                println!("Circle"); 
                println!("  Name: {}", c.name);
                if c.param_names.len() > 0 {
                    println!("  Parameters: "); 
                    for param in &c.param_names {
                        println!("    {}", param); 
                    }
                } 
                else {
                    println!("  No parameters"); 
                }

                if c.element_entries.len() > 0 {
                    println!("  Element entries: "); 
                    for elem in &c.element_entries {
                        println!("    {}", elem.element_name); 
                        
                        println!("      Name: {}", elem.entry_name); 
                        println!("      Parameter expressions ({}): ", elem.param_expressions.len()); 
                        for param in &elem.param_expressions {
                            print!("        {}", param);
                            //for token in &param.tokens {
                            //    print!("{} ", token.value); 
                            //} 
                            println!("");
                        }

                        println!("      Nodes: "); 
                        for node in &elem.nodes {
                            println!("        {}", node); 
                        }
                    }
                } 
                else {
                    println!("  No element entries");  
                }
            },
            Command::Element(c) => {
                println!("Element"); 
                println!("  Name: {}", c.name);
                if c.param_names.len() > 0 {
                    println!("  Parameters: "); 
                    for param in &c.param_names {
                        println!("    {}", param); 
                    }
                } 
                else {
                    println!("  No parameters"); 
                }

                if c.nodes.len() > 0 {
                    println!("  Nodes: "); 
                    for node in &c.nodes {
                        println!("    {}", node); 
                    }
                } 
                else {
                    println!("  No nodes"); 
                }

                if c.element_entries.len() > 0 {
                    println!("  Element entries: "); 
                    for elem in &c.element_entries {
                        println!("    {}", elem.element_name); 
                        
                        println!("      Name: {}", elem.entry_name); 
                        println!("      Parameter expressions ({}): ", elem.param_expressions.len()); 
                        for param in &elem.param_expressions {
                            print!("        {}", param);
                            //for token in &param.tokens {
                            //    print!("{} ", token.value); 
                            //} 
                            println!("");
                        }

                        println!("      Nodes: "); 
                        for node in &elem.nodes {
                            println!("        {}", node); 
                        }
                    }
                } 
                else {
                    println!("  No element entries");  
                }
            },
            Command::Function(c) => {
                println!("Function"); 
                println!("  Name: {}", c.name);
                if c.param_names.len() > 0 {
                    println!("  Parameters: "); 
                    for param in &c.param_names {
                        println!("    {}", param); 
                    }
                } 

                println!("  Expression: {}", c.expression); 
            }
            _ => { }  
        }
    }
}

fn _print_statements(statements: &Vec<Statement>) {
    println!("TOTAL STATEMENTS: {}", statements.len());

    let mut i = 0; 
    for statement in statements {
        println!("STATEMENT #{}", i + 1); 
        for token in &statement.tokens {
            println!("    {0: <10} {1: <10} {2: <10}", token.value, token.position, token.token_type);
        }

        let mut j = 0;
        for substatement in &statement.substatements {
            println!("    SUBSTATEMENT #{}.{}", i + 1, j + 1); 
            for token in &substatement.tokens {
                println!("        {0: <10} {1: <10} {2: <10}", token.value, token.position, token.token_type);
            }

            j += 1;
        }

        println!(""); 
        i += 1; 
    } 
}

