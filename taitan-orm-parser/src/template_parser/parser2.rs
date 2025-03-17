// use crate::template_parser::structs::expr::Expr;
// use crate::template_parser::structs::operators::LogicOp;
// use crate::template_parser::ArithmeticOp;
// use crate::{Atomic, Operator, Sign, SimpleExpr};
// use crate::Operator::Arithmetic;
//
//
// #[derive(Debug, PartialEq, Clone)]
// pub enum AtomicExpr {
//     Simple(SimpleExpr),
//     BinaryExpr {
//         left: Box<AtomicExpr>,
//         op: Operator,
//         right: Box<AtomicExpr>,
//     },
//     Nested(Box<AtomicExpr>),  // ()嵌套表达式
//     Not(Box<AtomicExpr>),     // NOT 表达式
// }
// pub fn parse_atomics_to_expr(atomics: Vec<Atomic>) -> AtomicExpr {
//     let mut operands: Vec<AtomicExpr> = Vec::new(); // 操作数栈
//     let mut operators: Vec<Operator> = Vec::new(); // 运算符栈
//     let mut bracket_stack: Vec<(Vec<AtomicExpr>, Vec<Operator>)> = Vec::new(); // 括号栈
//
//     // Value, Placeholder, 直接入栈，等待规约
//     // Operator
//     //    如果比栈顶的操作符优先级高，则入栈
//     //    如果比栈顶的操作符优先级低，则栈顶和操作数应该尝试组合为AtomicExpr
//     // ConnectOp,
//     // Sign,
//     //    遇到左括号
//     //    遇到右括号
//     for atomic in atomics {
//         match atomic {
//             Atomic::Value(value) => {
//                 operands.push(AtomicExpr::Simple(SimpleExpr::Single(Atomic::Value(value))));
//             }
//             Atomic::Placeholder(placeholder) => {
//                 operands.push(AtomicExpr::Simple(SimpleExpr::Single(Atomic::Placeholder(placeholder))));
//             }
//             Atomic::Operator(op) => {
//                 // 处理一元操作符 Not
//                 if let Operator::Logic(LogicOp::Not) = op {
//                     // Not 是一元操作符，直接压入运算符栈
//                     operators.push(op);
//                     continue;
//                 }
//
//                 // 处理二元操作符
//                 while let Some(top_op) = operators.last() {
//                     if precedence(&op) <= precedence(top_op) {
//                         let op = operators.pop().unwrap();
//                         apply_operator(op, &mut operands); // 使用公共函数
//                     } else {
//                         break;
//                     }
//                 }
//                 operators.push(op);
//             }
//             Atomic::ConnectOp(_) => {
//                 // 处理连接操作符（如点号）
//                 // 这里可以根据具体需求实现
//             }
//             Atomic::Sign(sign) => {
//                 match sign {
//                     Sign::Bracket('(') => {
//                         // 遇到左括号，保存当前状态
//                         bracket_stack.push((operands, operators));
//                         operands = Vec::new();
//                         operators = Vec::new();
//                     }
//                     Sign::Bracket(')') => {
//                         // 遇到右括号，处理括号内的表达式
//                         let mut nested_expr = operands.pop().unwrap();
//                         while let Some(op) = operators.pop() {
//                             if let Operator::Logic(LogicOp::Not) = op {
//                                 // Not 是一元操作符
//                                 nested_expr = AtomicExpr::Not(Box::new(nested_expr));
//                             } else {
//                                 // 处理二元操作符
//                                 let right = operands.pop().unwrap();
//                                 let left = operands.pop().unwrap();
//                                 nested_expr = AtomicExpr::BinaryExpr {
//                                     left: Box::new(left),
//                                     op,
//                                     right: Box::new(right),
//                                 };
//                             }
//                         }
//                         // 恢复括号外的状态
//                         let (prev_operands, prev_operators) = bracket_stack.pop().unwrap();
//                         operands = prev_operands;
//                         operators = prev_operators;
//                         operands.push(AtomicExpr::Nested(Box::new(nested_expr)));
//                     }
//                     Sign::Star | Sign::Semicolon | Sign::Bracket(_) | Sign::Unknown(_) => {
//                         // 处理其他符号
//                         // 这里可以根据具体需求实现
//                     }
//                 }
//             }
//         }
//     }
//
//     // 处理剩余的运算符
//     while let Some(op) = operators.pop() {
//         apply_operator(op, &mut operands); // 使用公共函数
//     }
//
//     // 返回最终的表达式
//     operands.pop().unwrap()
// }
//
//
// fn apply_operator(op: Operator, operands: &mut Vec<AtomicExpr>) {
//     match op {
//         Operator::Logic(LogicOp::Not) => {
//             // Not 是一元操作符，只需要一个操作数
//             let operand = operands.pop().unwrap();
//             operands.push(AtomicExpr::Not(Box::new(operand)));
//         }
//         _ => {
//             // 处理二元操作符
//             let right = operands.pop().unwrap();
//             let left = operands.pop().unwrap();
//             operands.push(AtomicExpr::BinaryExpr {
//                 left: Box::new(left),
//                 op,
//                 right: Box::new(right),
//             });
//         }
//     }
// }
//
// // 算术优先级最高
// // 比较操作符其次
// // match等价于比价
// // logic操作符优先级最低
// fn precedence(op: &Operator) -> u8 {
//     match op {
//         Arithmetic(arithmetic) => match arithmetic {
//             ArithmeticOp::Add | ArithmeticOp::Sub => 4,
//             ArithmeticOp::Mul | ArithmeticOp::Div | ArithmeticOp::Mod => 5,
//         },
//         Operator::Compare(compare) => 3,
//         Operator::Match(matcher) => 2,
//         Operator::Logic(logic) => match logic {
//             LogicOp::Not => 2,
//             LogicOp::And => 1,
//             LogicOp::Or => 0,
//         },
//     }
// }
