
#[derive(Debug, Clone, PartialEq)]
pub enum VariableName {
    Simple(String),




    Hash(String),
    At(String),
}

pub trait ToSqlSegment {

    // ${ name } 在to_sql的时候替换为 {{ name }}
    // #{ name } 替换为 ?，并且variables里面返回名称，保证后续的绑定

    fn to_sql(&self) -> String;

    // #{ name } 替换为 ?，并且variables里面返回名称，保证后续的绑定
    // @{ name } 替换为 if self.name.is_some() { v.push("name".to_string()); }
    fn get_variables(&self) -> Vec<VariableName> {
        Vec::new()
    }
}
