use crate::template_parser::structs::binary_op::BinaryOp;
use crate::{Atomic, Sign, VariableChain};
use tracing::debug;

// <expr> ::= <simple-expr>
// | <binary-expr>
// | <function-call>
// | <nested-expr>
// | <not-expr>
//
// <simple-expr> ::= <atomic> <binary-op> <atomic>
//
// <binary-expr> ::= <expr> <binary-op> <expr>
//
// <function-call> ::= <variable-chain> "(" [ <expr> { "," <expr> } ] ")"
//
// <nested-expr> ::= "(" <expr> ")"
//
// <not-expr> ::= "NOT" <expr>
#[derive(Debug, PartialEq, Clone)]
pub enum


Expr {
    Simple {
        left: Atomic,
        op: BinaryOp,
        right: Atomic,
    },
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        // VariableChain(Expr)
        name: VariableChain,
        args: Option<Box<Expr>>,
    },
    Nested(Box<Expr>), // ()嵌套表达式
    Not(Box<Expr>),    // NOT 表达式
}

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Atomic>,
    position: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Atomic>) -> Result<Expr, String> {
        debug!("Parser::parse({:?})", tokens);
        let mut parser = Parser {
            tokens,
            position: 0,
        };
        // parser.parse_expr_comma().ok_or("Parser: failed to parse expr".to_string())
        parser.parse_expr()
    }

    pub fn current(&self) -> Option<&Atomic> {
        self.tokens.get(self.position)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        debug!("Parser::parse_expr() in pos {} with {:?}", self.position, self.current());
        // 处理 NOT Expr
        if self.tokens.get(self.position) == Some(&Atomic::Not) {
            self.position += 1;
            let expr = self.parse_expr()?;
            return Ok(Expr::Not(Box::new(expr)));
        }

        // 处理(Expr)
        if self.tokens.get(self.position) == Some(&Atomic::Sign(Sign::Bracket('('))) {
            self.position += 1;
            let expr = self.parse_expr()?;
            return if self.tokens.get(self.position) == Some(&Atomic::Sign(Sign::Bracket(')'))) {
                self.position += 1;
                Ok(Expr::Nested(Box::new(expr)))
            } else {
                Err("Expected closing parenthesis".to_string())
            };
        }

        // 处理 <simple-expr> ::= <atomic> <binary-op> <atomic>
        let simple_expr_opt = self.parse_expr_simple();
        if let Some(simple_expr) = simple_expr_opt {
            let next = self.tokens.get(self.position);
            if let Some(next) = next {
                if let Some(binary_op) = next.extract_binary_op() {
                    self.position += 1;
                    let right = self.parse_expr()?;
                    return Ok(Expr::BinaryExpr {
                        left: Box::new(simple_expr),
                        op: binary_op,
                        right: Box::new(right),
                    });
                }
            } else {
                return Ok(simple_expr);
            }
        }

        // 处理 <function-call> ::= <variable-chain> "(" [ <expr> { "," <expr> } ] ")"
        let function_call_expr_opt = self.parse_function_call();
        if let Some(function_call_expr) = function_call_expr_opt {
            return Ok(function_call_expr);
        }

        // 处理 <binary-expr> ::= <expr> <binary-op> <expr>
        let binary_expr_opt = self.parse_expr_comma();
        if let Some(binary_expr) = binary_expr_opt {
            return Ok(binary_expr);
        }

        Err("Invalid expression".to_string())
    }

    // fn parse_atomic(&mut self) -> Result<Atomic, String> {
    //     if let Some(token) = self.tokens.get(self.position) {
    //         self.position += 1;
    //         Ok(token.clone())
    //     } else {
    //         Err("Unexpected end of input".to_string())
    //     }
    // }

    // fn is_operand(&self, atomic: &Atomic) -> bool {
    //     match atomic {
    //         Atomic::BinaryOp(_) => false,
    //         Atomic::Sign(_) => false,
    //         Atomic::Not => false,
    //         _ => true,
    //     }
    // }
    //
    // fn is_binary_operator(&self, atomic: &Atomic) -> bool {
    //     match atomic {
    //         Atomic::BinaryOp(_) => true,
    //         _ => false,
    //     }
    // }

    fn parse_function_call(&mut self) -> Option<Expr> {
        debug!("Parser::parse_function_call() in pos {} with {:?}", self.position, self.current());
        let variable_chain = self.tokens.get(self.position)?;
        let variable_chain = if let Atomic::VariableChain(v) = variable_chain {
            v.clone()
        } else {
            return None;
        };

        let left_sign = self.tokens.get(self.position + 1)?;
        let _left_bracket = left_sign.extract_left_bracket()?;

        self.position += 2;
        let args = self.parse_expr_comma();
        let boxed_args = if let Some(args) = args {
            Some(Box::new(args))
        } else {
            None
        };

        let right_sign = self.tokens.get(self.position)?;
        let right_bracket = right_sign.extract_right_bracket();
        if right_bracket.is_some() {
            self.position += 1;
        }

        let expr = Expr::FunctionCall {
            name: variable_chain,
            args: boxed_args,
        };
        Some(expr)
    }

    fn parse_expr_comma(&mut self) -> Option<Expr> {
        debug!("Parser::parse_expr_comma() in pos {} with {:?}", self.position, self.current());
        let left_expr = self.parse_expr_or()?;
        let op = self.tokens.get(self.position + 1)?;
        let binary_op = if let Atomic::BinaryOp(op) = op {
            op.extract_comma()?
        } else {
            return None;
        };

        self.position += 1;
        let right_expr = self.parse_expr_or()?;

        let expr = Expr::BinaryExpr {
            left: Box::new(left_expr),
            op: binary_op,
            right: Box::new(right_expr),
        };
        Some(expr)
    }

    fn parse_expr_or(&mut self) -> Option<Expr> {
        debug!("Parser::parse_expr_or() in pos {} with {:?}", self.position, self.current());
        let left_expr = self.parse_expr_and()?;
        let op = self.tokens.get(self.position + 1)?;
        let binary_op = if let Atomic::BinaryOp(op) = op {
            op.extract_or()?
        } else {
            return None;
        };

        self.position += 1;
        let right_expr = self.parse_expr_and()?;

        let expr = Expr::BinaryExpr {
            left: Box::new(left_expr),
            op: binary_op,
            right: Box::new(right_expr),
        };
        Some(expr)
    }
    fn parse_expr_and(&mut self) -> Option<Expr> {
        debug!("Parser::parse_expr_and() in pos {} with {:?}", self.position, self.current());
        let left_expr = self.parse_expr().ok()?;
        let op = self.tokens.get(self.position + 1)?;
        let binary_op = if let Atomic::BinaryOp(op) = op {
            op.extract_and()?
        } else {
            return None;
        };

        self.position += 1;
        let right_expr = self.parse_expr().ok()?;

        let expr = Expr::BinaryExpr {
            left: Box::new(left_expr),
            op: binary_op,
            right: Box::new(right_expr),
        };
        Some(expr)
    }
    fn parse_expr_simple(&mut self) -> Option<Expr> {
        debug!("Parser::parse_expr_simple() in pos {} with {:?}", self.position, self.current());
        let first = self.tokens.get(self.position)?;
        let second = self.tokens.get(self.position + 1)?;
        let third = self.tokens.get(self.position + 2)?;
        if !first.is_operand() || !third.is_operand() {
            return None;
        }
        let op = second.extract_binary_op()?;
        let expr = Expr::Simple {
            left: first.clone(),
            op,
            right: third.clone(),
        };
        self.position += 3;
        Some(expr)
    }
}
