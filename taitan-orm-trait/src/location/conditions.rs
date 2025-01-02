use std::str::Chars;
use crate::location::location_expr::LogicalOperator;
use crate::{LocationExpr, LocationTrait};

pub trait LocationExprTrait {
    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;
}



#[derive(Debug)]
pub enum Conditions<T>
where
    T: LocationExprTrait,
{
    Operator {
        op: LogicalOperator,
        children: Vec<Conditions<T>>,
    },
    Leaf(T),
}

#[inline(always)]
pub fn wrap_where_seg<T>(name: &str, location_expr: &LocationExpr<T>, wrap_char: char, place_holder: char) -> String {
    let mut sql = String::new();
    sql.push(wrap_char);
    sql.push_str(name);
    sql.push(wrap_char);
    sql.push_str(location_expr.get_cmp_sql());
    sql.push(place_holder);
    sql
}
impl<T> Conditions<T>
where
    T: LocationExprTrait,
{
    pub fn group(op: LogicalOperator, children: Vec<Conditions<T>>) -> Self {
        Conditions::Operator { op, children }
    }

    pub fn element(location: T) -> Self {
        Conditions::Leaf(location)
    }
    pub fn to_sql(&self, wrap_char: char, place_holder: char) -> String {
        match self {
            Conditions::Operator { op, children } => {
                let mut sql_parts = Vec::new();
                for child in children {
                    sql_parts.push(format!("({})", child.to_sql(wrap_char, place_holder)));
                }

                match op {
                    LogicalOperator::And => sql_parts.join(" AND "),
                    LogicalOperator::Or => sql_parts.join(" OR "),
                    LogicalOperator::Not => {
                        if sql_parts.len() != 1 {
                            panic!("NOT operator must have exactly one child");
                        }
                        format!("NOT {}", sql_parts[0])
                    }
                }
            }
            Conditions::Leaf(location_expr) => location_expr
                .get_where_clause(wrap_char, place_holder)
                .to_string(),
        }
    }
}
//
// struct Parser<'a> {
//     chars: Chars<'a>,
//     current: Option<char>,
// }
//
// impl<'a> Parser<'a> {
//     pub fn new(sql: &'a str) -> Self {
//         let mut parser = Parser {
//             chars: sql.chars(),
//             current: None,
//         };
//         parser.advance();
//         parser
//     }
//
//     fn advance(&mut self) {
//         self.current = self.chars.next();
//     }
//
//     fn parse_expression(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         self.parse_or()
//     }
//
//     fn parse_or(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         let mut nodes = vec![self.parse_and()?];
//
//         while self.eat_whitespace().is_some() && self.eat_keyword("OR").is_some() {
//             nodes.push(self.parse_and()?);
//         }
//
//         Ok(Conditions::group(LogicalOperator::Or, nodes))
//     }
//
//     fn parse_and(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         let mut nodes = vec![self.parse_not()?];
//
//         while self.eat_whitespace().is_some() && self.eat_keyword("AND").is_some() {
//             nodes.push(self.parse_not()?);
//         }
//
//         Ok(Conditions::group(LogicalOperator::And, nodes))
//     }
//
//     fn parse_not(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         if self.eat_whitespace().is_some() && self.eat_keyword("NOT").is_some() {
//             return Ok(Conditions::group(LogicalOperator::Not, vec![self.parse_primary()?]));
//         }
//
//         self.parse_primary()
//     }
//
//     fn parse_primary(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         if self.eat_whitespace().is_some() && self.eat('(').is_some() {
//             let expr = self.parse_expression()?;
//             if self.eat_whitespace().is_some() && self.eat(')').is_some() {
//                 return Ok(expr);
//             } else {
//                 return Err("Expected ')'");
//             }
//         }
//
//         self.parse_leaf()
//     }
//
//     fn parse_leaf(&mut self) -> Result<Conditions<&'a str>, &'static str> {
//         // 这里简化处理，假设条件表达式是 "field = value"
//         let start = self.current_position();
//         while self.current.is_some() && !self.current.unwrap().is_whitespace() {
//             self.advance();
//         }
//         let end = self.current_position();
//         let slice = &self.slice(start, end);
//
//         if slice.contains('=') {
//             Ok(Conditions::element(LocationExpr(slice)))
//         } else {
//             Err("Invalid leaf expression")
//         }
//     }
//
//     fn eat_whitespace(&mut self) -> Option<char> {
//         while let Some(c) = self.current {
//             if c.is_whitespace() {
//                 self.advance();
//             } else {
//                 break;
//             }
//         }
//         self.current
//     }
//
//     fn eat_keyword(&mut self, keyword: &str) -> Option<()> {
//         let start = self.current_position();
//         for c in keyword.chars() {
//             if self.current == Some(c) {
//                 self.advance();
//             } else {
//                 self.rewind_to(start);
//                 return None;
//             }
//         }
//         Some(())
//     }
//
//     fn eat(&mut self, expected: char) -> Option<()> {
//         if self.current == Some(expected) {
//             self.advance();
//             Some(())
//         } else {
//             None
//         }
//     }
//
//     fn current_position(&self) -> usize {
//         self.chars.as_str().len() - self.chars.as_str().chars().count()
//     }
//
//     fn slice(&self, start: usize, end: usize) -> &'a str {
//         &self.chars.as_str()[start..end]
//     }
//
//     fn rewind_to(&mut self, position: usize) {
//         let remaining = self.chars.as_str()[position..].chars();
//         self.chars = remaining;
//         self.advance();
//     }
// }




#[cfg(test)]
mod test {

    use crate::location::location_expr::{ LogicalOperator};
    use crate::{CmpOperator, LocationExpr, LocationTrait, Optional};
    use sqlx::types::Uuid;
    use time::macros::format_description;
    use time::PrimitiveDateTime;
    use crate::location::conditions::{wrap_where_seg, Conditions, LocationExprTrait};
    use crate::location::conditions::test::UserLocationExpr::*;

    #[derive(Debug)]
    pub enum UserLocationExpr {
        RequestId(LocationExpr<Uuid>),
        Name(LocationExpr<String>),
        Age(LocationExpr<i32>),
        Birthday(LocationExpr<PrimitiveDateTime>),
    }


    impl LocationExprTrait for UserLocationExpr {
        fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
            match self {
                RequestId(request_id) => {
                    wrap_where_seg("request_id", request_id, wrap_char, place_holder)
                },
                Name(name) => {
                    wrap_where_seg("name", name, wrap_char, place_holder)
                }
                Age(age) => {
                    wrap_where_seg("age", age, wrap_char, place_holder)
                }
                Birthday(birthday) => {
                    wrap_where_seg("birthday", birthday, wrap_char, place_holder)
                }
            }
        }
    }

    #[test]
    fn location_expr_spec() {
        let name = Conditions::element(Name(LocationExpr::new(CmpOperator::Eq, "Alice".to_string())));
        let age = Conditions::element(Age(LocationExpr::new(CmpOperator::Eq, 30)));
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let birthday = Conditions::element(Birthday(LocationExpr::new(
            CmpOperator::Eq,
            PrimitiveDateTime::parse("2020-01-02 03:04:05", format).unwrap(),
        )));

        // 创建 AND 节点
        let and_node = Conditions::group(LogicalOperator::And, vec![name, age]);

        // 创建 OR 节点
        let or_node = Conditions::group(LogicalOperator::Or, vec![and_node, birthday]);

        // 生成 SQL 表达式
        let sql_expression = or_node.to_sql('`', '?');

        // 打印 SQL 表达式
        assert_eq!("", sql_expression);
    }
}
