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

    pub fn expect(&self, sql: &str, index: usize) {
        assert_eq!(self.get(index), sql);
    }
}