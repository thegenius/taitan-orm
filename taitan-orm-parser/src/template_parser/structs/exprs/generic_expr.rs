use crate::template::{BoolValue, MatchOp, TextValue};
use crate::template_parser::structs::operators::{CompareOp, ListInOp, LogicOp, Paren};
use crate::template_parser::{ArithmeticExpr, ArithmeticOp, LogicExpr, TextExpr};
use crate::{Atomic, AtomicStream, Operator, Sign};
use proc_macro2::fallback::unforce;
use tracing::{debug, error};


#[derive(Debug, Clone, PartialEq)]
pub enum GenericExpr {
    Atomic(Atomic),
    TextExpr(TextValue),
    ArithmeticExpr {
        left: Box<GenericExpr>,
        op: ArithmeticOp,
        right: Box<GenericExpr>,
    },
    CompareExpr {
        left: Box<GenericExpr>,
        op: CompareOp,
        right: Box<GenericExpr>,
    },
    LogicExpr {
        left: Box<GenericExpr>,
        op: LogicOp,
        right: Box<GenericExpr>,
    },
    NestedExpr(Box<GenericExpr>),
    ListInExpr {
        left: Box<GenericExpr>,
        op: ListInOp,
        right: Box<GenericExpr>,
    },
}

impl GenericExpr {

    pub fn parse_str(input: &str) -> Result<GenericExpr, String> {
        let stream = AtomicStream::parse(input)?;
        Self::parse(stream.atomics)
    }
    pub fn parse(atomics: Vec<Atomic>) -> Result<GenericExpr, String> {
        debug!("GenericExpr::parse({:?})", atomics);
        let mut operands: Vec<GenericExpr> = Vec::new(); // 操作数栈
        let mut operators: Vec<Operator> = Vec::new(); // 操作符栈

        for token in atomics {
            match token {
                Atomic::Number(_) | Atomic::Text(_) | Atomic::Bool(_) | Atomic::Maybe(_) | Atomic::Sign(_) => {
                    // 操作数直接压入操作数栈
                    debug!("GenericExpr::parse() push atomic: {:?}", &token);
                    operands.push(GenericExpr::Atomic(token));
                }
                Atomic::Operator(operator) => {
                    match operator {
                        Operator::Paren(Paren::Left) => {
                            // 左括号直接入栈
                            operators.push(operator);
                        }
                        Operator::Paren(Paren::Right) => {
                            // 右括号：弹出操作符并构建表达式，直到遇到左括号
                            while let Some(top) = operators.last() {
                                if let Operator::Paren(Paren::Left) = top {
                                    break;
                                }
                                Self::reduce(&mut operands, &mut operators)?;
                            }
                            // 弹出左括号
                            debug!("GenericExpr::parse() pop left paren");
                            if operators.pop() != Some(Operator::Paren(Paren::Left)) {
                                return Err("Mismatched parentheses".to_string());
                            }
                        }
                        _ => {
                            // 处理其他操作符
                            while let Some(top) = operators.last() {
                                if let Operator::Paren(Paren::Left) = top {
                                    break; // 遇到左括号，停止弹出
                                }
                                if precedence(top) >= precedence(&operator) {
                                    // debug!("GenericExpr::parse() reduce current: {:?}, top: {:?}", &operator, &top);
                                    // 栈顶优先级更高，弹出并构建表达式
                                    Self::reduce(&mut operands, &mut operators)?;
                                } else {
                                    break;
                                }
                            }
                            // 当前操作符入栈
                            // debug!("GenericExpr::parse() push operator: {:?}", &operator);
                            operators.push(operator);
                            debug!("GenericExpr::parse() operators len[{}]: {:?}", operators.len(), operators);
                        }
                    }
                }
            }
        }

        // 处理剩余的操作符
        while let Some(op) = operators.last() {
            if let Operator::Paren(Paren::Left) = op {
                return Err("Mismatched parentheses".to_string());
            }
            debug!("GenericExpr::parse remaining operator: {:?}", &operators);
            Self::reduce(&mut operands, &mut operators)?;
        }

        // 最终操作数栈中应只有一个表达式
        if operands.len() != 1 {
            error!("GenericExpr::parse() operands len: {:?}", &operands.len());
            error!("GenericExpr::parse() operands {:?}", &operands);
            error!("GenericExpr::parse() operators {:?}", &operators);
            return Err("Invalid expression".to_string());
        }

        operands.pop().ok_or("Empty operands".to_string())
    }
    fn reduce(
        operands: &mut Vec<GenericExpr>,
        operators: &mut Vec<Operator>,
    ) -> Result<(), String> {
        // debug!("GenericExpr::reduce({:?}, {:?})", operands, &operators);
        if let Some(op) = operators.pop() {
            debug!("GenericExpr::reduce op: ({:?}) remaining len[{:?}]", op, operators.len());
            match op {
                Operator::Arithmetic(arithmetic_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::ArithmeticExpr {
                        left: Box::new(left),
                        op: arithmetic_op,
                        right: Box::new(right),
                    });
                }
                Operator::Compare(compare_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::CompareExpr {
                        left: Box::new(left),
                        op: compare_op,
                        right: Box::new(right),
                    });
                }
                Operator::Logic(logic_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::LogicExpr {
                        left: Box::new(left),
                        op: logic_op,
                        right: Box::new(right),
                    });
                }
                Operator::ListInOp(list_in_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::ListInExpr {
                        left: Box::new(left),
                        op: list_in_op,
                        right: Box::new(right),
                    });
                }
                Operator::Paren(_) => return Err("Unexpected parenthesis in reduce".to_string()),
            }
        }
        Ok(())
    }
}

// 归约操作：从操作数栈和操作符栈中构建表达式

// 操作符优先级
fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Arithmetic(arithmetic_op) => match arithmetic_op {
            ArithmeticOp::Mul | ArithmeticOp::Div | ArithmeticOp::Mod => 6,
            ArithmeticOp::Add | ArithmeticOp::Sub => 5,
        },
        Operator::Compare(_) => 4,
        Operator::ListInOp(_) => 4,
        Operator::Logic(logic_op) => match logic_op {
            LogicOp::Not => 3,
            LogicOp::And => 2,
            LogicOp::Or => 1,
        },
        Operator::Paren(_) => 0, // 括号优先级最低
    }
}
