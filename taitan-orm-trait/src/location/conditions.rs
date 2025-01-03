use crate::error::NotValidConditionError;
use crate::location::location_expr::LogicalOperator;
use crate::LocationExpr;
use rinja::filters::e;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Arguments, Database, Type};
use std::fmt::Debug;
use std::str::Chars;

pub trait Condition {
    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;
    fn add_to_sqlite_arguments<'a, 'b>(
        &'a self,
        args: &'b mut SqliteArguments<'b>,
    ) -> Result<(), sqlx::error::BoxDynError>
    where
        'a: 'b;
    fn add_to_mysql_arguments(
        &self,
        args: &mut MySqlArguments,
    ) -> Result<(), sqlx::error::BoxDynError>;
    fn add_to_postgres_arguments(
        &self,
        args: &mut PgArguments,
    ) -> Result<(), sqlx::error::BoxDynError>;
}

#[derive(Debug, Clone)]
pub enum Conditions {
    Group {
        op: LogicalOperator,
        children: Vec<Conditions>,
    },
    Element(String),
}

impl Conditions {
    pub fn is_operator(&self, operator: &LogicalOperator) -> bool {
        match self {
            Conditions::Group { op, .. } => op == operator,
            _ => false,
        }
    }

    pub fn group(op: LogicalOperator, children: Vec<Conditions>) -> Conditions {
        Conditions::Group { op, children }
    }
}

pub struct SqlArguments<'a, DB>
where
    DB: Database,
{
    arguments: DB::Arguments<'a>,
    wrap_char: char,
    place_holder: char,
}

pub struct ConditionsWithArguments<'a, DB: Database + Debug> {
    arguments: SqlArguments<'a, DB>,
    conditions: Conditions,
}

// pub enum ArgumentsEnum<'a> {
//     MySql(MySqlArguments),
//     Postgres(PgArguments),
//     Sqlite(SqliteArguments<'a>),
// }

#[inline(always)]
pub fn wrap_where_seg(name: &str, cmp_expr: &str, wrap_char: char, place_holder: char) -> String {
    let mut sql = String::new();
    sql.push(wrap_char);
    sql.push_str(name);
    sql.push(wrap_char);
    sql.push_str(cmp_expr);
    sql.push(place_holder);
    sql
}

fn process_element<'a, T>(
    location_expr: &'a T,
    args: &'a mut SqliteArguments<'a>,
) -> Result<(), sqlx::error::BoxDynError>
where
    T: Condition,
{
    location_expr.add_to_sqlite_arguments(args)?;
    Ok(())
}

pub fn to_sql(
    conditions: &Conditions,
    wrap_char: char,
    place_holder: char,
) -> Result<String, NotValidConditionError> {
    let mut stack: Vec<(&Conditions, Option<&Conditions>, bool)> = vec![(conditions, None, false)]; // (node, parent children_processed)
    let mut sql_parts = Vec::new(); // 用于收集生成的SQL片段
                                    // let mut operators_stack = Vec::new(); // 用于追踪操作符以处理优先级

    while let Some((node, parent, children_processed)) = stack.pop() {
        match node {
            Conditions::Group { op, children } => {
                if !children_processed {
                    // 如果后续弹出到我自己了，那么说明我的所有子节点已经处理完成
                    stack.push((node, parent, true));

                    match op {
                        LogicalOperator::Not => {
                            if children.len() != 1 {
                                return Err(NotValidConditionError(
                                    "A not condition must have exactly one element".to_string(),
                                ));
                            }
                            stack.push((children.first().unwrap(), Some(node), false));
                        }
                        _ => {
                            if children.len() <= 1 {
                                return Err(NotValidConditionError(
                                    "A and/or condition must have more than one elements"
                                        .to_string(),
                                ));
                            }
                            // 然后将所有子节点按逆序压入栈中
                            for child in children.iter().rev() {
                                stack.push((child, Some(node), false));
                            }
                        }
                    }
                } else {
                    // 进入到这里说明我是一个group节点，且孩子节点都处理完成了
                    // 收集当前操作符的所有子表达式
                    let mut sub_expressions = Vec::new();
                    while let Some(part) = sql_parts.pop() {
                        sub_expressions.push(part);
                        if sub_expressions.len() == children.len() {
                            break;
                        }
                    }
                    sub_expressions.reverse();

                    // 根据操作符类型决定是否添加括号
                    // let should_wrap = matches!(op, LogicalOperator::Or)
                    //     || operators_stack.last().map_or(false, |parent_op: &&LogicalOperator| **parent_op == LogicalOperator::And);

                    let joined_sql: String = match op {
                        LogicalOperator::And => {
                            let parent_is_not = parent
                                .map_or(false, |parent| parent.is_operator(&LogicalOperator::Not));
                            // 父节点是不是not的时候不需要加()
                            if !parent_is_not {
                                sub_expressions.join(" AND ")
                            } else {
                                // 父节点是not的时候需要加()才能保证正确
                                // 例如 NOT (a AND b)
                                format!("({})", sub_expressions.join(" AND "))
                            }
                        }
                        LogicalOperator::Or => {
                            // Or 操作符通常需要打括号来保证正确性
                            // 已经是最外层或者父节点是or的时候不需要加()
                            let parent_is_or = parent
                                .map_or(true, |parent| parent.is_operator(&LogicalOperator::Or));
                            if parent_is_or {
                                sub_expressions.join(" OR ")
                            } else {
                                // 父节点不是or的时候不需要加()才能保证正确
                                // 例如 NOT (a OR b)
                                // 例如 (a OR b) AND (c OR d)
                                format!("({})", sub_expressions.join(" OR "))
                            }
                        }
                        LogicalOperator::Not => {
                            let parent_is_not = parent
                                .map_or(false, |parent| parent.is_operator(&LogicalOperator::Not));
                            if parent_is_not {
                                return Err(NotValidConditionError(
                                    "nest not is not allowed".to_string(),
                                ));
                            } else {
                                format!("NOT {}", sub_expressions.first().unwrap())
                            }
                        }
                    };

                    sql_parts.push(joined_sql);
                }
            }
            Conditions::Element(location_expr) => {
                // 对于叶子节点，直接生成SQL片段并追加到输出字符串中。
                sql_parts.push(location_expr.to_string());
            }
        }
    }

    Ok(sql_parts.join(""))
}



