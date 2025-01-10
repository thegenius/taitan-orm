use taitan_orm_trait::{ParsedTemplateSql, TemplateSqlValue};

/// not 语句只能转化成where sql，不能转化成set sql
/// 1. 关于not的嵌套
///    多个not直接嵌套后会被优化为单个not
/// 2. 关于not的子元素为simple
///    2.1 not expr的子元素如果是optional的，且比较符是 = 和 <>
///         2.1.1 not age = %{age} 应该渲染为
///             {% if age.is_some() %}not age =  ?{% else if age.is_null() %} age IS NOT NULL {% else %}{% endif %}
///         2.1.2 not age <> %{age} 应该渲染为
///             {% if age.is_some() %}not age <> ?{% else if age.is_null() %} age IS NULL {% else %}{% endif %}
///    2.2 not expr的子元素如果是optional的，且比较符不是 = 和 <>
///         not age >= %{age} 应该渲染为
///         {% if age.is_some() %}not age >= ?{% else %}{% endif %}
/// 3. 关于not的子元素是()或者and或者or
/// not (age = %{age} AND name = %{name})应该渲染为
/// {% if age.is_some() && name.is_some() %} NOT {% endif %} (age = ? AND name = ?)
#[test]
fn not_expr_spec_01() {
    let parsed = ParsedTemplateSql::parse("NOT age = %{age}").unwrap();
    let sql = parsed.get_where_sql();
    assert_eq!(sql, "{% if age.is_some() %} NOT age = ? {% else if age.is_null() %} age IS NOT NULL {% else %}{% endif %}");
}


#[test]
fn and_expr_spec_01() {
    let parsed = ParsedTemplateSql::parse("age = %{age} AND name = %{name}").unwrap();
    let sql = parsed.get_where_sql();
    assert_eq!(sql, "{% if age.is_some() %}age = ?{% elif age.is_null() %}age IS NULL{% else %}{% endif %} {% if age.is_some() || age.is_null() && name.is_some() || name.is_null() %} AND {% endif %} {% if name.is_some() %}name = ?{% elif name.is_null() %}name IS NULL{% else %}{% endif %}");
}
