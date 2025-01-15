
# Write API
写入API的设计原则如下：
### 第一原则: 最小化心智负担
 目前只需要理解4个概念，就能完全掌握这些API  
（1）entity，就是表结构的直接映射，entity的字段应该和表结构一致  
（2）mutation，就是表的更新操作，包含除主键外所有字段的optional  
（3）unique，就是表的唯一索引，包含主键，在表中可以唯一定位到一行记录  
（4）location，就是表的条件搜索，可以定位到表中0-n条记录  
### 第二原则：最少的API来实现一套逻辑完备的写入操作
当前的最小集只有10个API，其中7个是常规写入操作，3个是批量操作。
### 第三原则：最大化程序员开发体验
第三条解释起来最麻烦，但基本都有明确的示例：  
 (1) insert/upsert/create返回bool，而不是int，
直接暴露底层的int会把非常多的复杂性带入到使用，
比如insert on duplicate可能返回理论上只可能返回0，1，所以true和false比int更加清晰
 (2) create需要处理自增ID生成的逻辑，不同数据库完全不一样，框架把这些复杂性统一掉
 (3) entity/mutation/unique/location都是trait，完全可以自定义


# Write API
The design principles of the Write API are as follows:

### First Principle: Minimize Cognitive Load  
To fully understand and use these APIs, you only need to grasp four core concepts:  
Entity: An entity is a direct mapping of a database table structure. The fields of an entity should match those of the table.  
Mutation: A mutation represents update operations on a table. It includes all fields of optional type except the primary key.  
Unique: A unique constraint refers to the unique index of a table, including the primary key. It uniquely identifies a single record in the table.  
Location: A location specifies condition-based searches within a table, which can locate 0 to n records.
### Second Principle: Minimal API Set for Complete Write Operations    
The current minimal set consists of only 10 APIs, with 7 being standard write operations and 3 for batch operations.

### Third Principle: Maximize Developer Experience  
This principle is the most complex to explain but generally comes with clear examples:

1. insert/upsert/create return bool instead of int.  
Exposing the underlying integer can introduce significant complexity into usage.  
For instance, insert on duplicate might theoretically return only 0 or 1.    
Therefore, using true and false is clearer than using integers.  

2. create handles auto-increment ID generation logic:
Different databases handle auto-increment IDs differently. The framework abstracts this complexity to provide a unified experience.  
3. entity/mutation/unique/location are traits and can be customized:  
These concepts are implemented as traits, allowing for full customization by developers.
```
insert(entity) -> ()
upsert(entity) -> ()
create(entity) -> ()

update(mutation, unique  ) -> bool;
change(mutation, location) -> u64;

delete(unique  ) -> bool;
purify(location) -> u64;
```

# batch
```
batch_insert([entity])                 -> ()
batch_insert_ignore_conflict([entity]) -> ()
batch_upsert([entity])                 -> ()
```

## insert 
```rust
cognitive sense    : insert(entity) -> bool
function  signature: async fn insert(&self, entity: &dyn Entity) -> Result<bool>
```
插入操作是最基本的写入操作，来执行表的insert操作，他的心智模型就是简单的插入，如果遇到冲突就插入失败。
由于表字段可能有default和not null约束，还可能是auto increment，entity的字段类型可能是optional的。

|                  | not optional         | optional::None   | optional::Null       | optional::Some  |
|------------------|----------------------|------------------|----------------------|-----------------|
| -                | ✅                    | null             | null                 | ✅               |
| default          | ✅                    | default          | null                 | ✅               |
| not null         | ✅                    | run time error   | null, run time error | ✅               |
| not null default | ✅                    | default          | null, run time error | ✅               |
| auto increment   | ❌ compile time error | ✅                | null, run time error |  run time error |

从工程实践上来讲数据库字段最佳实践是：
1. 尽量避免使用null，null在写入和查询时都需要特殊处理，易触发一些你意想不到的逻辑错误 
2. 新增字段必需要包含default
3. select *禁止使用，因为新增字段就可能触发逻辑错误
4. 尽量避免使用auto increment，这会让后续迁移到分布式系统时面临巨大挑战