impl<'t, DB: Database> ConditionsWithArguments<'t, DB> {
    pub fn new(
        arguments: DB::Arguments<'t>,
        conditions: Conditions,
        wrap_char: char,
        place_holder: char,
    ) -> Self {
        Self {
            arguments: SqlArguments {
                arguments,
                wrap_char,
                place_holder,
            },
            conditions,
        }
    }


    pub fn element<'a, T>(
        arguments: &mut SqlArguments<'a, DB>,
        name: &str,
        location: LocationExpr<T>,
    ) -> Conditions
    where
        T: sqlx::Encode<'a, DB> + Type<DB> + 'a,
    {
        let element = Conditions::Element(wrap_where_seg(
            name,
            location.get_cmp_sql(),
            arguments.wrap_char,
            arguments.place_holder,
        ));
        arguments.arguments.add(location.val).unwrap();
        // if let Some(args) = args {
        //     match args {
        //         ArgumentsEnum::MySql(args) => args.add(&location.val).unwrap(),
        //         ArgumentsEnum::Postgres(args) => args.add(&location.val).unwrap(),
        //         ArgumentsEnum::Sqlite(args) => args.add(&location.val).unwrap(),
        //     }
        // }
        element
    }

    // recursive version
    // pub fn to_sql(&self, wrap_char: char, place_holder: char) -> String {
    //     match self {
    //         Conditions::Operator { op, children } => {
    //             let mut sql_parts = Vec::new();
    //             for child in children {
    //                 sql_parts.push(format!("({})", child.to_sql(wrap_char, place_holder)));
    //             }
    //
    //             match op {
    //                 LogicalOperator::And => sql_parts.join(" AND "),
    //                 LogicalOperator::Or => sql_parts.join(" OR "),
    //                 LogicalOperator::Not => {
    //                     if sql_parts.len() != 1 {
    //                         panic!("NOT operator must have exactly one child");
    //                     }
    //                     format!("NOT {}", sql_parts[0])
    //                 }
    //             }
    //         }
    //         Conditions::Leaf(location_expr) => location_expr
    //             .get_where_clause(wrap_char, place_holder)
    //             .to_string(),
    //     }
    // }

    // pub fn get_mysql_args(&self) -> Result<MySqlArguments, sqlx::error::BoxDynError> {
    //     let mut stack: Vec<(&Conditions<T>, bool)> = vec![(self, false)];
    //     let mut elements: Vec<&T> = Vec::new();
    //     while let Some((node, children_processed)) = stack.pop() {
    //         match node {
    //             Conditions::Group { op, children } => {
    //                 if !children_processed {
    //                     // 如果后续弹出到我自己了，那么说明我的所有子节点已经处理完成
    //                     stack.push((node, true));
    //                     for child in children.iter().rev() {
    //                         stack.push((child, false));
    //                     }
    //                 }
    //             }
    //             Conditions::Element(location_expr) => {
    //                 elements.push(location_expr);
    //             }
    //         }
    //     }
    //
    //     let mut args = MySqlArguments::default();
    //     for element in elements {
    //         element.add_to_mysql_arguments(&mut args)?;
    //     }
    //     Ok(args)
    // }

    // pub fn get_sqlite_args<'a, 'b>(&'a self, args: &'b mut SqliteArguments<'b>) -> Result<(), sqlx::error::BoxDynError> where 'a: 'b {
    //     let mut stack: Vec<(&Conditions<T>, bool)> = vec![(self, false)];
    //     let mut elements: Vec<T> = Vec::new();
    //     while let Some((node, children_processed)) = stack.pop() {
    //         match node {
    //             Conditions::Group { op, children } => {
    //                 if !children_processed {
    //                     // 如果后续弹出到我自己了，那么说明我的所有子节点已经处理完成
    //                     stack.push((node, true));
    //                     for child in children.iter().rev() {
    //                         stack.push((child, false));
    //                     }
    //                 }
    //             }
    //             Conditions::Element(location_expr) => {
    //                 elements.push(location_expr.clone());
    //             }
    //         }
    //     }
    //
    //     // let mut args = SqliteArguments::default();
    //     for element in elements {
    //         element.add_to_sqlite_arguments(args)?;
    //     }
    //     Ok(())
    // }
}

