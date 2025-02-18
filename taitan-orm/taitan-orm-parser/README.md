


# Info parser

struct parser

attribute parser

field parser


# Field Mapper
(1) name: `field`  
(2) mark: `?`  
(3) indexed mark: `$1`  
(4) set statement: `field` = `?`  
(5) set indexed: `field`=`$1`
(6) condition: `field` > `?`
(7) condition indexed: `field` <= `$1`

# Connector：本质上就是为了生成逗号
负责连接 , 的生成
将field list切割为3个区间  
(1) leading required是开头处的required字段  
这部分单独出来是因为可以进行非常多的compile time优化
1.1 这个区间可以视为一个整体，内部用逗号连接，编译期完成  
1.2 整个区间结束，添加has_prev = true  
1.3 整个区间结束，添加index = index + len  

(2) optional区间
1.1 每个字段单独处理  
1.2 如果小于first_required，判断has_prev，添加has_prev=true  
1.3 如果大于first_required，不判断has_prev，不添加has_prev  
1.4 每个字段添加 index = index + 1  

(3) 后续的required
1.1 不需要index的情况下，可以区间一起处理
1.2 需要index的情况下，需要单独处理
1.3 不需要添加has_prev和判断has_prev
1.4 单独处理时，每个字段添加 index = index + 1

```
[ leading required ] { [ optional ] [ required ] } *

```