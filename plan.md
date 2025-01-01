
# API设计，把API设计逐渐固定下来
1. transaction支持能够更加优雅

# 多数据库支持
1. 参数生成的trait可能需要按照数据库拆分开来
2. generator
3. executor
4. api
分层的设计逻辑需要重新review

sqlite: YES
mysql: No
postgres: No

# 文档

# 正确性测试

# 性能测试


# 额外优化
是否支持Cow<'a, str>作为entity的字符串字段，目前因为查询返回值一定不能用&str
