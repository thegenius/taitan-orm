
## Quick Start
### 4 Main Read API
Because unique only locate to 0-1 record, so order and page is not needed.
```
select       (selection, unique               ) -> Optional<SE>
search       (selection, location, order, page) -> Vec<SE>
search_all   (selection, location, order      ) -> Vec<SE>
search_paged (selection, location, order, page) -> PagedList<SE>
```

### 6 Sugar Read API
Other read APIs are just syntactic sugar and maybe some performance optimize.
```
# equals to: select(selection::full, unique).is_some()
exists(unique) -> bool

# equals to: select_all(selection::full, location).len()
count(location) -> u64

# equals to: select(selection::full, unique) -> Optional<SE>
select_full(unique) -> Optional<SE>

# equals to: search(selection::full, location, order, page) -> Vec<SE>
search_full(location, order, page) -> Vec<SE>

# equals to: search_all(selection::full, order) -> Vec<SE>
search_full_all(location, order) -> Vec<SE>

# equals to: search_paged(selection::full, location, order, page) -> PagedList<SE>
search_full_paged(location, order, page) -> PagedList<SE>
```


## Cognitive Model

### Term: 
```
select: read 0-1 rows from database, with unique as condition
search: read 0-n rows from database, with location as condition

unique: a special where condition, only locate to 0-1 row
location: a normal where condition, locate to 0-n row

order: the ordering of rows, must include at least 1 unique index
page: the page info

all: return all rows that satisfied with the condition
paged: input page info, return PagedList as result
full: ignore selection params，select all the fields
```

### Read Model:
```text
_______________________________________
| selection | location | order | page |
---------------------------------------
     |          |         |       |--> sql{ limit 200, 100 }
     |          |         |--> sql{ order by age, id }
     |          |--> sql{ where name = '' }
     |-->sql{ select name, age, id from `user` }
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