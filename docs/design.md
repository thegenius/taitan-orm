
对于MySql只能区分数据的新旧是否一致，不能区分是否是插入
对于Postgres只能区分是否是插入，不能区分新旧值是否一致


| database | true          | false                    |
|----------|---------------|--------------------------|
| mysql    | insert/update | update but value is same |
| postgres | insert        | update                   |


```sql
INSERT INTO table_name (id, col1, col2)
VALUES (1, 'val1', 'val2')
ON DUPLICATE KEY UPDATE col1 = VALUES(col1), col2 = VALUES(col2);

SELECT ROW_COUNT();
-- 如果插入了一条新行，ROW_COUNT() 将返回 1。
```

```sql
INSERT INTO table_name (id, col1, col2)
VALUES (1, 'new_val1', 'new_val2')
ON DUPLICATE KEY UPDATE col1 = VALUES(col1), col2 = VALUES(col2);

SELECT ROW_COUNT();
-- 如果更新了一行，ROW_COUNT() 将返回 1。
```

```sql
INSERT INTO table_name (id, col1, col2)
VALUES (1, 'existing_val1', 'existing_val2')
ON DUPLICATE KEY UPDATE col1 = VALUES(col1), col2 = VALUES(col2);

SELECT ROW_COUNT();
-- 如果更新操作没有实际改变任何数据，ROW_COUNT() 将返回 0。
```
