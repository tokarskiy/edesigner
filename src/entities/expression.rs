use crate::entities::Token;
use crate::commands::variant_eq;
use crate::entities::TokenType; 
use std::collections::HashMap; 
use std::fmt::Formatter;
use std::fmt::Display; 
use crate::entities::ErrorEntry;
use crate::entities::errors;
use crate::entities::Position;
use std::f64;

#[derive(Clone)]
pub struct Expression {
    pub tokens: Vec<Token>,
    root: Option<Node>, 
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.root.clone() {
            Option::None => write!(f, "[NULL EXPRESSION]"),
            Option::Some(node) => write!(f, "{}", node)
        }
    }
}

#[derive(Clone)]
enum Node {
    Number(String), 
    Operator(OperatorNode),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = node_to_str(self);
        write!(f, "{}", string)
    }
}

fn node_to_str(node: &Node) -> String {
    match node {
        Node::Number(num) => { format!("{}", num) }
        Node::Operator(op_node) => {
            let mut res = op_node.name.clone(); 
            let mut next = false; 
            
            res.push_str("["); 
            for inner_node in &op_node.nodes {
                let string = node_to_str(&inner_node); 
                if next {
                    res.push_str(" "); 
                }
                else {
                    next = true; 
                }

                res.push_str(format!("{}", string).as_str()); 
            }
            res.push_str("]"); 

            res
        },
    }
}

struct OperatorNode {
    name: String, 
    nodes: Vec<Node>,
}

impl Clone for OperatorNode {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            nodes: self.nodes.to_vec(), 
        }
    }
}

impl Expression {
    #[allow(dead_code)]
    pub fn evaluate(&self, input: &HashMap<String, f64>) -> Option<f64> {
        match self.root.clone() {
            Option::Some(node) => {
                evaluate_node(&node, input)
            },
            Option::None => Option::None,
        }
    }
}

#[allow(dead_code)]
fn evaluate_node(node: &Node, input: &HashMap<String, f64>) -> Option<f64> {
    match node {
        Node::Number(num_str) => {
            let is_num = num_str.parse::<f64>();
            match is_num {
                Ok(num) => Option::Some(num),
                Err(_) => {
                    let get = input.get(num_str)?;
                    Option::Some(*get)
                }
            }
        },
        Node::Operator(operator) => {
            let nums: Vec<Option<f64>> = operator.nodes.iter()
                .map(|x| evaluate_node(x, input))
                .collect(); 
            
            if nums.contains(&Option::None) {
                Option::None 
            }
            else {
                let func = get_func(&operator.name)?; 
                let nums: Vec<f64> = nums.iter() 
                    .map(|x| x.expect(""))
                    .collect(); 

                if nums.len() != func.args_count {
                    Option::None
                }   
                else {
                    let f = func.lambda;
                    f(&nums)
                }
            }   
        },
    }
}  

/// Returns the lambda and amount of arguments 
fn get_func(name: &String) -> Option<Function> {
    let search_result = STANDART_FUNCTIONS
        .binary_search_by(|&f| name.cmp(&String::from(f.name)));
    
    match search_result {
        Result::Ok(index) => {
            let func = STANDART_FUNCTIONS[index];

            Option::Some(func.clone())
        },
        Result::Err(_) => Option::None,
    }
}

#[derive(Clone)]
struct Function<'a> {
    name: &'a str,
    args_count: usize,
    lambda: &'a dyn Fn(&Vec<f64>) -> Option<f64>
}

const STANDART_FUNCTIONS: &'static [&'static Function] = &[
    
];

#[derive(Clone)]
enum ExpressionElement {
    Postfix(Vec<Token>, Option<Token>),
    Common(Token), 
}

impl ExpressionElement {
    fn get_first_token(&self) -> Option<Token> {
        match self {
            ExpressionElement::Common(token) => Option::Some(token.clone()), 
            ExpressionElement::Postfix(tokens, ftoken) => {
                match ftoken {
                    Option::None => {
                        match tokens.first() {
                            Option::Some(token) => Option::Some(token.clone()),
                            Option::None => Option::None,
                        }
                    },
                    Option::Some(token) => Option::Some(token.clone()),
                }        
            }
        }
    }
}

