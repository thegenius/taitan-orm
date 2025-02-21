use taitan_orm_parser::DatabaseType;

pub struct ExpectSql {
    sql_list: &'static [&'static str]
}
impl ExpectSql {
    pub fn new(sql_list: &'static[&'static str])->Self {
        Self {
            sql_list
        }
    }
    pub fn get(&self, index: usize) -> &'static str {
        self.sql_list[index]
    }

    pub fn expect(&self,  sql: &str, index: usize) {
        if sql != self.get(index) {
            panic!("{}", format!("sql mismatch: \nactual: {:?}, \n\nexpect: {:?}", sql, self.get(index)));
        }
    }
}