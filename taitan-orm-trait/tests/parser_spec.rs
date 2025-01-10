use taitan_orm_trait::ParsedTemplateSql;

/// not 语句只能转化成where sql，不能转化成set sql
/// not expr的子元素如果是optional的，需要向上传递到not
/// not age = %{age} 应该渲染为
/// {% if age.is_some() %}not age =  ?{% else if age.is_null() %}not age is null{% else %}{% endif %}
/// not age <> %{age} 应该渲染为
/// {% if age.is_some() %}not age <> ?{% else if age.is_null() %}age is null{% else %}{% endif %}
/// not age >= %{age} 应该渲染为
/// {% if age.is_some() %}not age >= ?{% else %}{% endif %}
/// not (age = %{age} AND name = %{name})应该渲染为
/// {% if age.is_some() && name.is_some() %}NOT {child_expr}{% else %}{child_expr}{% endif %}
#[test]
fn not_expr_spec_01() {
    let parsed = ParsedTemplateSql::parse("NOT age = %{age}").unwrap();
    let sql = parsed.get_where_sql();
    // assert_eq!(sql, "{% if age.is_some() %}not age =  ?{% else if age.is_null() %}not age is null{% else %}{% endif %}");
    assert_eq!(
        sql,
        "NOT {% if age.is_some() %}age = ?{% elif age.is_null() %}age IS NULL{% else %}{% endif %}"
    );
}