fn get_expression_elems(tokens: &Vec<Token>, errs_acc: &mut Vec<ErrorEntry>) -> Vec<ExpressionElement> {
    let mut level: i32 = 0; 
    let mut level_tokens: Vec<Token> = vec![]; 
    let mut exprs: Vec<ExpressionElement> = vec![]; 
    let mut fn_call: Option<Token> = Option::None; 
    let mut i: usize = 0; 

    for token in tokens {
        if level < 0 {
            errs_acc.push(ErrorEntry::new(
                errors::OPEN_CLOSED_BRACKETS, 
                &token.position, 
                &vec![])); 
        }

        if variant_eq(&token.token_type, &TokenType::Symbol) && token.value == "(" {
            if level > 0 {
                level_tokens.push(token.clone()); 
            } 
            else {
                if level == 0 {
                    fn_call = match tokens.get(i - 1) { // prev name is word => it's a function call
                        Option::None => Option::None,
                        Option::Some(x) if variant_eq(&TokenType::Word, &x.token_type) => {
                            exprs.pop(); 
                            Option::Some(x.clone())
                        },
                        _ => Option::None,
                    };
                }
            }

            level += 1; 
        }
        else if variant_eq(&token.token_type, &TokenType::Symbol) && token.value == ")" {
            if level > 1 {
                level_tokens.push(token.clone()); 
            }   

            level -= 1;

            if level == 0 {
                exprs.push(ExpressionElement::Postfix(level_tokens.clone(), fn_call.clone())); 
                fn_call = Option::None; 
                level_tokens = vec![]; 
            } 
        }
        else {
            match level > 0 {
                true => {
                    level_tokens.push(token.clone()); 
                },
                false => {
                    exprs.push(ExpressionElement::Common(token.clone())); 
                }
            }
        }

        i += 1; 
    }

    exprs
}

// Checking levels isn't nesessary because we dealing with ExpressionElements
fn comma_divide(exprs: &Vec<ExpressionElement>) -> Vec<Vec<ExpressionElement>> {
    let mut res: Vec<Vec<ExpressionElement>> = vec![]; 
    let mut curr: Vec<ExpressionElement> = vec![]; 
    for expr in exprs {
        match expr {
            ExpressionElement::Common(token) => {
                if variant_eq(&token.token_type, &TokenType::Symbol) && token.value == "," {
                    res.push(curr.clone()); 
                    curr = vec![]; 
                }
                else {
                    curr.push(expr.clone());
                }
            },
            ExpressionElement::Postfix(_, _) => {
                curr.push(expr.clone());
            },
        }
    }

    if curr.len() > 0 {
        res.push(curr.clone()); 
    }
    
    res
}

