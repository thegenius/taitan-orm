use std::borrow::Cow;
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::{DatabaseType, FieldMapper, TableDef};

#[test]
fn condition_mapper_spec() {
    let input: DeriveInput = parse_quote! {
        #[primary(a, b)]
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>,
            f: String,
            g: String,
            h: Optional<Cow<'b, str>>,
            i: String
        }
    };

    let table_def = TableDef::parse(&input);
    let conditions_mapper = FieldMapper::default();


    let sets = conditions_mapper
        .gen_conditions(&table_def.fields, &DatabaseType::MySql)
        .to_string();
    assert_eq!(sets, "{ let mut s = String :: default () ; let mut has_prev = false ; s . push_str (format ! (\"a{}?\" , self . a . get_cmp_sql ()) . as_ref ()) ; s . push_str (format ! (\",b{}?\" , self . b . get_cmp_sql ()) . as_ref ()) ; s . push_str (format ! (\",c{}?\" , self . c . get_cmp_sql ()) . as_ref ()) ; has_prev = true ; if self . d . is_some () { s . push_str (format ! (\",d{}?\" , self . d . get_cmp_sql ()) . as_ref ()) ; } if self . e . is_some () { s . push_str (format ! (\",e{}?\" , self . e . get_cmp_sql ()) . as_ref ()) ; } s . push_str (format ! (\",f{}?\" , self . f . get_cmp_sql ()) . as_ref ()) ; s . push_str (format ! (\",g{}?\" , self . g . get_cmp_sql ()) . as_ref ()) ; if self . h . is_some () { s . push_str (format ! (\",h{}?\" , self . h . get_cmp_sql ()) . as_ref ()) ; } s . push_str (format ! (\",i{}?\" , self . i . get_cmp_sql ()) . as_ref ()) ; ; s }");

    let sets = conditions_mapper
        .gen_conditions_indexed(&table_def.fields, &DatabaseType::MySql)
        .to_string();
    assert_eq!(sets, "{ let mut s = String :: default () ; let mut has_prev = false ; let mut index = 1 ; s . push_str (format ! (\"a{}${}\" , self . a . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; s . push_str (format ! (\",b{}${}\" , self . b . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; s . push_str (format ! (\",c{}${}\" , self . c . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; has_prev = true ; if self . d . is_some () { s . push_str (format ! (\",d{}${}\" , self . d . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; } if self . e . is_some () { s . push_str (format ! (\",e{}${}\" , self . e . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; } s . push_str (format ! (\"f{}${}\" , self . f . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; s . push_str (format ! (\",g{}${}\" , self . g . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; if self . h . is_some () { s . push_str (format ! (\",h{}${}\" , self . h . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; } s . push_str (format ! (\"i{}${}\" , self . i . get_cmp_sql () , index) . as_ref ()) ; index = index + 1 ; ; s }");
}

struct Foo<'a, 'b> {
    a: &'a str,
    b: Cow<'b, str>,
    c: String,
    d: Option<Cow<'b, str>>,
    e: Option<Cow<'b, str>>,
    f: String,
    g: String,
    h: Option<Cow<'b, str>>,
    i: String,
}

impl<'a, 'b> Foo<'a, 'b> {
    fn test(&self) {
        let s = {
            // let mut s = String::default();
            // let mut has_prev = false;
            // s.push_str(format!("a{}?", self.a.get_cmp_sql()).as_ref());
            // s.push_str(format!(",b{}?", self.b.get_cmp_sql()).as_ref());
            // s.push_str(format!(",c{}?", self.c.get_cmp_sql()).as_ref());
            // has_prev = true;
            // if self.d.is_some() {
            //     s.push_str(format!(",d{}?", self.d.get_cmp_sql()).as_ref());
            // }
            // if self.e.is_some() {
            //     s.push_str(format!(",e{}?", self.e.get_cmp_sql()).as_ref());
            // }
            // s.push_str(format!(",f{}?", self.f.get_cmp_sql()).as_ref());
            // s.push_str(format!(",g{}?", self.g.get_cmp_sql()).as_ref());
            // if self.h.is_some() {
            //     s.push_str(format!(",h{}?", self.h.get_cmp_sql()).as_ref());
            // }
            // s.push_str(format!(",i{}?", self.i.get_cmp_sql()).as_ref());
            // s
        };
        let s = {
            // let mut s = String::default();
            // let mut has_prev = false;
            // let mut index = 1;
            // s.push_str(format!("a{}${}", self.a.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // s.push_str(format!(",b{}${}", self.b.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // s.push_str(format!(",c{}${}", self.c.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // has_prev = true;
            // if self.d.is_some() {
            //     s.push_str(format!(",d{}${}", self.d.get_cmp_sql(), index).as_ref());
            //     index = index + 1;
            // }
            // if self.e.is_some() {
            //     s.push_str(format!(",e{}${}", self.e.get_cmp_sql(), index).as_ref());
            //     index = index + 1;
            // }
            // s.push_str(format!("f{}${}", self.f.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // s.push_str(format!(",g{}${}", self.g.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // if self.h.is_some() {
            //     s.push_str(format!(",h{}${}", self.h.get_cmp_sql(), index).as_ref());
            //     index = index + 1;
            // }
            // s.push_str(format!("i{}${}", self.i.get_cmp_sql(), index).as_ref());
            // index = index + 1;
            // s
        };
    }
}
