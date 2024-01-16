mod parser;

fn main() {
    let code = std::fs::read_to_string("code.wfl").unwrap();
    let intermediate_code = parser::parse_code(&code);
    println!("{:?}", intermediate_code);
}
