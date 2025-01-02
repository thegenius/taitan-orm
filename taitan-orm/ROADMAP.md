
# [1] Executor的设计稳定下来
1. 尽量保证可以写数据库无关的代码
2. 写一个axum的例子来保证可以跑通

# [2] Error的设计需要再进行一遍
1. 逻辑错误：重试无法成功的错误
约束冲突：例如主键冲突，唯一键冲突，check约束冲突
语法错误：不支持的语法
2. 系统错误区：暂时性的系统问题
连接超时：
拿不到连接：

# [3] 简洁化

## [3.1] re-export，保证用户不需要重复引入我们依赖的库
1. sqlx
2. serde
3. time
4. bigdecimal
5. uuid

## [3.2] prelude设计，保证绝大多数时候只需要简单引入即可

# [4] Feature化，让不是必须的组件变为可选

# [4.1] 把依赖的组件设计为feature
bigdecimal
time
uuid
serde
# [4.2] 生成的代码可以依据feature变化


# [5] 文档
1. readme丰富化
2. rust docs能够清晰地查阅api
3. 文章介绍usage和implementation

# [6] 正确性测试
1. 基于sqlite的specification测试，保证所有功能点都有覆盖
2. 基于docker的mysql和postgres测试
3. 基于fuzz的模糊测试

# [7] 性能测试
1. writer/reader单项测试
2. transaction测试
3. template测试
4. 基于database被mock的micro benchmark


# 额外优化
是否支持Cow<'a, str>作为entity的字符串字段，目前因为查询返回值一定不能用&str
