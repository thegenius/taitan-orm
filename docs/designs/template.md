


# template怎么转化为Arguments
if e.is_some() {
   args.add(e)?;
}

# template怎么获取结果
任何实现了Selected trait的结构都可以用于获取结果

# template怎么转化成sql
```
#[template(sql="select name from `user`")]
pub struct UserTemplate {
  name: String
}
```
其中转化为正确的sql是最为复杂的，
（1）怎么实现变量名称绑定
${name}: 字符串替换
#{name}: 替换为?，sql在编译期就
@{name}: option替换
本质上替换\${name}和#{name}都相对简单，甚至不需要支持完整的sql解析
但是@{name}对option的支持，就会导致需要解析expr，才能够做完整的expr忽略

mutation的set语法 
```text
|          template        |   when option   | comma condition |
----------------------------------------------------------------
| name = @{name};            |                 |                                                     |
| name = @{name}, age = ?;   | age = ?         | {% if name.is_some() %},{%endif%}                   |
| name = ?, age=@{age}       | name = ?        | {% if age.is_some()  %},{%endif%}                   |
| name = @{name}, age=@{age} |                 | {% if name.is_some() && age.is_some() %},{%endif%}  |
```


```rust
pub enum TemplateExpr {
    Simple {
        first_part: TemplateExprFirstPart,
        operator: String,
        second_part: TemplateExprSecondPart,
        optional_context: UnitOptionalContext,
    },
    Not {
        expr: Box<TemplateExpr>,
        optional_context: OptionalContext,
    },
    Parenthesized {
        expr: Box<TemplateExpr>,
        optional_context: OptionalContext,
    },
    And {
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        optional_context: PairOptionalContext,
    },
    Or {
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        optional_context: PairOptionalContext,
    },
}
```

(2) 是否需要支持原来的rinja模板替换
如果要支持rinja，就要解析 {% %} 和 {{ }}

(3)
```
#[sql = ""] //
struct Template {

}
```