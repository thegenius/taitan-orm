use crate::template_parser::structs::binary_op::BinaryOp;
use crate::{Atomic, VariableChain};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
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
        name: VariableChain,
        args: Vec<Expr>,
    },
    Nested(Box<Expr>), // ()嵌套表达式
    Not(Box<Expr>),    // NOT 表达式
}

fn parse(atomics: Vec<Atomic>) -> Result<Expr, String> {
    let mut exprs = Vec::new();
    let mut operand_stack: Vec<Atomic> = Vec::new();
    let mut operator_stack: VecDeque<BinaryOp> = VecDeque::new();

    for atom in atomics {
        match atom {
            Atomic::Bool(value) => operand_stack.push(Atomic::Bool(value)),
            Atomic::Number(value) => operand_stack.push(Atomic::Number(value)),
            Atomic::Text(value) => operand_stack.push(Atomic::Text(value)),
            Atomic::VariableChain(chain) => operand_stack.push(Atomic::VariableChain(chain)),
            Atomic::Placeholder(ph) => operand_stack.push(Atomic::Placeholder(ph)),
            Atomic::Template(template) => operand_stack.push(Atomic::Template(template)),
            Atomic::BinaryOp(op) => {
                while let Some(top_op) = operator_stack.pop_back() {
                    if precedence(&top_op) >= precedence(&op) {
                        apply_operator(&mut operand_stack, top_op)?;
                    } else {
                        operator_stack.push_back(top_op);
                        break;
                    }
                }
                operator_stack.push_back(op);
            }
            _ => return Err("Unsupported atomic type".to_string()),
        }
    }

    // Apply remaining operators
    while let Some(op) = operator_stack.pop_back() {
        apply_operator(&mut operand_stack, op)?;
    }

    if operand_stack.len() == 1 {
        Ok(operand_stack.pop().unwrap())
    } else {
        Err("Invalid expression".to_string())
    }
}

fn precedence(op: &BinaryOp) -> i32 {
    match op {
        BinaryOp::Logic(_) => 1,
        BinaryOp::Compare(_) => 2,
        BinaryOp::Arithmetic(_) => 3,
        BinaryOp::Match(_) => 4,
        BinaryOp::Comma => 0,
    }
}

fn apply_operator(operand_stack: &mut Vec<Expr>, op: BinaryOp) -> Result<(), String> {
    if operand_stack.len() < 2 {
        return Err("Not enough operands".to_string());
    }
    let right = operand_stack.pop().unwrap();
    let left = operand_stack.pop().unwrap();
    let binary_expr = Expr::BinaryExpr {
        op,
        left: Box::new(left),
        right: Box::new(right),
    };
    operand_stack.push(binary_expr);
    Ok(())
}
