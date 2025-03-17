use crate::{Atomic, VariableChain};
use crate::template::{ArithmeticOp, CompareOp, GenericExpr, ListInOp, LogicOp, TextValue};

// #[derive(Debug, Clone, PartialEq)]
// pub enum Clause {
//     AtomicExpr(GenericExpr),
//     ExprList(Vec<GenericExpr>), // ExprList是多个GenericExpr以逗号,分割的序列
//     FnCall {
//         name: VariableChain,    // name是任意合法的VariableChain
//         args: Vec<GenericExpr>  // args是多个GenericExpr以逗号,分割的序列
//     }
// }
//
// impl Clause {
//     pub fn parse(atomics: Vec<Atomic>) -> Result<Clause, String> {
//
//     }
// }