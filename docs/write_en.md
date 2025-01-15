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
cognitive model of insert:
```text
insert( entity{ field1, field2, field3, field4 } ) -> ()
                  |       |       |       |--> Optional::None ----[ IGNORE]----> colum4
                  |       |       |--> Optional::Null ------------[ NULL  ]----> colum3(set)
                  |       |--> Optional::Some(val) ---------------[ value ]----> colum2(set)
                  |--> Not Optional: val -------------------------[ value ]----> colum1(key not exists)
```
upsert has the similar cognitive model as insert, but when primary key/unique key conflict, execute update.
```text
upsert( entity{ field1, field2, field3, field4 } ) -> ()
                  |       |       |       |--> Optional::None ----[ IGNORE]----> colum4
                  |       |       |--> Optional::Null ------------[ NULL  ]----> colum3(update)
                  |       |--> Optional::Some(val) ---------------[ value ]----> colum2(update)
                  |--> Not Optional: val -------------------------[ value ]----> colum1(key already exists)
```

create has the similar cognitive model as insert, but when there is generated colum, it will fetch from database.
```text
create( mut entity{ field1, field2, field3, field4 } ) -> ()
                      |       |       |       |<-- Optional::None <----[ FETCH ]----- colum4(generated)
                      |       |       |--> Optional::Null -------------[ NULL  ]----> colum3
                      |       |--> Optional::Some(val) ----------------[ value ]----> colum2
                      |--> Not Optional: val --------------------------[ value ]----> colum1(key not exists)
```

```rust
async fn insert(&self, entity: &dyn Entity) -> Result<()>
```
The insert operation is the most basic write operation, used to perform an INSERT into a table.   
Its mental model is straightforward: simply insert a new record into the table; if a conflict occurs, the insertion fails.

However, because table columns may have DEFAULT and NOT NULL constraints,   
and could also be AUTO INCREMENT, the field types in the entity might be optional (Optional).   
This adds complexity to the insert operation.

|                  | not optional         | optional::None   | optional::Null | optional::Some  |
|------------------|----------------------|------------------|----------------|-----------------|
| -                | ✅                    | null             | null           | ✅               |
| default          | ✅                    | default          | null           | ✅               |
| not null         | ✅                    | run time error   | run time error | ✅               |
| not null default | ✅                    | default          | run time error | ✅               |
| auto increment   | ❌ compile time error | ✅                | run time error |  run time error |
| generated        | ❌ compile time error | ✅                | run time error |  run time error |

Best Practices for Database Fields (From an Engineering Perspective)

From an engineering perspective, best practices for database fields include the following:

#### Avoid Using NULL as Much as Possible:
* NULL requires special handling during writes and queries and can easily trigger unexpected logical errors.
* NULL is prone to causing issues during JSON serialization and deserialization, as well as during database read and write operations.
#### Newly Added Fields Must Include a Default Value (DEFAULT):
* When adding new fields, always specify a default value to ensure that inserts do not fail due to missing values and to reduce potential errors.
#### Prohibit the Use of SELECT *:
* Avoid using SELECT *, as adding new fields can lead to logical errors. Explicitly listing the required columns enhances code maintainability and stability.
#### Avoid Using Auto Increment (AUTO INCREMENT) as Much as Possible:
* Minimize the use of auto-increment fields, as this can present significant challenges when migrating to distributed systems. Managing globally unique auto-increment IDs in a distributed environment is complex and can lead to conflicts or other issues.




