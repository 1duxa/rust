#[allow(dead_code)]
pub mod lexer3000 {

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Number(i32),
        Ident(String),
        Char(char),
        Math(Op),
        Logical(UnaryOp),
        Brace(Brace),
        Bool(BoolBit),
        Misc(Other),
        KeyWord(Key),
        Eof,
    }
    #[derive(Debug, PartialEq)]
    pub enum BoolBit {
        True = 0b00000000,
        False = 0b00000001,
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Op {
        Plus,
        Minus,
        Divide,
        Multiply,
        DividePercent,
    }
    #[derive(Debug, PartialEq)]
    pub enum UnaryOp {
        Or,
        Not,
        And,
    }
    #[derive(Debug, PartialEq)]
    pub enum Brace {
        NormalOpen,
        NormalClose,
        CurvyOpen,
        CurvyClose,
        SquareOpen,
        SquareClose,
    }
    #[derive(Debug, PartialEq)]
    pub enum Key {
        In,
        Fn,
        Let,
        Question,
    }
    #[derive(Debug, PartialEq)]
    pub enum Other {
        Colon,
        SemiColon,
        Dot,
        Coma,
        Quotation,
        SingleQuot,
        Equality,
    }

    #[derive(Debug)]
    pub struct MegaLexer3000(pub String);

    impl MegaLexer3000 {
        pub fn lexify(&self) -> Vec<Token> {
            let mut tokens: Vec<Token> = Vec::new();
            let mut curr_number: i32 = 0;
            let mut curr_string: String = "".to_string();
            for zaza in self.0.chars() {
                if !zaza.is_numeric() && curr_number != 0 {
                    tokens.push(Token::Number(curr_number));
                    curr_number = 0;
                }
                if !zaza.is_alphabetic() && !curr_string.is_empty() {
                    match curr_string.as_str() {
                        "in" => tokens.push(Token::KeyWord(Key::In)),
                        "?" => tokens.push(Token::KeyWord(Key::Question)),
                        "fn" => tokens.push(Token::KeyWord(Key::Fn)),
                        "let" => tokens.push(Token::KeyWord(Key::Let)),
                        "true" => tokens.push(Token::Bool(BoolBit::True)),
                        "false" => tokens.push(Token::Bool(BoolBit::False)),
                        _ => tokens.push(Token::Ident(curr_string)),
                    }
                    curr_string = "".to_string();
                }
                match zaza {
                    ' ' | '\n' => continue,
                    '+' => tokens.push(Token::Math(Op::Plus)),
                    '-' => tokens.push(Token::Math(Op::Minus)),
                    '/' => tokens.push(Token::Math(Op::Divide)),
                    '*' => tokens.push(Token::Math(Op::Multiply)),
                    '%' => tokens.push(Token::Math(Op::DividePercent)),
                    '(' => tokens.push(Token::Brace(Brace::NormalOpen)),
                    ')' => tokens.push(Token::Brace(Brace::NormalClose)),
                    '{' => tokens.push(Token::Brace(Brace::CurvyOpen)),
                    '}' => tokens.push(Token::Brace(Brace::CurvyClose)),
                    '[' => tokens.push(Token::Brace(Brace::SquareOpen)),
                    ']' => tokens.push(Token::Brace(Brace::SquareClose)),
                    '&' => tokens.push(Token::Logical(UnaryOp::And)),
                    '|' => tokens.push(Token::Logical(UnaryOp::Or)),
                    '!' => tokens.push(Token::Logical(UnaryOp::Not)),
                    ',' => tokens.push(Token::Misc(Other::Coma)),
                    '.' => tokens.push(Token::Misc(Other::Dot)),
                    ';' => tokens.push(Token::Misc(Other::SemiColon)),
                    ':' => tokens.push(Token::Misc(Other::SemiColon)),
                    '"' => tokens.push(Token::Misc(Other::Quotation)),
                    '\'' => tokens.push(Token::Misc(Other::SingleQuot)),
                    '=' => tokens.push(Token::Misc(Other::Equality)),
                    '0'..='9' => {
                        curr_number = curr_number * 10 + zaza.to_digit(10).unwrap() as i32;
                    }
                    'a'..='z' | 'A'..='Z' => {
                        curr_string.push(zaza);
                    }
                    _ => panic!("something wrong"),
                }
            }
            if curr_number != 0 {
                tokens.push(Token::Number(curr_number));
            }

            if !curr_string.is_empty() {
                match curr_string.as_str() {
                    "in" => tokens.push(Token::KeyWord(Key::In)),
                    "?" => tokens.push(Token::KeyWord(Key::Question)),
                    "fn" => tokens.push(Token::KeyWord(Key::Fn)),
                    "let" => tokens.push(Token::KeyWord(Key::Let)),
                    _ => tokens.push(Token::Ident(curr_string)),
                }
            }
            tokens
        }
    }
}
