#[derive(Debug, PartialEq)]
pub enum Token{
    Name(String),

    NewFunction,
    NewVariable,

    EndSentence,
    Colon,

    LeftParen,
    RightParen,
    LeftBraces,
    RightBraces,

    SingleQuote,

    Eq,
    Plus,
    Minus,

    None, //temporary token

}
#[derive(Debug)]
pub struct Tokens{
    tokens:Vec<Token>,
    current_token:String,
}
impl Tokens{
    fn new() -> Self{
        Self{
            tokens:Vec::new(),
            current_token:String::new(),
        }
    }
    fn name_to_token (token_str:String) -> Token{
        match token_str.as_str(){
            "fn" => Token::NewFunction,
            "let" => Token::NewVariable,
            _ => Token::Name(token_str)
        }
    }
    fn push_token_name(&mut self){
        if !self.current_token.is_empty() {
            self.tokens.push(Tokens::name_to_token(self.current_token.clone()));
            self.current_token.clear();
        }
    }
    fn push_token(&mut self, token:Token){
        self.push_token_name();
        self.tokens.push(token);
    }
}
#[derive(Debug)]
pub enum TokenizerError{
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Tokens::new();

    for (_id,c) in input.chars().enumerate() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                tokens.current_token.push(c);
            },
            ' ' => {
                tokens.push_token_name();
            },
            token => {
                let t = match token{
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '{' => Token::LeftBraces,
                    '}' => Token::RightBraces,
                    '=' => Token::Eq,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    ':' => Token::Colon,
                    ';' => Token::EndSentence,
                    '\'' => Token::SingleQuote,
                    _ => Token::None,
                };
                if t != Token::None{
                    tokens.push_token(t);
                }
            },
        }
    }
    tokens.push_token_name();
    Ok(tokens.tokens)
}