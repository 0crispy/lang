use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TokenInfo{
    pub token:Token,
    pub char_range:Range<usize>,
}
impl TokenInfo{
    fn new(token:Token, char_range:Range<usize>) -> Self{
        Self { token, char_range}
    } 
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Name(String),

    NewFunction,
    NewVariable,

    EndSentence,
    Colon,
    Comma,

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
    pub tokens:Vec<TokenInfo>,
    current_token:String,
    pub last_char:Range<usize>,
}
impl Tokens{
    fn new() -> Self{
        Self{
            tokens:Vec::new(),
            current_token:String::new(),
            last_char: Range::default(),
        }
    }
    fn name_to_token (token_str:String) -> Token{
        match token_str.as_str(){
            "fn" => Token::NewFunction,
            "let" => Token::NewVariable,
            _ => Token::Name(token_str)
        }
    }
    fn push_token_name(&mut self, char_id:usize){
        if !self.current_token.is_empty() {
            self.tokens.push(
                TokenInfo::new(
                    Tokens::name_to_token(self.current_token.clone()),
                    (char_id-self.current_token.clone().len())..char_id
                )
            );
            self.current_token.clear();
        }
    }
    fn push_token(&mut self, token:Token, char_id:usize){
        self.push_token_name(char_id);
        self.tokens.push(TokenInfo::new(token, char_id..char_id));
    }
}
#[derive(Debug)]
pub enum TokenizerError{
}

pub fn tokenize(input: &str) -> Result<Tokens, TokenizerError> {
    let mut tokens = Tokens::new();
    for (id,c) in input.chars().enumerate() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                tokens.current_token.push(c);
            },
            ' ' => {
                tokens.push_token_name(id);
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
                    ',' => Token::Comma,
                    _ => Token::None,
                };
                if t != Token::None{
                    tokens.push_token(t, id);
                }
            },
        }
        tokens.last_char = id..id;
    }
    tokens.push_token_name(tokens.last_char.end);
    Ok(tokens)
}