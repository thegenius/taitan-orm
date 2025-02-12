use std::borrow::Cow;
use std::env;
use std::io::BufRead;
use bigdecimal::BigDecimal;
use tracing::field::Field;
use uuid::Uuid;
use taitan_orm_macro::SchemaNew;
use taitan_orm_trait::Optional;
use serde_json::Value;

#[derive(SchemaNew)]
#[debug = "./table_spec.test.def"]
struct TableSpec <'a>{
    pub field1: f64,
    pub field2: Option<i32>,
    pub field3: Optional<u64>,

    pub field: BigDecimal,
    pub field4: bigdecimal::BigDecimal,
    pub field5: Option<BigDecimal>,
    pub field6: Optional<BigDecimal>,

    pub field7: Uuid,
    pub field8: uuid::Uuid,
    pub field9: Option<Uuid>,
    pub field10: Optional<uuid::Uuid>,

    pub field11: String,
    pub field12: &'a str,
    pub field13: Cow<'a, str>,
    pub field14: Optional<String>,
    pub field15: Optional<&'a str>,
    pub field16: Optional<Cow<'a, str>>,
}

fn read_file_with_buffer(file_path: &str, buffer_size: usize) -> std::io::Result<String> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::with_capacity(buffer_size, file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n'); // 保留换行符
    }

    Ok(content)
}

#[test]
fn test_table_def_extractor() {
    match env::current_dir() {
        Ok(current_dir) => println!("当前工作目录: {:?}", current_dir),
        Err(e) => eprintln!("无法获取当前工作目录: {}", e),
    }
    let def_content= read_file_with_buffer("../table_spec.test.def", 4096).unwrap();
    let def_value: Value = serde_json::from_str(&def_content).unwrap();

    let expect_value: Value = serde_json::from_str(r#"
    {
  "table_name": "TableSpec",
  "serde_structs": [],
  "columns": [
    {
      "struct_field": {
        "name": "field1",
        "rust_type": "f64",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field2",
        "rust_type": "i32",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field3",
        "rust_type": "u64",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field",
        "rust_type": "BigDecimal",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field4",
        "rust_type": "bigdecimal::BigDecimal",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field5",
        "rust_type": "BigDecimal",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field6",
        "rust_type": "BigDecimal",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field7",
        "rust_type": "Uuid",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field8",
        "rust_type": "uuid::Uuid",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field9",
        "rust_type": "Uuid",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field10",
        "rust_type": "uuid::Uuid",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field11",
        "rust_type": "String",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field12",
        "rust_type": "&'a str",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field13",
        "rust_type": "Cow<'a, str>",
        "default_value": null,
        "is_optional": false
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field14",
        "rust_type": "String",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field15",
        "rust_type": "&'a str",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    },
    {
      "struct_field": {
        "name": "field16",
        "rust_type": "Cow<'a, str>",
        "default_value": null,
        "is_optional": true
      },
      "table_column": {
        "name": null,
        "column_type": null,
        "default_value": null,
        "is_nullable": false,
        "is_generated": false,
        "is_auto_inc": false,
        "is_primary_key_part": false
      }
    }
  ],
  "primary_key_fields": [],
  "non_primary_key_fields": [],
  "unique_keys": {},
  "index_key": {}
}"#
    ).unwrap();

    // assert_eq!(def_value, expect_value);
}