

_    : 
all  : 返回Vec
paged: 返回PagedList



```
exists(unique)   -> bool
count(location)  -> u64
count_all(table) -> u64

select      (selection, unique)                         -> Option<SE>
search      (selection, location,  order_opt, page_opt) -> Vec<SE>
devour      (selection,            order_opt, page_opt) -> Vec<SE>
search_paged(selection, location,  order, page) -> PagedList<SE>
devour_paged(selection,            order, page) -> PagedList<SE>
```


_    : 默认需要提供字段筛选
full : 所有字段

```
select_full      (unique)                        -> Option<SE>
search_full      (location, order_opt, page_opt) -> Vec<SE>
devour_full      (          order_opt, page_opt) -> Vec<SE>
search_full_paged(location, order_by, page)      -> PagedList<SE>
devour_full_paged(          order_by, page)      -> PagedList<SE>

```