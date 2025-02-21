#[table(user)]
#[primary(a, b, c)]
struct Foo<'a, 'b> {
      name: &'a str,
      select: Cow<'b, str>,
      and: String,
      age: Option<Cow<'b, str>>,
      #[field(name = user_name, db_type = BIGINT, nullable = true, auto_inc = true)]
      primary: Optional<Cow<'b, str>>
}