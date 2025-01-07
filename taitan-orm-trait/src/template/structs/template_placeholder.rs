use crate::template::structs::template_variable_chain::TemplateVariableChain;
use crate::template::ToSql;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatePlaceholder {
    Dollar(TemplateVariableChain),
    Hash(TemplateVariableChain),
    Percent(TemplateVariableChain),
}

impl TemplatePlaceholder {
    pub fn to_sql(&self) -> String {
        match self {
            TemplatePlaceholder::Dollar(val) => format!("{{{{{}}}}}", val.to_set_sql()),
            TemplatePlaceholder::Hash(val) => "?".to_string(),
            TemplatePlaceholder::Percent(val) => format!(
                "{{% if {}.is_some() %}}{{{{{}.unwrap()}}}}{{% elif {}.is_null() %}}NULL{{% else %}}{{% endif %}}",
                val.to_set_sql(),
                val.to_set_sql(),
                val.to_set_sql(),
            ),
        }
    }
}

impl Display for TemplatePlaceholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplatePlaceholder::Dollar(val) => write!(f, "{}", val),
            TemplatePlaceholder::Hash(val) => write!(f, "{}", val),
            TemplatePlaceholder::Percent(val) => write!(f, "{}", val),
        }
    }
}

/// hash signs应该被转化为 ?
/// dollar signs应该被转化为 {{ var }}，这样可以被渲染
/// percent expr 应该被转化为
/// {% if val.is_some() %}
/// {{val.unwrap()}}
/// {% elif val.is_null() %}
/// NULL
/// {% else %}
/// {% endif %}
impl ToSql for TemplatePlaceholder {
    fn to_set_sql(&self) -> String {
        self.to_sql()
    }

    fn to_where_sql(&self) -> String {
        self.to_sql()
    }
}

#[cfg(test)]
mod tests {
    use crate::template::TemplatePlaceholder;
    use crate::template::TemplateVariable;
    use crate::template::TemplateVariableChain;
    use crate::template::ToSql;

    #[test]
    fn test_to_set_sql() {
        let placeholder = TemplatePlaceholder::Hash(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("var".to_string())],
        });
        let sql = placeholder.to_set_sql();
        assert_eq!(sql, "?");


        let placeholder = TemplatePlaceholder::Dollar(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("var".to_string())],
        });
        let sql = placeholder.to_set_sql();
        assert_eq!(sql, "{{var}}");

        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("var".to_string())],
        });
        let sql = placeholder.to_set_sql();
        assert_eq!(sql, "{% if var.is_some() %}{{var.unwrap()}}{% elif var.is_null() %}NULL{% else %}{% endif %}");

    }
}
