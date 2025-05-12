use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::expr::visit::Visitor;
use crate::token::{Token, TokenVal};

mod visit {
    use crate::expr::{Expr};
    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, expr: &Expr) -> T;
        fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
        fn visit_literal_expr(&mut self, expr: &Expr) -> T;
        fn visit_unary_expr(&mut self, expr: &Expr) -> T;
    }
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(TokenVal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
}


struct AstPrinter;

impl AstPrinter {
    pub fn print (&mut self, expr:Box<Expr>) -> String {
        walk_expr(self, &expr)
    }
    fn parenthesize(&mut self, name:String, expressions: Vec<&Box<Expr>>) -> String {
        let mut str = String::new();

        str.push_str("(");
        str.push_str(name.as_str());

        for expr in expressions {
            str.push_str(" ");
            str.push_str(walk_expr(self, &expr).as_str())
        }
        str.push_str(")");
        str
    }
}
impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Binary(left, token, right) => self.parenthesize(token.lexeme.clone(), vec![left, right]),
            _ => "".to_string()
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Grouping(expr) =>  self.parenthesize("group".to_string(),  vec![expr]),
            _ => "".to_string()
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Literal(TokenVal::Str(str)) => str.to_string(),
            Literal(TokenVal::Float(f64)) => f64.to_string(),
            _ => "".to_string()
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Unary(token, expr) => self.parenthesize(token.lexeme.clone(), vec![expr]),
            _ => "".to_string()
        }
    }
}

pub fn walk_expr<T>(visitor: &mut dyn Visitor<T>, e: &Box<Expr>) -> T {
     match &**e {
        Binary(_,_,_) => {
            visitor.visit_binary_expr(e)
        },
        Literal(_) => {
            visitor.visit_literal_expr(e)
        },
        Grouping(_) => {
            visitor.visit_grouping_expr(e)
        },
        Unary(_, _) => {
            visitor.visit_unary_expr(e)
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    use super::*;

    #[test]
    fn correct_scanning_simple_function_structure() {
        let token_minus = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: TokenVal::Str("".to_string()),
            line: 1,
        };
        let start = Token {
            token_type: TokenType::Str,
            lexeme: "*".to_string(),
            literal: TokenVal::Str("".to_string()),
            line: 1,
        };
        let expression = Box::from(Binary(
            Box::from(Unary(token_minus, Box::from(Box::from(Literal(TokenVal::Float(121.0)))))),
            start,
            Box::from(Grouping(Box::from(Literal(TokenVal::Float(22.27)))))
        ));

        let mut printer = AstPrinter{};
        assert_eq!(printer.print(expression), "(* (- 121) (group 22.27))");
    }
}