fn check_expressions_order(elems: &Vec<ExpressionElement>, errs_acc: &mut Vec<ErrorEntry>) -> Option<usize> {
    let mut init: usize = 0;
    let mut i: usize = 0; 
    let mut first: bool = true;
    let len = elems.len(); 
    if len == 0 {
        return Option::Some(0); 
    }

    for elem in elems {
        match elem {
            ExpressionElement::Common(token) => {
                if variant_eq(&token.token_type, &TokenType::Symbol) {
                    if first {
                        first = false; 
                        init = 1; 
                        if !(token.value == "+" || token.value == "-") { // TODO: Fix this
                            errs_acc.push(ErrorEntry::new(
                                errors::UNDEFINED_UNARY_OPERATOR,
                                &token.position,
                                &vec![token.value.clone()],
                            ));
                        }
                    }

                    if i == elems.len() {
                        errs_acc.push(ErrorEntry::new(
                            errors::EXPRESSION_LAST_SYMBOL,
                            &token.position, 
                            &vec![], 
                        ));
                        return Option::Some(i); 
                    }

                    if (init + i) % 2 != 1 {
                        errs_acc.push(ErrorEntry::new(
                            errors::EXPECTED_BINARY_OPERATOR,
                            &token.position,
                            &vec![token.value.clone()], 
                        ));

                        return Option::Some(i); 
                    }
                }
                else {
                    if first {
                        first = false; 
                    }

                    if (init + i) % 2 != 0 {
                        errs_acc.push(ErrorEntry::new(
                            errors::EXPECTED_NUM_VAR,
                            &token.position,
                            &vec![token.value.clone()], 
                        ));

                        return Option::Some(i); 
                    }
                }
            },
            ExpressionElement::Postfix(_, _) => {
                if first {
                    first = false; 
                }

                if (init + i) % 2 != 0 {
                    return Option::Some(i); 
                }
            },
        }

        i += 1; 
    }

    let last = elems[len - 1].clone(); 
    match last {
        ExpressionElement::Common(token) => {
            if variant_eq(&token.token_type, &TokenType::Symbol) {
                errs_acc.push(ErrorEntry::new(
                    errors::EXPRESSION_LAST_SYMBOL,
                    &token.position.clone(),
                    &vec![], 
                ));
            }
        },
        _ => { },
    }

    Option::None
}

struct Operator<'a> {
    text: &'a str,
}

static BIN_OPERATORS: &'static [&'static [&'static Operator]; 4] = & [
    &[&Operator{ text: "==" }, &Operator{ text: "!=" }],
    &[&Operator{ text: "<=" }, &Operator{ text: "<" },
      &Operator{ text: ">=" }, &Operator{ text: ">" }],
    &[&Operator{ text: "+" }, &Operator{ text: "-" }],
    &[&Operator{ text: "*" }, &Operator{ text: "/" }],
];

static UN_OPERATORS: &'static [&'static Operator] = &[
    &Operator{ text: "+" }, &Operator{ text: "-" }
];

fn process_postfix(exprs: &Vec<ExpressionElement>, func_call: Option<Token>, errs_acc: &mut Vec<ErrorEntry>) -> Node {
    match func_call {
        Option::None => {
            match check_expressions_order(exprs, errs_acc) {
                Option::Some(_) => { return Node::Number(format!("0")); },
                Option::None => { },
            };
        },
        _ => { },
    };
    
    let div = comma_divide(exprs); 
    match func_call {
        Option::None => {
            let len = div.len(); 
            match len {
                0 => {
                    Node::Number(format!("0"))
                },
                1 => {
                    divide_elems_by_first_priority_operator(exprs, errs_acc)
                },
                _ => {
                    let elem = exprs[0].clone();
                    errs_acc.push(ErrorEntry::new(
                        errors::EXPRESSION_SHOULD_RET_NUMBER,
                        &match elem.get_first_token() {
                            Option::Some(token) => token.position.clone(),
                            Option::None => Position { line: 0, line_position: 0 },
                        },
                        &vec![],
                    ));

                    Node::Number(format!("0"))
                }
            }
        },
        Option::Some(func_call) => {
            let nodes: Vec<Node> = div.iter()
                .map(|x| divide_elems_by_first_priority_operator(x, errs_acc))
                .collect(); 

            Node::Operator(OperatorNode{
                name: func_call.value.clone(),
                nodes: nodes, 
            })
        }
    }
}

