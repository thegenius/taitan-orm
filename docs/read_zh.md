

_    : 
order: 凡是返回列表的查询API，都需要提供order，这就保证了查询结果的确定性
all  : 参数不提供page，返回Vec
paged: 参数需要提供page, 返回PagedList
full : 表示省略了入参的selection，默认查询表的所有字段 

## Read APIs cognitive model:
```text
_______________________________________
| selection | location | order | page |
---------------------------------------
     |          |         |       |--> sql{ limit 200, 100 }
     |          |         |--> sql{ order by age, id }
     |          |--> sql{ where name = '' }
     |-->sql{ select name, age, id from `user` }
```
# basic read api
because unique only locate to 0-1 record, so order and page is not needed.
```text
select       (selection, unique               ) -> Option<SE>
search       (selection, location, order, page) -> Vec<SE>
search_all   (selection, location, order      ) -> Vec<SE>
search_paged (selection, location, order, page) -> PagedList<SE>
```

# Other read APIs are just syntactic sugar
If selection is not need
```
select_full       (unique               ) -> Option<SE>
search_full       (location, order, page) -> Vec<SE>
search_full_all   (location, order      ) -> Vec<SE>
search_full_paged (location, order, page) -> PagedList<SE>
```
## location可能实现all()方法，返回一个实现了Location trait的结构体，其get_where_sql()方法返回空
如果这样实现，可以省略掉6个API

## If we just, location is not need, 
```
devour       (selection,            order, page) -> Vec<SE>
devour_all   (selection,            order)       -> Vec<SE>
devour_paged (selection,            order, page) -> PagedList<SE>

devour_full       (          order, page) -> Vec<SE>
devour_full_all   (          order      ) -> Vec<SE>
devour_full_paged (          order, page) -> PagedList<SE>
```

```
exists(unique)     -> bool
count(location)    -> u64
count_table(table) -> u64   // maybe merged into count(location)
```

## Unique trait
1. 自动生成的struct EntityPrimary是一个trait
2. 自动生成的struct EntityUnique是一个trait

## Location trait
1. 自动生成的struct EntityIndex是一个trait
2. 单行条件是Location trait
3. 简单组合条件是Location trait

```



```