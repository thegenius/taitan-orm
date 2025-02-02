## Overall Error Design
```
 ____________________________________________________
| parse  | connect | encode | sql | execute | decode |
 ----------------------------------------------------
   |          |        |       |       |         |--> row not found
   |          |        |       |       |         |--> column not found, name or index
   |          |        |       |       |         |--> type not match
   |          |        |       |       |--> write constraint violation
   |          |        |       |       |--> execute time out
   |          |        |       |--> can not gen sql, e.g., mutation with all none fields
   |          |        |       |--> syntax error, database return this error after parse sql
   |          |        |--> type not supported
   |          |        |--> sqlx args add error
   |          |--> temporary connect fail, e.g., can not get conn from pool
   |          |--> permanent connect fail, e.g., config error or tls error
   |--> comperator parse error
   |--> location expr parse error
   |--> OrderBy struct construct error
```




