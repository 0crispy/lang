mod tokenizer;
use std::{collections::HashMap, ops::Range, sync::Mutex, hash::Hash, iter::Peekable, vec::IntoIter};
use tokenizer::{Token,TokenizerError};

use self::tokenizer::{TokenInfo, Tokens};

#[derive(Debug)]
pub enum ParseError{
    Tokenizer(TokenizerError),
    InvalidToken(Range<usize>, String),
    Placeholder,
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
    input:Vec<FunctionInput>,
    output:Option<Type>,
    block:Block,
}
#[derive(Debug)]
struct FunctionInput{
    name:String,
    input_type:Type,
}
#[derive(Debug)]
struct Block{
    statements:Vec<Statement>,
}
impl Block{
    fn placeholder() -> Self{
        Self{
            statements: Vec::new(),
        }
    }
}
#[derive(Debug)]
enum Statement{
    None,
    NewVariable{
        var:Variable,
        value:Block,
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
enum Type{
    TypeName(String),
    TypeTraitName(String),
}

pub fn parse_code(code:&str) -> Result<IntermediateCode, ParseError>{
    let mut parser = Parser::new(code)?;
    parser.parse()?;


    Ok(parser.intermediate)
} 
struct Parser{
    tokens:Mutex<Peekable<IntoIter<TokenInfo>>>,
    tokens_info:Tokens,
    intermediate:IntermediateCode,
}
impl Parser{
    fn new(code:&str) -> Result<Self,ParseError>{
        let tokens_info = tokenizer::tokenize(&code)?;
        println!("{:?}", tokens_info.tokens);
        let tokens = Mutex::new(tokens_info.tokens.clone().into_iter().peekable());
        let intermediate = IntermediateCode{
            global_functions: HashMap::new(),
        };
        Ok(Parser { tokens, tokens_info, intermediate })
    }
    fn parse(&mut self) -> Result<(),ParseError>{
        
        loop{
            let token = if let Some(token) = self.tokens.lock().unwrap().next(){
                token
            }
            else{
                break;
            };

            match token.token{
                Token::NewFunction => {
                    let function = self.parse_function()?;
                    self.intermediate.global_functions.insert(function.name.clone(), function);
                },
                _ => {}
            }
        }
        Ok(())
    }
    fn parse_block(&self) -> Result<Block,ParseError>{
        let token = self.next_token("Expected start of block")?;
        let mut contents:Vec<Statement> = Vec::new();
        match token.token{
            Token::NewVariable => {
                let var_name = self.parse_name("Expected variable name")?;
                self.expect_token(Token::Eq, "Expected '='")?;
                //TODO
            }
            Token::Name(name) => {
                let token = self.next_token("Expected SOMETHING")?;
                //maybe a function?
                if token.token == Token::LeftParen{
                    //indeed, a function.
                    
                }
                //TODO
            }
            _ => {}
        }
        Err(ParseError::Placeholder)
    }
    fn parse_function(&self) -> Result<Function,ParseError>{
        let fn_name = self.parse_name("Expected function name")?;
        self.expect_token(Token::LeftParen, "Expected left paren")?;
        let mut fn_arguments = Vec::new();
        loop{
            let token = self.next_token("Expected ')' or function argument name")?;
            if let Token::RightParen = token.token{
                break;
            }
            let var_name = if let Token::Name(var_name) = token.token{var_name}else{
                return Err(ParseError::InvalidToken(token.char_range, "Expected ')' or function argument name".to_string()))
            };
            self.expect_token(Token::Colon, "Expected ':' ")?;
            let argument_type = self.parse_type()?;

            fn_arguments.push(FunctionInput { name: var_name, input_type:argument_type });
            let token = self.next_token("Expected ')' or ','")?;
            if token.token == Token::RightParen{
                break;
            }
            if token.token == Token::Comma{
                self.next_token("Expected function argument name")?;
            }
        }
        self.expect_token(Token::LeftBraces,"Expected '{'")?;            
        let block = self.parse_block()?;
        self.expect_token(Token::RightBraces, "Expected '}'")?;
        Ok(Function{
            scope: Scope,
            name: fn_name,
            input: fn_arguments,
            output: None,
            block,
        })
    }
    fn next_token(&self, unwrap_msg:&str) -> Result<TokenInfo, ParseError>{
        if let Some(token) = self.tokens.lock().unwrap().next(){
            Ok(token.clone())
        }
        else{
            return Err(ParseError::InvalidToken(self.tokens_info.last_char.clone(),unwrap_msg.to_string()));
        }
    }
    fn expect_token(&self, check_token:Token, unwrap_msg:&str) -> Result<(),ParseError>{
        let token = self.next_token(unwrap_msg)?;
        if token.token != check_token{
            return Err(ParseError::InvalidToken(token.char_range, unwrap_msg.to_string()))
        }
        Ok(())
    }
    fn parse_name(&self, unwrap_msg:&str) -> Result<String,ParseError>{
        let token = self.next_token(unwrap_msg)?;
        if let Token::Name(name) = token.token{
            Ok(name)
        }
        else{
            Err(ParseError::InvalidToken(token.char_range, unwrap_msg.to_string()))
        }
    }
    fn parse_type(&self) -> Result<Type,ParseError>{
        let mut token = self.next_token("Expected type ")?;
        if token.token == Token::SingleQuote{
            token = self.next_token("Expected type ")?;
            let var_trait_type = if let Token::Name(var_type) = token.token.clone(){var_type}
            else{return Err(ParseError::InvalidToken(token.char_range,"Expected trait".to_string()))};
            return Ok(Type::TypeTraitName(var_trait_type));
        }
        else{
            let var_type = if let Token::Name(var_type) = token.token.clone(){var_type}
            else{return Err(ParseError::InvalidToken(token.char_range,"Expected type ".to_string()))};
            return Ok(Type::TypeName(var_type));
        }
    }
}
