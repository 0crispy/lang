mod tokenizer;
use std::collections::HashMap;
use tokenizer::{Token,TokenizerError};

#[derive(Debug)]
pub enum ParseError{
    Tokenizer(TokenizerError),
    NoNameAfterFunction,
}
impl From<TokenizerError> for ParseError{
    fn from(error:TokenizerError) -> Self{
        ParseError::Tokenizer(error)
    }
}
#[derive(Debug)]
pub struct IntermediateCode{
    global_functions:HashMap<String,Function>,
}
#[derive(Debug)]
struct Function{
    scope:Scope,
    name:String,
    input:FunctionInput,
    output:Type,
    statement:Statement,
}
#[derive(Debug)]
struct FunctionInput{
    name:String,
    input_type:Type,
}
#[derive(Debug)]
struct Statement{
    contents:Box<StatementContents>,
    next:Option<Box<Statement>>,
}
#[derive(Debug)]
enum StatementContents{
    NewVariable{
        var:Variable,
        value:Statement,
    }
}
#[derive(Debug)]
struct Variable{
    scope:Scope,
    name:String,
}
#[derive(Debug)]
struct Scope;
#[derive(Debug)]
struct Type;

pub fn parse_code(code:&str) -> Result<IntermediateCode, ParseError>{
    let mut tokens = tokenizer::tokenize(&code)?;
    println!("{:?}", tokens);
    
    let mut tokens = tokens.into_iter();
    loop{
        let token = if let Some(token) = tokens.next() {
            token
        }else {break;};
        match token{
            Token::NewFunction => {
                //now we expect the name of the function:
                if let Some(Token::Name(name)) = tokens.next(){

                }
                else{
                    return Err(ParseError::NoNameAfterFunction);
                }
            },
            _ => {}
        }
    }
    Ok(IntermediateCode{
        global_functions:HashMap::new()
    })
} 