#[derive(Debug)]
pub struct TemplateParseError(pub String);
impl std::error::Error for TemplateParseError {}
impl std::fmt::Display for TemplateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "template parse error: {}", self.0)
    }
}
impl<T> From<T> for TemplateParseError
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        TemplateParseError(s.into())
    }
}