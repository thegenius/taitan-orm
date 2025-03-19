# 关于Template的语法解析器
```
${}的支持合并到rinja template
____________________________     ______________________
| rinja template render    |     |   @{} placeholder  |    
----------------------------     ----------------------
       |                                      |
       |    ______________________________    |
       |->  |      #{} placeholder       |  <-|   
            --------------_---------------
        __________________|
       |
       |    ______________________________
       |->  |           ?/$1             |     
            ------------------------------           


pub struct StaticRenderedSql {
    sql: &'static str,
}

pub struct DynamicRenderedSql {
    sql: String,
    variables: Vec<String>
}

           
#[derive(Template)]
#[sql=""]
pub struct UserTemplate {
   a: &str,               // 视为required 字段
   b: Option<&str>,       // None视为NULL，Some(T)视为正常值
   b: Option<Option<&str> // None视为忽略，Some(None)视为NULL，Some(T)视为正常值
}         

impl UserTemplate {
   pub fn get_template_sql(&self)->String {
       // {% %} 和 {{ }} 模板参数原样保持
       // @{} 且类型为Option<Option<T>> 替换为 {% if xxx.is_some() %} #{} {% endif %}
   }
   pub fn get_rendered_sql(&self) -> RenderedSql {
       let mut rendered_sql = self.render();
       rendered_sql = 替换所有#{} 为?/$n
   }
   pub fn get_rendered(&self) -> (String, Arguments) {
       let rendered_sql = self.get_rendered_sql();
       let mut args = Arguments::default();
       for variable in rendered_sql.variables {
           mat
       }
   }
}

  
```

解析过程
```
lexer:
        __________________
&str -> ｜ Generic Token |
        ------------------
pub enum ValueToken {
    Null,
    Number(Number),
    Text(Text),
    Bool(Bool),
}
        
pub enum ExprToken {
    Value(ValueToken),
    Operator(Operator),           // 各类操作符+-*/% like in = > < <> !=
    VariableChain(Chain),
    Placeholder(Placeholder),
    AmbiguousSign(AmbiguousSign), // * + - 现在这个3个符号有二义性
    Sign(Sign),                   // 各种特殊符号，例如括号()[]{}等
}   

enum GenericToken {
   Keyword(&'static str),
   TemplatePart(TemplatePart),
   ExprToken(ExprToken)
}        
        
根据Token的类型，可以分为几个大类
（1）只包含#{name}，这类可以直接静态编译
（2）包含{% %} {{}}的模板，直接动态编译
（3）包含@{name}的动态可选参数类型，需要解析为表达式
（4）同时包含{% %}和@{name}报错，不允许        
```
```
syntax parser
将Token List解析成
（1）Keyword, Expr, Keyword, Expr
（2）Keyword, TemplatePart, Keyword, TemplatePart
```


## 字符流转化为Atomics
(1)AtomicValue
Bool
Text
Number: digit, not include +/-
Template: {% if %} {% endif %}
Placeholder: #{} @{} ${}
VariableChain: a.b.c

(2)Sign
Bracket: () [] {}
Comma

(3)Operator
   Arithmetic: Add Sub Mul Div Mod
   Logic: And Or Not
   Compare: = >= <= > < <> !=
   Match: Like In





## basic expr
(1) 数学表达式 a + b - c / d
(2) 比较表达式 a > c
(3) 赋值表达式 a = 12
(4) 简单修饰 not a >= c

## complicate expr
(1) logic and/or/()/not
(2) comma



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
________________________________________________
|          template          |   when option   |  
------------------------------------------------
| name = @{name};            |                 |    
| name = @{name}, age = ?;   | age = ?         |
| name = ?, age=@{age}       | name = ?        | 
| name = @{name}, age=@{age} |                 | 
------------------------------------------------
```
```
Variable:
   Literal
   Hash
   Dollar
   At

Comperator:
   <=
   >=
   =
   !=
   
Connective
   Comma
   And
   Or
   
SimpleExpr:
   Variable Comperator Variable   
 
ConnectedExpr
   CommaExpr
   AndExpr
   OrExpr
   
Decorator:
   Not
   Paren

```




```rust
fn gen_set_sql(&self)->Cow<'a, str>;
```

location的condition语法
```text
___________________________________________________
|          template              |   connective   |  
------------------------------------------------------------------------------------------- 
| name <= @{name} AND age = ?;   | {% if name.is_some() %} AND {% endif %} |
| name <= ?       AND age=@{age} | {% if age.is_some %} AND {% endif %} | 
| name <= @{name} AND age=@{age} | {% if name.is_some() &&  age.is_some %} AND {% endif %} | 
--------------------------------------------------------------------------------------------
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