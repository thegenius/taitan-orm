Why need another framework? This is a question every new framework must answer. The most common answer is that the author believes existing frameworks are not good enough, but this "not good enough" is often subjective.

If someone creates a new framework based on subjective bias, it is usually foolish. And these foolish and crude frameworks will be buried by the tides of time.

An effective way to address this is to put the views on the table, subject peer review, and let the truth become clearer.

### [1] SQL First, Not Object First (or Struct First)
As a Rustacean, I am also a Java engineer. In the Java ecosystem, there are two common ORM systems: JPA and MyBatis. JPA is the official standard, while MyBatis is almost the de facto standard in China. In interviews for Chinese engineers, a potential question is: Why should we choose MyBatis instead of JPA? Choosing MyBatis over JPA seems hard to understand for many international programmers because, at first glance, MyBatis appears to be inferior to JPA in every way.
```text
1. JPA is officially promoted, and investing significant effort in iteration and optimization each year.
2. JPA has a rich ecosystem, being the most powerful ORM framework in the Java ecosystems.
3. JPA has excellent cross-database capabilities, generating different SQL for various databases.
```
MyBatis's strongest feature is its templating ability, which supports handwritten SQL. However, JPA already supports handwritten SQL through the @NativeQuery annotation, so even this standout feature of MyBatis is covered by JPA. Despite this, a large number of architects and engineering experts in China have chosen the MyBatis family of frameworks, such as MyBatis-Plus. If you’re willing to dig deeper into this issue, some essential insights emerge.

#### (1) SQL is more powerful than object-oriented languages when querying
Take a complicate SQL query as an example:
```sql
深色版本
WITH RankedOrders AS (
SELECT
id,
customer_id,
order_date,
amount,
SUM(amount) OVER (PARTITION BY customer_id) AS total_amount,
ROW_NUMBER() OVER (PARTITION BY customer_id ORDER BY order_date DESC) AS rank
FROM orders
)
SELECT
id,
customer_id,
order_date,
amount,
total_amount
FROM RankedOrders
WHERE rank = 1
```


Emerging SQL-like DSLs also surpass objects in their respective domains.   
For example,  
Neo4j’s Cypher language:
```sql
// Find all people (f) that 'Alice' (p) knows in a graph database.
MATCH (p:Person)-[:KNOWS]->(f:Person)
WHERE p.name = 'Alice'
RETURN f.name
Or InfluxDB’s Flux language:
```

InfluxDB's Flux language:
```sql
// Calculate the average value every 5 minutes.
from(bucket: "my_bucket")
|> range(start: -1h)
|> filter(fn: (r) => r._measurement == "sensor_data")
|> aggregateWindow(every: 5m, fn: mean, createEmpty: false)
|> yield(name: "downsampled_data")
```

From these three examples, we can draw a clear conclusion:
```text
1. SQL-centric, not object-centric.  
2. SQL control objects, not the opposite.
```



#### (2) Simulating relationships at the object level is impractical for massive datasets
Below is a common definition of Author and Post entities, where one author can publish multiple posts:

```java
@Entity
public class Author {
@Id
private Long id;

    private String name;

    @OneToMany(mappedBy = "author", fetch = FetchType.LAZY)
    private List<Post> posts;
}

@Entity
public class Post {
@Id
private Long id;

    private String title;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "author_id")
    private Author author;
}
```
This entity definition works well when the number of posts is small. However, when the number of posts becomes large (e.g., 10,000), this structure quickly collapses. There are many optimization strategies, but they are mostly workarounds and cannot fundamentally solve the problem. The only reasonable choice for extremely large datasets is pagination. It is difficult to elegantly express paginated join queries in object-centric structures. In JPA, you often need to write custom queries:
```java
public interface UserRepository extends JpaRepository<User, Long> {
    @Query("SELECT a FROM Author a JOIN FETCH a.posts p WHERE a.id = :id")
    Page<User> findUserWithPost(@Param("id") Long id, Pageable pageable);
}
```
At this point, JPA usage is essentially equivalent to writing SQL manually in MyBatis. Similarly, in Rust’s Sea-ORM:

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
#[sea_orm(primary_key)]
pub id: i32,
pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
#[sea_orm(has_many = "super::fruit::Entity")]
Fruit,
}

impl Related<super::fruit::Entity> for Entity {
fn to() -> RelationDef {
Relation::Fruit.def()
}
}
```
Even emerging ORMs like Toasty follow a similar approach:
```rust

#[derive(Debug)]
#[toasty::model]
struct User {
#[key]
#[auto]
id: Id<Self>,

    name: String,

    #[unique]
    email: String,

    #[has_many]
    todos: [Todo],

    moto: Option<String>,
}

#[derive(Debug)]
#[toasty::model]
struct Todo {
#[key]
#[auto]
id: Id<Self>,

    #[index]
    user_id: Id<User>,

    #[belongs_to(key = user_id, references = id)]
    user: User,

    title: String,
}
```
In terms of simulating relationships, JPA, Sea-ORM, and Toasty all follow the same path, which fails when dealing with massive associated data. Thus, we arrive at the following conclusion:

```text
When data volumes are large, object-based relationship simulations are fundamentally unusable.
```
  
### [2] More Features Are Not Always Better; Fewer Error-Prone Features Are Better
If we objectively compare the features of JPA and MyBatis, JPA is more powerful:
```text
 _______________________________
|        JPA Capacity           |
|    -------------------        |
|   |  MyBatis Capacity |       |
|    -------------------        |
|______________________________ |
```

Many libraries and frameworks pursue additional features while introducing numerous hidden dangers. The biggest danger is misuse. To use JPA effectively, team members must be highly proficient in it to avoid most pitfalls. Architects or lead programmers often need to establish strict coding guidelines to maintain code quality. For example, some teams prohibit the use of JPA’s built-in OneToMany and ManyToMany relationships. However, if these features are banned, JPA often becomes less appealing than MyBatis. Looking at Sea-ORM and Toasty, the situation is similar—they follow JPA’s old path. Many of their convenient features only work well with small datasets, making them prone to misuse. Moreover, Sea-ORM and Toasty offer weak support for handwritten SQL.

### [3] Ultimate Support for SQL Templates Is the Right Path for SQL-Centric Libraries
In JPA, the primary way to write SQL is through placeholders:

```sql
SELECT id, name FROM `user` WHERE id = :id
```
This support is insufficient for many complex scenarios. Imagine a complex query console with multiple optional filters. If a condition is not selected, it should not affect the query. Dynamic query assembly requires more than simple placeholders—it needs dynamic templating capabilities:

```sql

SELECT `id`, `name`, `age`
    FROM `user`
WHERE {% if age.is_some() %} age >= :{age} AND {% endif %} `name` = :{name}
```

### [4] For Extreme Performance and API Simplicity, Macros Should Be Maximized
We know that doing more at compile time leads to better runtime performance. Rust macros are the most powerful compile-time processing mechanism. Maximizing the use of macros may achieve the best possible performance.

**Based on these four key points, I decided to create a brand-new ORM: TaiTan-ORM.**