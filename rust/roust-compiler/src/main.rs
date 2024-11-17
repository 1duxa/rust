mod lexer;

use lexer::lexer3000::MegaLexer3000;
mod parser;
use parser::mega_parser::*;
fn main() {
    let tokens = MegaLexer3000("1234+34555-523".to_string()).lexify();
    println!("{:?}", tokens);
    Parser(tokens).parse();
}
