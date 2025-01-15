

_    : 
order: 凡是返回列表的查询API，都需要提供order，这就保证了查询结果的确定性
all  : 参数不提供page，返回Vec
paged: 参数需要提供page, 返回PagedList
full : 表示省略了入参的selection，默认查询表的所有字段 


```
exists(unique)   -> bool
count(location)  -> u64
count_table(table) -> u64 // maybe merged into count(location)

select       (selection, unique                ) -> Option<SE>
search       (selection, location,  order, page) -> Vec<SE>
devour       (selection,            order, page) -> Vec<SE>
search_all   (selection, location,  order)       -> Vec<SE>
devour_all   (selection,            order)       -> Vec<SE>
search_paged (selection, location,  order, page) -> PagedList<SE>
devour_paged (selection,            order, page) -> PagedList<SE>
```


```
select_full       (unique               ) -> Option<SE>
search_full       (location, order, page) -> Vec<SE>
devour_full       (          order, page) -> Vec<SE>
search_full_all   (location, order      ) -> Vec<SE>
devour_full_all   (          order      ) -> Vec<SE>
search_full_paged (location, order, page) -> PagedList<SE>
devour_full_paged (          order, page) -> PagedList<SE>

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