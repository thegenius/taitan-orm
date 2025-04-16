为什么需要一个新的ORM框架，这是每个新的框架都需要回答的问题。
最通常的答案是作者认为现有的框架不足够好，但是这种不够好通常是带有主观偏见的。
如果因为一些主观偏见，就花大量精力造一个新的轮子，通常是愚蠢的。而这些愚蠢而简陋的轮子通常会在时代的洪流中慢慢被淹没。
一个非常有效的应对办法是：尽早把观点放到台面上来，接受同行的审阅，真理会在一次又一次的辩论中愈发清晰。

## [1] SQL First而不是Object First(也就是Struct First)
我是一个多年的Java工程师，在Java的生态中，有2套最为常见的ORM系统，JPA和MyBatis。其中JPA是官方的主流标准，
而MyBatis是在中国流行到几乎成为事实标准的ORM框架。
在中国的工程师面试问题中，有一道可能的题目就是为什么应该应该选MyBatis，而不是JPA？
不选择JPA，在很多国际化程序员眼中是离经叛道的，也是难以理解的。因为MyBatis看起来全面弱于JPA。  
(1) JPA是官方推进的，有官方的人每年投入大量精力迭代和优化  
(2) JPA是生态丰富的，JPA是Java生态中拥有最强大生态的ORM框架，不带之一  
(3) JPA具有不错的跨数据库能力，可以针对不同数据库生成不同的代码，屏蔽数据库的差异
而MyBatis最能拿得出手的，就是拥有模板的能力，支持手写SQL。但是JPA已经支持了@NativeQuery注解，也支持了手写SQL，
所以某种意义上，连手写SQL这个最能拿得出手的部分，JPA也支持了。  

然后，中国大量的架构师和工程专家选择了MyBatis系列框架，例如MyBatis-Plus。如果愿意稍微深入探索一下这个问题，一些
本质的见解就浮现出来了。  
(1) SQL本质是比对象更加强大的查询语言，任何企图通过通过对象关系模拟SQL的方式都是相对拙略的
以一个SQL查询为例子，可以得到一个简单明了的结论：SQL在查询表达上的能力是远远强于面向对象的Object或者rust中的struct
```sql
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
新兴的类SQL的DSL语言，在各自的领域中所能够表达的查询能力也是强于Object的
例如Neo4j的Cypher语言
```sql
// 在图数据库中查找名为 'Alice' 的人（p），并通过 KNOWS 关系找到她认识的所有人（f）。
MATCH (p:Person)-[:KNOWS]->(f:Person)
WHERE p.name = 'Alice'
RETURN f.name
```
例如InfluxDB上的Flux语言
```sql
// 每5分钟一次的平均值
from(bucket: "my_bucket")
    |> range(start: -1h)
    |> filter(fn: (r) => r._measurement == "sensor_data")
    |> aggregateWindow(every: 5m, fn: mean, createEmpty: false)
    |> yield(name: "downsampled_data")
```
以上面的3个例子为基点，我们可以得到一个清晰的结论
```
以SQL为中心，而不是以对象为中心
用SQL控制对象，而是用对象控制SQL
```


(2) 尝试在对象层面建立起关系映射在超大数据量时是不可行的设计
下面是一个常见的作者和发帖的实体定义，一个作者可能发布多个帖子
```Java
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
这样的实体定义在帖子数量很少的情况下，工作得非常出色，但是一旦考虑到帖子数量如果可能很多，例如1W，那么这个
结构就会迅速崩坏。优化方案有很多，但是基本是缓解，而不能彻底解决。因为本质上当数据超多的时候，只有分页查询
是合理的选择，在以Object为中心的结构体定义中，很难优雅的表达分页join查询。在JPA中通常需要需要单独写查询
```java
public interface UserRepository extends JpaRepository<User, Long> {
    @Query("SELECT a FROM Author a JOIN FETCH a.posts p WHERE a.id = :id")
    Page<User> findUserWithPost(@Param("id") Long id, Pageable pageable);
}
```
上面的这种JPA使用方案就已经基本等价于MyBatis手写了。
类似于JPA的，rust生态中的Sea-ORM的类似定义如下
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

包括新兴的Toasty，也走入了类似JPA的思路：
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

在关于模拟关系这个思路上，JPA/Sea-ORM/Toasty三者基本都是一脉相承的思路，
这个思路都会在关联数据庞大的时候失灵。 至此，我们基本得到一个观点
```
当数据量巨大的时候，基于对象关联模拟的关联关系，基本是无法直接使用的。
```

## [2] 功能不是越多越好，而是不能犯错的功能越多越好
如果真讨论JPA和MyBatis的功能，客观上讲JPA是更为强大的
```
 _______________________________
|        JPA Capacity           |
|    -------------------        |
|   |  MyBatis Capacity |       |
|    -------------------        |
|______________________________ |

```
很多时候库和框架都在追求更多的特色功能的同时埋下了非常多的隐患，最大的隐患其实就是被滥用。
在使用JPA时，需要保证团队中的人员都非常精通JPA，才能避免掉绝大多数的坑。很多架构师或者主程序员
需要通过制定非常多的代码规范才能保证团队代码质量底线。例如：有的团队就会规定禁止使用JPA自带的
OneToMany, ManyToMany关联关系。但是如果把这些都禁止了，那么很多时候不如用MyBatis了。
再来看Sea-ORM和Toasty，基本情况是一样的，他们都走上了JPA的老路。他们提供的很多便捷性功能
只在数据量小的时候有效，而这部分功能极容易被滥用。甚至Sea-ORM和Toasty还不如JPA，Sea-ORM和Toasty
对手写SQL的支持是非常弱的。

## [3] 对SQL模板的极致支持，是SQL为中心的库应该走的方向
JPA中最主要的手写SQL方式是占位符
```sql
SELECT id,name from `user` WHERE id = :id
```
这样的支持在面对很多复杂场景时是不足够的。
设想，一个复杂的查询控制台，有很多查询条件可供用户选择，不选的时候某个条件不生效。
这种动态拼装查询的场景，用最简单的占位符是不足以支持的，这个时候需要一些动态模板的能力
```sql
select `id`, `name`, `age` FROM `user` where {% if age.is_some() %} age >= :{age} AND {% endif %} `name` = :{name}
```

## [4] 出于对性能和API简洁性的极致追求，应该最大化使用Macro
我们都知道编译期如果能够做更多的事情，那么运行态就能够获得更加极致的性能。
而rust中的宏是最为强大的编译期处理机制，最大化利用宏能够获得最极致的性能。

综上4个主要观点，导致我准确造一个全新的ORM：TaiTan-ORM.





