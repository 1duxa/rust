pub mod mega_parser {
    use crate::lexer::lexer3000::*;

    #[derive(Debug)]
    pub struct Parser(pub Vec<Token>);

    #[derive(Debug, PartialEq)]
    pub enum Expr {
        Number(i32),
        Operation {
            left: Box<Expr>,
            op: Op,
            right: Box<Expr>,
        },
    }
    impl Expr {
        pub fn untangle(&self) -> i32 {
            match self {
                Expr::Number(num) => *num,
                Expr::Operation { left, op, right } => {
                    let left_val = left.untangle();
                    let right_val = right.untangle();
                    match op {
                        Op::Plus => left_val + right_val,
                        Op::Minus => left_val - right_val,
                        Op::Divide => left_val / right_val,
                        Op::Multiply => left_val * right_val,
                        Op::DividePercent => left_val % right_val,
                    }
                }
            }
        }
    }
    impl Parser {
        #[allow(unused)]
        pub fn parse(mut self) -> Option<Expr> {
            self.0.push(Token::Eof);
            let mut curr_op: Option<Op> = None;
            let mut curr_expr: Option<Expr> = None;
            let mut curr_num: Option<i32> = None;

            for token in self.0 {
                match token {
                    Token::Number(num) => {
                        curr_num = Some(num);
                        if let Some(left_expr) = curr_expr.take() {
                            if let Some(op) = curr_op.take() {
                                curr_expr = Some(Expr::Operation {
                                    right: Box::new(left_expr),
                                    op,
                                    left: Box::new(Expr::Number(num)),
                                });
                                curr_num = None;
                            }
                        } else if curr_expr.is_none() && curr_op.is_some() && curr_num.is_some() {
                            curr_expr = Some(Expr::Operation {
                                right: Box::new(Expr::Number(curr_num.unwrap())),
                                op: curr_op.unwrap(),
                                left: Box::new(Expr::Number(num)),
                            });
                            curr_num = None;
                        } else if curr_expr.is_none() {
                            curr_expr = Some(Expr::Number(num));
                        }
                    }
                    Token::Math(op) => {
                        curr_op = Some(op);
                    }
                    Token::Eof => {
                        println!("{:#?}", curr_expr.take().unwrap().untangle());
                    }
                    _ => (),
                }
            }

            curr_expr
        }
    }
}
