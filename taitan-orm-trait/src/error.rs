#[derive(Debug)]
pub struct NotImplementError(pub String);
impl std::error::Error for NotImplementError {}
impl std::fmt::Display for NotImplementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "method {} is not implements", self.0)
    }
}

#[derive(Debug)]
pub struct NotValidOrderByError(pub String);
impl std::error::Error for NotValidOrderByError {}
impl std::fmt::Display for NotValidOrderByError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "order by fields: {} is not valid", self.0)
    }
}

#[derive(Debug)]
pub struct NotValidConditionError(pub String);
impl std::error::Error for NotValidConditionError {}
impl std::fmt::Display for NotValidConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "condition: {} is not valid", self.0)
    }
}


