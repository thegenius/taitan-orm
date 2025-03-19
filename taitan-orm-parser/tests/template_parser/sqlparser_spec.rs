use sqlparser::ast::Statement;
use sqlparser::dialect::Dialect;
use sqlparser::parser::{Parser, ParserError};
use sqlparser::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
struct CustomPlaceholderDialect;


fn parse_placeholder_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Colon => {
                // let current_token = &token;
                // 检查下一个 Token 是否是 `{`
                if let Some(Token::LBrace) = iter.peek() {
                    iter.next(); // 消耗 `{`
                    let mut placeholder = ":{".to_string();
                    // 解析占位符内容
                    while let Some(token) = iter.next() {
                        match token {
                            Token::RBrace => {
                                iter.next(); // 结束占位符
                                placeholder.push('}');
                                break;
                            }
                            Token::Word(id) => placeholder.push_str(&id.value),
                            _ => placeholder.push_str(&token.to_string()),
                        }
                    }
                    // 将占位符转换为 Token::Placeholder
                    result.push(Token::Placeholder(placeholder));
                } else {
                    // 如果不是占位符，保留原始 Token
                    result.push(token);
                }
            }
            _ => result.push(token),
        }
    }
    result
}


pub fn parse_sql(sql: &str, dialect: &dyn Dialect) -> Result<Vec<Statement>, ParserError> {
    let mut tokenizer = Tokenizer::new(dialect, sql);
    let tokens = tokenizer.tokenize()?;
    let tokens = parse_placeholder_tokens(tokens);
    let parser = Parser::new(dialect);
    let statements = parser.with_tokens(tokens).parse_statements()?;
    Ok(statements)
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlparser::dialect::Dialect;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::dialect::MySqlDialect;
    use sqlparser::dialect::PostgreSqlDialect;
    use sqlparser::dialect::SQLiteDialect;
    use sqlparser::keywords::Keyword::{NoKeyword, AND, ID, NAME};
    use sqlparser::parser::Parser;
    use sqlparser::tokenizer::Whitespace::Space;
    use sqlparser::tokenizer::Word;


    fn test_dialect_placeholder<'a>(dialect: &'a dyn Dialect) {
        let sql = "name=:{name}";
        let mut tokenizer = Tokenizer::new(dialect, sql);
        let tokens = tokenizer.tokenize().unwrap();
        let tokens = parse_placeholder_tokens(tokens);
        let expected = vec![
            Token::Word(Word {
                value: "name".to_string(),
                quote_style: None,
                keyword: NAME,
            }),

            Token::Eq,
            Token::Placeholder(":{name}".to_string()),
        ];
        assert_eq!(expected, tokens);
    }
    #[test]
    fn test_custom_placeholder() {
        test_dialect_placeholder(&PostgreSqlDialect {});
        test_dialect_placeholder(&MySqlDialect {});
        test_dialect_placeholder(&SQLiteDialect {});
    }


    fn test_parser_dialect<'a>(dialect: &'a dyn Dialect) {
        let sql = "SELECT name FROM users WHERE id=:{id}";
        let statements = parse_sql(sql, dialect).unwrap();
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn test_parse_sql() {
        test_parser_dialect(&PostgreSqlDialect {});
        test_parser_dialect(&MySqlDialect {});
        test_parser_dialect(&SQLiteDialect {});
    }

}
