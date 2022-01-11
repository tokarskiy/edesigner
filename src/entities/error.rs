use crate::entities::Position; 

#[derive(Clone)]
pub struct Error<'a> {
    pub code: &'a str, 
    pub message: &'a str, 
}

#[derive(Clone)]
pub struct ErrorEntry {
    pub error: &'static Error<'static>, // ref to global errors objects
    pub position: Position,
    pub args: Vec<String>,
    pub file_name: String,
}

impl ErrorEntry {
    pub fn new(error: &'static Error, position: &Position, args: &Vec<String>) -> ErrorEntry {
        ErrorEntry {
            error: error,
            position: position.clone(),
            args: args.clone(),
            file_name: format!(""),
        }
    }

    //pub tokens: Vec<Token>,
    pub fn console_print(&self) {
        let file_name = match self.file_name == format!("") {
            true => format!("CODE"),
            false => self.file_name.clone(),   
        };

        println!("{} {}:", file_name, self.position);
        
        let mut i: usize = 1; 
        let mut msg: String = self.error.message.to_string(); 
        for arg in &self.args {
            let entry: String = format!("~{}~", i);
            msg = msg.replace(&entry, arg);
            //msg = std::str::replace(msg, "")
            i += 1;
        }

        println!("{}", msg); 
    }
}

#[allow(dead_code)]
pub fn apply_file_name(errs: &Vec<ErrorEntry>, file_name: &String) -> Vec<ErrorEntry> {
    errs.iter() 
        .map(|x| ErrorEntry{
            position: x.position.clone(),
            error: x.error,
            args: x.args.clone(),
            file_name: file_name.clone(),
        })
        .collect::<Vec<ErrorEntry>>()
}

pub mod errors {
    use crate::entities::Error;

    pub const UNDEFINED_SYMBOL: &'static Error = &Error{
        code: "STX001",
        message: "Undefined symbol: '~1~'. ", 
    };    

    pub const UNFINISHED_COMMAND: &'static Error = &Error{
        code: "STX002",
        message: "Unfinished command. Missing separator. "
    }; 

    pub const OPEN_CLOSED_BRACKETS: &'static Error = &Error{
        code: "STX003", 
        message: "Discrepancy in the number of open and closed brackets. "
    };

    pub const UNDEFINED_STATEMENT: &'static Error = &Error{
        code: "STX004",
        message: "Undefined statement: '~1~'. "
    };

    pub const NAME_KEYWORD: &'static Error = &Error{
        code: "STX005",
        message: "Expected unqualified-id, found '~1~'. ", 
    };

    pub const NO_SUBSTATEMENTS: &'static Error = &Error{
        code: "STX006",
        message: "This statement should have substatements. ", //TODO: Change
    };

    pub const ARE_SUBSTATEMENTS: &'static Error = &Error{
        code: "STX007",
        message: "This statement shouldn't have substatements. ", 
    };

    pub const WROND_COMMAND_SEMANTIC: &'static Error = &Error{
        code: "STX008", 
        message: "Wrong statement semantic. Expected: '~1~'. ",
    };

    pub const INVALID_NAME: &'static Error = &Error{
        code: "STX009", 
        message: "Invalid name: '~1~'. "
    }; 

    pub const UNEXPECTED_SYMBOL: &'static Error = &Error{
        code: "STX010",
        message: "Expected '~1~', found '~2~'. ",
    };

    pub const SYMBOL_NOT_FOUND: &'static Error = &Error{
        code: "STX011",
        message: "Symbol '~1~' not found. ",
    };

    pub const UNDEFINED_UNARY_OPERATOR: &'static Error = &Error{
        code: "STX012", 
        message: "Undefined unary operator '~1~'. "
    };

    pub const EXPRESSION_LAST_SYMBOL: &'static Error = &Error{
        code: "STX013",
        message: "Last element of expression can't be an operator. ", 
    };

    pub const EXPECTED_BINARY_OPERATOR: &'static Error = &Error{
        code: "STX014", 
        message: "Expected binary operator, found '~1~'. "
    };

    pub const EXPECTED_NUM_VAR: &'static Error = &Error{
        code: "STX015",
        message: "Expected number or variable, found '~1~'. "
    };

    pub const EXPRESSION_SHOULD_RET_NUMBER: &'static Error = &Error{
        code: "STX016",
        message: "Expression should return a number. ",
    };

    pub const NO_VAR_AFTER_UNARY: &'static Error = &Error{
        code: "STX017",
        message: "No statement after unary operator. "
    };
}