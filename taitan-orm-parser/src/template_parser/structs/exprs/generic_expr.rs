use crate::template::{BoolValue, MatchOp, TextValue};
use crate::template_parser::structs::operators::{
    ArithmeticUnaryOp, CompareOp, ListInOp, LogicOp, Paren,
};
use crate::template_parser::{ArithmeticExpr, ArithmeticOp, LogicExpr, MaybeValue, TextExpr};
use crate::{Atomic, AtomicStream, Operator, Sign, VariableChain};
use proc_macro2::fallback::unforce;

use crate::template_parser::error::TemplateParseError;
use crate::template_parser::structs::operators::ConnectOp;
use tracing::{debug, error};
use crate::template_parser::structs::atomics::{GenericAtomic, GenericAtomicStream, MySqlAtomic};

pub type ParseResult<T> = std::result::Result<T, TemplateParseError>;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericExpr {
    Atomic(GenericAtomic),
    ArithmeticExpr {
        left: Box<GenericExpr>,
        op: ArithmeticOp,
        right: Box<GenericExpr>,
    },
    AnnotatedArithmeticExpr {
        unary_op: ArithmeticUnaryOp,
        expr: Box<GenericExpr>,
    },
    CompareExpr {
        left: Box<GenericExpr>,
        op: CompareOp,
        right: Box<GenericExpr>,
    },
    Not(Box<GenericExpr>),
    LogicExpr {
        left: Box<GenericExpr>,
        op: LogicOp,
        right: Box<GenericExpr>,
    },
    CommaExpr {
        left: Box<GenericExpr>,
        op: ConnectOp,
        right: Box<GenericExpr>,
    },
    NestedExpr(Box<GenericExpr>),
    FnCallExpr {
        name: VariableChain,
        args: Box<GenericExpr>, // must be CommaExpr
    },
    ListInExpr {
        left: Box<GenericExpr>,
        op: ListInOp,
        right: Box<GenericExpr>,
    },
}

impl GenericExpr {
    pub fn parse_str(input: &str) -> ParseResult<GenericExpr> {
        let stream = GenericAtomicStream::parse::<MySqlAtomic>(input)?;
        let (remaining, parsed) = Self::parse(stream.atomics)?;
        Ok(parsed)
    }

    fn is_inner_comma_expr(&self) -> bool {
        match self {
            GenericExpr::CommaExpr { .. } => true,
            GenericExpr::Atomic(_) => true,
            GenericExpr::NestedExpr(inner) => inner.is_inner_comma_expr(),
            _ => false,
        }
    }
    fn is_nested_comma_expr(&self) -> bool {
        match self {
            GenericExpr::NestedExpr(inner) => inner.is_inner_comma_expr(),
            _ => false,
        }
    }

    fn is_unary_op(arithmetic_op: ArithmeticOp, prev: &Option<GenericAtomic>) -> bool {
        // 如果 + 或 - 出现在以下位置，则是一元操作符：
        // 1. 表达式的开头
        // 2. 在另一个操作符之后
        // 3. 在左括号之后
        match arithmetic_op {
            ArithmeticOp::Add | ArithmeticOp::Sub => {}
            _ => return false,
        }

        match prev {
            None => true,
            Some(atomic) => match atomic {
                GenericAtomic::Operator(_) => true,
                _ => false,
            },
        }
    }

