# sqlx在最底层提供的能力如下
1. 把sql + args合并为query结构
2. args默认实现了Default trait，可以通过sqlx::query_with(query, Default::default())生成query
3. query.execute(EX)可以执行一个操作，其中EX要求是Executor trait的实现  
4. 返回了4类结果

| func         | result      | descption                                |
|--------------|-------------|------------------------------------------|
| execute      | QueryResult | affect_rows                              |
| fetch_all    | Vec<Row>    | if no row, Vec will be empty             |
| fetch_one    | Row         | will rise RowNotFound Error if not found |
| fetch_option | Option<Row> | fetch at most one Row                    |


# GenericExecutor的设计
1. 结果转化层的封装
QueryResult -> u64
Vec<Row>    -> Vec<SE>
Row         -> SE
Option<Row> -> Option<SE>
2. 屏蔽query对象的构建
3. connection 和 transaction要都能够传入
4. 不同数据库的args都可以泛型传入
generic_exists            (ex, stmt, args) -> Result<bool>
generic_exists_plain      (ex, stmt, _   ) -> Result<bool>
generic_execute           (ex, stmt, args) -> Result<u64>
generic_execute_plain     (ex, stmt, _   ) -> Result<u64>
generic_fetch_all         (ex, stmt, selection, args) -> Result<Vec<SE>>
generic_fetch_all_plain   (ex, stmt, selection, _   ) -> Result<Vec<SE>>
generic_fetch_one         (ex, stmt, selection, args) -> Result<SE>
generic_fetch_one_plain   (ex, stmt, selection, _   ) -> Result<SE>
generic_fetch_option      (ex, stmt, selection, args) -> Result<Option<SE>>
generic_fetch_option_plain(ex, stmt, selection, _   ) -> Result<Option<SE>>
generic_fetch_all_full         (ex, stmt, args) -> Result<Vec<SE>>
generic_fetch_all_full_plain   (ex, stmt, _   ) -> Result<Vec<SE>>
generic_fetch_one_full         (ex, stmt, args) -> Result<SE>
generic_fetch_one_full_plain   (ex, stmt, _   ) -> Result<SE>
generic_fetch_option_full      (ex, stmt, args) -> Result<Option<SE>>
generic_fetch_option_full_plain(ex, stmt, _   ) -> Result<Option<SE>>

# Executor/Transaction的设计
屏蔽掉ex，让更上层的API层不再感知connection/transaction
但是现在sqlx的Executor实现还无法完全做到屏蔽
现在通过GenericExecutor的泛型函数，大约可以把代码简化到160行
但是数据库 N + 事务N，现在支持sqlite+mysql+postgres需要重复6遍这160行代码  

exists            (stmt, args) -> Result<bool>
exists_plain      (stmt, _   ) -> Result<bool>
execute           (stmt, args) -> Result<u64>
execute_plain     (stmt, _   ) -> Result<u64>
fetch_all         (stmt, selection, args) -> Result<Vec<SE>>
fetch_all_plain   (stmt, selection, _   ) -> Result<Vec<SE>>
-- fetch_one         (stmt, selection, args) -> Result<SE> -- skip
-- fetch_one_plain   (stmt, selection, _   ) -> Result<SE> -- skip
fetch_option      (stmt, selection, args) -> Result<Option<SE>>
fetch_option_plain(stmt, selection, _   ) -> Result<Option<SE>>
fetch_all_full         (stmt, args) -> Result<Vec<SE>>
fetch_all_full_plain   (stmt, _   ) -> Result<Vec<SE>>
fetch_one_full         (stmt, args) -> Result<SE>
fetch_one_full_plain   (stmt, _   ) -> Result<SE>
fetch_option_full      (stmt, args) -> Result<Option<SE>>
fetch_option_full_plain(stmt, _   ) -> Result<Option<SE>>






# API设计

## 更新操作API设计
```
async fn insert(entity) -> Result<bool>                # if not exists -> insert, if exists -> fails   
async fn upsert(entity) -> Result<bool>                # if not exists -> insert, if exists -> update  
async fn create<'a>(&'a entity) -> Result<&'a entity>  # if not exists -> insert, if exists -> fails  

async fn update(mutation, unique)   -> Result<bool> 
async fn change(mutation, location) -> Result<u64>
    
async fn delete(unique)   -> Result<bool>
async fn purify(location) -> Result<u64>
```