fn divide_elems_by_first_priority_operator(elems: &Vec<ExpressionElement>, errs_acc: &mut Vec<ErrorEntry>) -> Node {
    // Check bin operators
    for &operator_priority_vec in BIN_OPERATORS {
        for &operator in operator_priority_vec {
            let op_text = &operator.text.to_string(); 
            let search_result = elems.iter()
                .skip(1)
                .position(|x| match x {
                    ExpressionElement::Common(Token { value: val, position: _, token_type: TokenType::Symbol }) if val == op_text => true, 
                    _ => false,
                }); 
            
            match search_result {
                Option::Some(search_position) => { 
                    return divide_elems_by_bin_operator(elems, &operator, search_position + 1, errs_acc); 
                },
                Option::None => { }
            }
        }
    }
        
    // Check un operators
    for &operator in UN_OPERATORS {
        let op_text = operator.text.to_string(); 
        let first = elems[0].clone(); 
        
        match first {
            ExpressionElement::Common(token) => {
                if variant_eq(&TokenType::Symbol, &token.token_type) && token.value == op_text {
                    return match divide_elems_by_un_operator(elems, &operator, errs_acc) {
                        Option::Some(x) => x,
                        Option::None => {
                            errs_acc.push(ErrorEntry::new(
                                errors::NO_VAR_AFTER_UNARY,
                                &token.position.clone(), 
                                &vec![],
                            ));
                            Node::Number(format!("0"))
                        }
                    }; 
                }
            }, 
            _ => { },
        }; 
    }

    let elems_len = elems.len(); 
    if elems_len == 0 {
        // TODO:
    }

    // It may be function call of single value, size of elems should be 1
    if elems_len > 1 {
        let elem = elems[0].clone(); 
        errs_acc.push(ErrorEntry::new(
            errors::EXPRESSION_SHOULD_RET_NUMBER,
            &match elem.get_first_token() {
                Option::Some(token) => token.position.clone(),
                Option::None => Position { line: 0, line_position: 0 },
            },
            &vec![], 
        ));
    }

    let elem = elems[0].clone(); 
    match elem {
        ExpressionElement::Common(token) => {
            match token.token_type {
                TokenType::Word | TokenType::Number => {
                    Node::Number(token.value)
                },
                _ => {
                    errs_acc.push(ErrorEntry::new(
                        errors::EXPECTED_NUM_VAR,
                        &token.position.clone(), 
                        &vec![token.value.clone()],
                    ));
                    Node::Number(format!("0"))
                }
            }
        },
        ExpressionElement::Postfix(tokens, fn_call) => {
            let elems = get_expression_elems(&tokens, errs_acc); 

            process_postfix(&elems, fn_call.clone(), errs_acc)
        }
    }
}

fn divide_elems_by_un_operator(elems: &Vec<ExpressionElement>, operator: &Operator, errs_acc: &mut Vec<ErrorEntry>) -> Option<Node> {
    // It's an operator call  
    let right: Vec<_> = elems.iter()
        .skip(1)
        .map(|x| x.clone())
        .collect(); 

    if right.len() == 0 {
        return Option::None; 
    }

    let nodes = vec![right].iter()
        .map(|x| divide_elems_by_first_priority_operator(x, errs_acc))
        .collect(); 
    
    Option::Some(Node::Operator(OperatorNode{
        name: operator.text.to_string(),
        nodes: nodes
    }))
}

fn divide_elems_by_bin_operator(elems: &Vec<ExpressionElement>, operator: &Operator, operators_index: usize, errs_acc: &mut Vec<ErrorEntry>) -> Node {
    // It's an operator call
    let left: Vec<_> = elems.iter()
        .take(operators_index)
        .map(|x| x.clone())
        .collect(); 
    
    let right: Vec<_> = elems.iter()
        .skip(operators_index + 1)
        .map(|x| x.clone())
        .collect(); 

    let nodes = vec![left, right].iter()
        .map(|x| divide_elems_by_first_priority_operator(x, errs_acc))
        .collect(); 
    
    Node::Operator(OperatorNode{
        name: operator.text.to_string(),
        nodes: nodes
    })
}

pub fn get_expression(tokens: &Vec<Token>, errs_acc: &mut Vec<ErrorEntry>) -> Option<Expression> {
    if tokens.len() == 0 {
        return Option::None; 
    }

    let elems = get_expression_elems(tokens, errs_acc);   
    match check_expressions_order(&elems, errs_acc) {
        Option::None => { },
        Option::Some(_) => { return Option::None; },
    }
    let root = divide_elems_by_first_priority_operator(&elems, errs_acc); 

    Option::Some(Expression{
        tokens: tokens.clone(),
        root: Option::Some(root),
    })
}