#[cfg(test)]
mod test {
    use crate::location::conditions::test::UserLocationExpr::*;
    use crate::location::conditions::{to_sql, wrap_where_seg, Condition, Conditions, ConditionsWithArguments, SqlArguments};
    use crate::location::location_expr::LogicalOperator;
    use crate::{CmpOperator, LocationExpr, Optional};
    use sqlx::mysql::MySqlArguments;
    use sqlx::postgres::PgArguments;
    use sqlx::sqlite::{SqliteArguments, SqliteTypeInfo};
    use sqlx::types::Uuid;
    use sqlx::{Arguments, Encode, MySql, Sqlite, Type};
    use time::macros::format_description;
    use time::PrimitiveDateTime;

    #[derive(Debug, Clone)]
    pub enum UserLocationExpr {
        RequestId(LocationExpr<Uuid>),
        Name(LocationExpr<String>),
        Age(LocationExpr<i32>),
        Birthday(LocationExpr<PrimitiveDateTime>),
    }

    impl Condition for UserLocationExpr {
        fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
            match self {
                RequestId(request_id) => wrap_where_seg(
                    "request_id",
                    request_id.get_cmp_sql(),
                    wrap_char,
                    place_holder,
                ),
                Name(name) => wrap_where_seg("name", name.get_cmp_sql(), wrap_char, place_holder),
                Age(age) => wrap_where_seg("age", age.get_cmp_sql(), wrap_char, place_holder),
                Birthday(birthday) => {
                    wrap_where_seg("birthday", birthday.get_cmp_sql(), wrap_char, place_holder)
                }
            }
        }

        fn add_to_sqlite_arguments<'a, 'b>(
            &'a self,
            args: &mut SqliteArguments<'b>,
        ) -> Result<(), sqlx::error::BoxDynError>
        where
            'a: 'b,
        {
            match self {
                RequestId(request_id) => args.add(&request_id.val)?,
                Name(name) => args.add(&name.val)?,
                Age(age) => args.add(&age.val)?,
                Birthday(birthday) => args.add(&birthday.val)?,
            }
            Ok(())
        }

        fn add_to_mysql_arguments(
            &self,
            args: &mut MySqlArguments,
        ) -> Result<(), sqlx::error::BoxDynError> {
            match self {
                RequestId(request_id) => args.add(&request_id.val)?,
                Name(name) => args.add(&name.val)?,
                Age(age) => args.add(&age.val)?,
                Birthday(birthday) => args.add(&birthday.val)?,
            }
            Ok(())
        }

        fn add_to_postgres_arguments(
            &self,
            args: &mut PgArguments,
        ) -> Result<(), sqlx::error::BoxDynError> {
            match self {
                RequestId(request_id) => args.add(&request_id.val)?,
                Name(name) => args.add(&name.val)?,
                Age(age) => args.add(&age.val)?,
                Birthday(birthday) => args.add(&birthday.val)?,
            }
            Ok(())
        }
    }

    #[test]
    fn conditions_expr_spec() {
        let args = MySqlArguments::default();
        let mut args: SqlArguments<MySql> = SqlArguments {
            arguments: args,
            wrap_char: '`',
            place_holder: '?',
        };

        let name = ConditionsWithArguments::element(
            &mut args,
            "name",
            LocationExpr::new(CmpOperator::Eq, "Alice".to_string()),
        );

        let age = ConditionsWithArguments::element(
            &mut args,
            "age",
            LocationExpr::new(CmpOperator::Eq, 30),
        );
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let birthday = ConditionsWithArguments::element(
            &mut args,
            "birthday",
            LocationExpr::new(
                CmpOperator::Eq,
                PrimitiveDateTime::parse("2020-01-02 03:04:05", format).unwrap(),
            ),
        );

        let and_node =
            Conditions::group(LogicalOperator::And, vec![name.clone(), age]);
        let or_node = Conditions::group(LogicalOperator::Or, vec![and_node, birthday]);
        let conditions = Conditions::group(LogicalOperator::And, vec![or_node, name]);

        let sql_expression = to_sql(&conditions, '`', '?').unwrap();

        assert_eq!(
            "(`name`=? AND `age`=? OR `birthday`=?) AND `name`=?",
            sql_expression
        );
    }
}
