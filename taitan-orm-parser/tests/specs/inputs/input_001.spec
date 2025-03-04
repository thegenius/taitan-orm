# [table(user)]
# [primary(a, b, c)]
# [serde_struct(entity, location)]
# [unique(name = uk1, fields = (f1, f2))]
# [unique(name = uk2, fields = (f3, f4))]
# [index(name = idx_1, fields = (f1, f2, f3))]
# [index(name = idx_2, fields = (f1, f2, f3))]
struct Foo < 'a, 'b > {
     a: & 'a str,
     b: Cow <'b, str >,
     c: String,
     d: Option < Cow < 'b, str > >,

     # [field(name = user_name, db_type = BIGINT, nullable = true, auto_inc = true)]
     e: Option < Cow < 'b, str > >
}