    pub fn parse(mut atomics: Vec<GenericAtomic>) -> ParseResult<(Vec<GenericAtomic>, GenericExpr)> {
        debug!("GenericExpr::parse({:?})", atomics);
        let mut operands: Vec<GenericExpr> = Vec::new(); // 操作数栈
        let mut operators: Vec<Operator> = Vec::new(); // 操作符栈
        let mut prev: Option<GenericAtomic> = None;

        for token in atomics.clone() {
            let current = token.clone();
            match token {
                GenericAtomic::Keyword(_) => {
                    return Ok((atomics, GenericExpr::Atomic(token.clone())));
                }
                GenericAtomic::Null=> {
                    operands.push(GenericExpr::Atomic(token.clone()));
                }
                GenericAtomic::Number(_) | GenericAtomic::Text(_) | GenericAtomic::Bool(_) | GenericAtomic::Maybe(_) => {
                    // 操作数直接压入操作数栈
                    debug!("GenericExpr::parse() push atomic: {:?}", &token);
                    operands.push(GenericExpr::Atomic(token.clone()));
                }
                GenericAtomic::Sign(sign) => {
                    if let Sign::Star = sign {}
                    operands.push(GenericExpr::Atomic(GenericAtomic::Sign(sign.clone())));
                }
                GenericAtomic::Operator(operator) => {
                    match operator {
                        Operator::Paren(Paren::Left) => {
                            // 检查是否是函数调用
                            if let Some(GenericExpr::Atomic(GenericAtomic::Maybe(
                                MaybeValue::VariableChain(v),
                            ))) = operands.last()
                            {
                                // 如果是 VariableChain，压入 FnCallOp
                                operators.push(Operator::FnCall(v.clone()));
                            } else {
                                // 否则，压入普通左括号
                                operators.push(Operator::Paren(Paren::Left));
                            }
                        }
                        Operator::Paren(Paren::Right) => {
                            // 右括号：弹出操作符并构建表达式，直到遇到左括号或 FnCallOp
                            while let Some(top) = operators.last() {
                                match top {
                                    Operator::Paren(Paren::Left) | Operator::FnCall(_) => break,
                                    _ => Self::reduce(&mut operands, &mut operators)?,
                                }
                            }
                            // 弹出左括号或 FnCallOp
                            debug!("GenericExpr::parse() pop left paren or FnCallOp");
                            if let Some(op) = operators.pop() {
                                match op {
                                    Operator::Paren(Paren::Left) | Operator::FnCall(_) => {
                                        // 处理函数调用
                                        Self::reduce(&mut operands, &mut operators)?
                                    }
                                    _ => return Err("Mismatched parentheses".into()),
                                }
                            } else {
                                return Err("Mismatched parentheses".into());
                            }
                        }
                        Operator::Arithmetic(arithmetic_op) => {
                            // 检查是否是二元操作符或一元操作符
                            let is_unary = Self::is_unary_op(arithmetic_op.clone(), &prev);

                            if is_unary {
                                // 压入一元操作符
                                match arithmetic_op {
                                    ArithmeticOp::Add => operators
                                        .push(Operator::ArithmeticUnary(ArithmeticUnaryOp::Add)),
                                    ArithmeticOp::Sub => operators
                                        .push(Operator::ArithmeticUnary(ArithmeticUnaryOp::Sub)),
                                    _ => unreachable!(),
                                }
                            } else {
                                // 处理二元操作符
                                while let Some(top) = operators.last() {
                                    if let Operator::Paren(Paren::Left) | Operator::FnCall(_) = top
                                    {
                                        break; // 遇到左括号或 FnCallOp，停止弹出
                                    }
                                    if precedence(top) >= precedence(&operator) {
                                        // 栈顶优先级更高，弹出并构建表达式
                                        Self::reduce(&mut operands, &mut operators)?;
                                    } else {
                                        break;
                                    }
                                }
                                // 当前操作符入栈
                                operators.push(operator.clone());
                            }
                            debug!(
                                "GenericExpr::parse() operators len[{}]: {:?}",
                                operators.len(),
                                operators
                            );
                        }
                        _ => {
                            // 处理其他操作符
                            while let Some(top) = operators.last() {
                                if let Operator::Paren(Paren::Left) | Operator::FnCall(_) = top {
                                    break; // 遇到左括号或 FnCallOp，停止弹出，直接入栈
                                }
                                if precedence(top) >= precedence(&operator) {
                                    // 栈顶优先级更高，弹出并构建表达式
                                    Self::reduce(&mut operands, &mut operators)?;
                                } else {
                                    break;
                                }
                            }
                            // 当前操作符入栈
                            operators.push(operator.clone());
                            debug!(
                                "GenericExpr::parse() operators len[{}]: {:?}",
                                operators.len(),
                                operators
                            );
                        }
                    }
                }
            }
            prev = Some(current);
        }

        // 处理剩余的操作符
        while let Some(op) = operators.last() {
            if let Operator::Paren(Paren::Left) | Operator::FnCall(_) = op {
                return Err("Mismatched parentheses".into());
            }
            debug!("GenericExpr::parse remaining operator: {:?}", &operators);
            Self::reduce(&mut operands, &mut operators)?;
        }

        // 最终操作数栈中应只有一个表达式
        if operands.len() != 1 {
            // error!("GenericExpr::parse() operands len: {:?}", &operands.len());
            // error!("GenericExpr::parse() operands {:?}", &operands);
            // error!("GenericExpr::parse() operators {:?}", &operators);
            return Err("Invalid expression".into());
        }

        let expr = operands.pop().unwrap();
        Ok((Vec::new(), expr))
    }