## 读取操作API设计
```
async fn exists(unique)  -> Result<bool>
async fn count(location) -> Result<u64>
async fn count_all(name) -> Result<u64>

async fn select<SE>(selection, unique)                                 -> Result<Option<SE>>
async fn search<SE>(selection, location, order_by_option, page_option) -> Result<Vec<SE>>
async fn devour<SE>(selection,           order_by_option, page_option) -> Result<Vec<SE>>

async fn search_paged<SE>(selection, location, order_by, page) -> Result<PagedList<SE>>
async fn devour_paged<SE>(selection,           order_by, page) -> Result<PagedList<SE>>
```

## 模板操作API设计
```
async fn execute_by_template(template)          -> Result<u64>

async fn fetch_one_by_template<SE>(template)    -> Result<SE>
async fn fetch_option_by_template<SE>(template) -> Result<Option<SE>>
async fn fetch_all_by_template<SE>(template)    -> Result<Vec<SE>>
async fn fetch_paged_by_template<SE>(template)  -> Result<PagedList<SE>>
```



trait: 从struct中通过宏生成基本的信息，相当于其他语言的反射，只不过是编译态的反射
sql generator: 通过trait反射回来的基本信息，生成必要的SQL语句
sql executor: 把struct转化为arguments, 把row转化为SelectedEntity

sql api: 操作API
\[database\]_commander: 实现sql api
\[database\]_transaction_commander: 实现sql api，但是传入事务，而不是传入连接 


## API Design

```rust
async fn insert(entity: &dyn Entity) -> Result<bool>;
async fn upsert(entity: &dyn Entity) -> Result<bool>;
async fn update<M: Mutation>(mutation: &M, unique: &M::Primary) -> Result<bool>;
async fn change<M: Mutation>(mutation: &M, location: &M::Location) -> Result<u64>;
async fn delete(unique: &dyn Unique) -> Result<bool>;
async fn purify(location: &dyn Location) -> Result<u64>;
```

```rust
async fn select<SE>(
    selection: &SE::Selection, 
    unique: &dyn Unique
) -> Result<Option<SE>>;

async fn search<SE>(
    selection: &SE::Selection, 
    location: &dyn Location, 
    order_by: &dyn OrderBy
) -> Result<Vec<SE>>;

async fn search_paged<SE>(
    selection: &SE::Selection,
    location: &dyn Location,
    order_by: &dyn OrderBy,
    page: &Pagination,
) -> Result<PagedList<Self::DB, SE>>;

async fn devour<SE>(
    selection: &SE::Selection, 
    order_by: &dyn OrderBy
) -> Result<Vec<SE>>;

async fn devour_paged<SE>(
    selection: &SE::Selection, 
    order_by: &dyn OrderBy, 
    page: &Pagination
) -> Result<PagedList<Self::DB, SE>>;

async fn count(location: &dyn Location) -> Result<u64>;

async fn count_table(table_name: &str) -> Result<u64>;

```



## executor design
```rust
 async fn fetch_optional_plain<'a, SE>(
    stmt: &'a str,
    selection: &'a SE::Selection,
) -> Result<Option<SE>>;

async fn fetch_optional<'a, SE>(
    stmt: &'a str,
    selection: &'a SE::Selection,
    args: <Self::DB as Database>::Arguments<'a>,
) -> Result<Option<SE>>;

async fn fetch_all_plain<'a, SE>(
    stmt: &'a str,
    selection: &'a SE::Selection,
) -> Result<Vec<SE>>;

async fn fetch_all<'a, SE>(
    stmt: &'a str,
    selection: &'a SE::Selection,
    args: <Self::DB as Database>::Arguments<'a>,
) -> Result<Vec<SE>>;

async fn execute_plain<'a>(
    stmt: &'a str
) -> Result<u64>;

async fn execute<'a, A>(
    stmt: &'a str,
    args: <Self::DB as Database>::Arguments<'a>,
) -> Result<u64>;

async fn fetch_execute_plain<'a, SE>(
    stmt: &'a str
) -> Result<SE>;

async fn fetch_execute<'a, SE>(
    stmt: &'a str,
    args: <Self::DB as Database>::Arguments<'a>,
) -> Result<SE>;
```


```
________________     ________________    ______________
|  mysql api   |     | postgres api |    | sqlite api |
------^---------     --------^-------    -------^------
      |                      |                  |
      |                      |                  |
      -----------------------^-------------------
                             |    
     ________________________________________________
     | ArgsExtractor + SqlGenerator + ResultMapping |
     ------------------------------------------------

                    Exector + ExecutorMut
                       GenericExecutor

```