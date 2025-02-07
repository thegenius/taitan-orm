

```
 ____________________________________________________
| info extractor | struct generator | impl generator |
 ----------------------------------------------------
        |               |                |--> generate impl occording to fields
        |               |--> generate struct of fields
        |--> extract info from struct and attributes
```

## (1) Info Extractor
信息抽取器的职责是从Schema和struct定义中抽取2个重要信息
```
FieldsNamed -> Vec<FieldDef>
```
```
Attributes -> TableDef
```

## (2) Struct Generator

## (3) Impl Generator

```
template sql parser
```
1. parser需要挪进来
2. 整体的实现结构分层还不清晰，需要重新架构