    fn reduce(
        operands: &mut Vec<GenericExpr>,
        operators: &mut Vec<Operator>,
    ) -> Result<(), String> {
        if let Some(op) = operators.pop() {
            debug!(
                "GenericExpr::reduce op: ({:?}) remaining len[{:?}]",
                op,
                operators.len()
            );
            match op {
                Operator::ArithmeticUnary(unary_op) => match unary_op {
                    ArithmeticUnaryOp::Add => {
                        let operand = operands
                            .pop()
                            .ok_or("Missing operand for UnaryPlus".to_string())?;
                        operands.push(GenericExpr::AnnotatedArithmeticExpr {
                            unary_op: ArithmeticUnaryOp::Add,
                            expr: Box::new(operand),
                        });
                    }
                    ArithmeticUnaryOp::Sub => {
                        let operand = operands
                            .pop()
                            .ok_or("Missing operand for UnaryMinus".to_string())?;
                        operands.push(GenericExpr::AnnotatedArithmeticExpr {
                            unary_op: ArithmeticUnaryOp::Sub,
                            expr: Box::new(operand),
                        });
                    }
                },
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
                Operator::Logic(logic_op) => match logic_op {
                    LogicOp::Not => {
                        let operand = operands
                            .pop()
                            .ok_or("Missing operand for Not".to_string())?;
                        operands.push(GenericExpr::Not(Box::new(operand)));
                    }
                    _ => {
                        let right = operands.pop().ok_or("Missing right operand".to_string())?;
                        let left = operands.pop().ok_or("Missing left operand".to_string())?;
                        operands.push(GenericExpr::LogicExpr {
                            left: Box::new(left),
                            op: logic_op,
                            right: Box::new(right),
                        });
                    }
                },
                Operator::ListInOp(list_in_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    if !right.is_inner_comma_expr() {
                        return Err("right operand must be comma expr".to_string());
                    }
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::ListInExpr {
                        left: Box::new(left),
                        op: list_in_op,
                        right: Box::new(right),
                    });
                }
                Operator::Connect(connect_op) => {
                    let right = operands.pop().ok_or("Missing right operand".to_string())?;
                    let left = operands.pop().ok_or("Missing left operand".to_string())?;
                    operands.push(GenericExpr::CommaExpr {
                        left: Box::new(left),
                        op: connect_op,
                        right: Box::new(right),
                    });
                }
                Operator::FnCall(variable_chain) => {
                    // 弹出参数列表
                    let args = operands.pop().ok_or("Missing fn call args".to_string())?;
                    if !args.is_inner_comma_expr() {
                        return Err("Function arguments must be a comma expression".to_string());
                    }

                    // 构建 FnCallExpr
                    let fn_call = GenericExpr::FnCallExpr {
                        name: variable_chain,
                        args: Box::new(args),
                    };
                    operands.push(fn_call);
                }
                Operator::Paren(p) => match p {
                    Paren::Left => {
                        let inner = operands.pop().ok_or("Missing right operand".to_string())?;
                        operands.push(GenericExpr::NestedExpr(Box::new(inner)));
                    }
                    Paren::Right => return Err("Unexpected parenthesis in reduce".to_string()),
                },
            }
        }
        Ok(())
    }
}

// 归约操作：从操作数栈和操作符栈中构建表达式

// 操作符优先级
fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::ArithmeticUnary(_) => 8, // 一元操作符的优先级高于二元操作符
        Operator::Arithmetic(arithmetic_op) => match arithmetic_op {
            ArithmeticOp::Mul | ArithmeticOp::Div | ArithmeticOp::Mod => 7,
            ArithmeticOp::Add | ArithmeticOp::Sub => 6,
        },
        Operator::Compare(_) => 5,
        Operator::ListInOp(_) => 5,
        Operator::Logic(logic_op) => match logic_op {
            LogicOp::Not => 4,
            LogicOp::And => 3,
            LogicOp::Or => 2,
        },
        Operator::Connect(_) => 1,
        Operator::Paren(_) | Operator::FnCall(_) => 0, // 括号优先级最低
    }
}
