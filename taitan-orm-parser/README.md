


# Info parser

struct parser

attribute parser

field parser


# Field Mapper
```
(1) name              : `field`
(2) upsert            : `field`=VALUES(field)
(3) mark              : `?`  
(4) mark indexed      : `$1`  
(5) set               : `field`=? 
(6) set indexed       : `field`=$1
(7) condition         : `field` {} ? 
(8) condition indexed : `field` {} $1
```

# Connector：本质上就是为了生成逗号

```
// 6个类型
// (1) LeadingRequired，第一组的required字段，前面没有Option
// (2) FailingRequired，第一组required字段，但是前面有option字段
// (3) TrailingRequired，非第一组的required字段
// (4) LeadingOptional，第一个optional字段，且前面没有required字段
// (5) FollowingOptional，非第一个optional字段，且前面没有required字段
// (6) TrailingOptional，optional字段，无论是否是第一个，只有前面有Required字段，就是TrailingOptional
__________________________________________________________________
              | Leading-R | Failing-R | Trailing-R | Optional(*) |
------------------------------------------------------------------
names         |    Y      |    Y      |     Y      |      N      |
upsert_sets   |    Y      |    Y      |     Y      |      N      |
marks         |    Y      |    Y      |     Y      |      N      |
sets          |    Y      |    Y      |     Y      |      N      |
marks_indexed |    Y      |    N      |     N      |      N      |
sets_indexed  |    Y      |    N      |     N      |      N      |
condition(*)  |    N      |    N      |     N      |      N      |
------------------------------------------------------------------


____________________________________________________________________________________________
              |       static     |     dynamic      |   static-indexed |  dynamic-indexed  |
--------------------------------------------------------------------------------------------
names         |       {n}        |        {n}       |        {n}       |       {n}         |  
upsert_sets   |   {n}=VALUES(n)  |   {n}=VALUES(n)  |   {n}=VALUES(n)  |  {n}=VALUES(n)    |
marks         |         ?        |         ?        |        $1        |       ${}         |
sets          |      {n}=?       |      {n}=?       |      {n}=$1      |     {n}=${}       |
exprs         |      panic!      |      {n}{}?      |      panic!      |     {n}{}${}      |
--------------------------------------------------------------------------------------------

map_static          (field, escaper)        ->Cow<'a, str>
map_static_indexed  (field, escaper, index) ->Cow<'a, str>
map_dynamic         (field, escaper)        ->TokenStream
map_dynamic_indexed (field, escaper)        ->TokenStream

map_static_fields         (fields, escaper) ->Cow<'a, str>
map_static_indexed_fields (fields, escaper) ->Cow<'a, str>



connect(fields, escaper): [names, upserts, marks, sets]
   groups = fields.split_into_group
      LeadingRequired : map_static_fields + has_prev=true
      FailingRequired : map_static_fields + has_prev=true
      TrailingRequired: map_static_fields 
      
      LeadingOptional  : translate(Optional, NoLeading)
      FollowingOptional: translate(Optional, CheckedLeading)
      TrailingOptional : translate(Optional, Leading)

#[marks_indexed, sets_indexed]
connect_indexed(fields, escaper): 
   groups = fields.split_into_group
      LeadingRequired : map_static_fields + has_prev=true
      FailingRequired :
          i  = 0: translate(Required, CheckedLeading) + has_prev=true
          i != 0: translate(Required, Leading)
      TrailingRequired: translate(Required, Leading) 
      
      LeadingOptional  : translate(NoLeading)
      FollowingOptional: translate(CheckedLeading)
      TrailingOptional : translate(Leading)

#[exprs, exprs_indexed]
connect_expr(fields, escaper, indexed)
   groups = fields.split_into_group
      LeadingRequired : 
          i  = 0: translate(Required, NoLeading, indexed) + has_prev=true
          i != 0: translate(Required, Leading)
      FailingRequired :
          i  = 0: translate(Required, CheckedLeading) + has_prev=true
          i != 0: translate(Required, Leading)
      TrailingRequired: translate(Required, Leading) 
      
      LeadingOptional  : translate(NoLeading)
      FollowingOptional: translate(CheckedLeading)
      TrailingOptional : translate(Leading)


```
field分为2个类型  
(1) required字段，可以聚集为一组  
这部分单独出来是因为可以进行非常多的compile time优化
```
1.1 一组required字段位于头部，可能编译期完成连接
    整组直接用逗号连接，添加has_prev = true  
1.2 一组required字段不位于头部，是第一组required字段组
    组内第一个字段需要判断has_prev来确定是否添加逗号，需要添加has_prev=true
    组内后续字段不需要判断has_prev，直接添加前置逗号，不添加has_prev=true  
1.3 一组required字段不位于头部，不是第一组required字段组
    组内字段不需要判断has_prev，直接添加前置逗号，不添加has_prev=true  
```

(2) optional字段，必须单独处理
```
2.1 前面没有required字段组
    需要判断has_prev来确定是否添加逗号，需要添加has_prev=true
2.2 前面有required字段组
    直接添加前置逗号，不需要has_prev=true
```



```
[ leading required ] { [ optional ] [ required ] } *

```


# SQL 参数化测试验证系统
```text
{ input_name: DeriveInput } 
[ input_name, DatabaseType, SqlType, ExpectedSql ]
```

