use std::env;
use std::io::BufRead;
use taitan_orm_macro::SchemaNew;

#[derive(SchemaNew)]
#[debug = "./table_spec.test.def"]
struct TableSpec {
    pub name: String,
    pub age: u64
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
    assert_eq!(def_content, "{\"table_name\":\"TableSpec\",\"serde_structs\":[],\"columns\":[],\"primary_key_fields\":[],\"non_primary_key_fields\":[],\"unique_keys\":{},\"index_key\":{}}\n");
}