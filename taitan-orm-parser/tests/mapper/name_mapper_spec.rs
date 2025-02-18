use std::borrow::Cow;
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::Connector;
use taitan_orm_parser::MultiFieldMapper;
use taitan_orm_parser::MySqlKeywordEscaper;
use taitan_orm_parser::{NamesMapper, TableDef};

#[test]
fn name_mapper_spec() {
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
    let names_mapper = NamesMapper::default();
    let escaper = MySqlKeywordEscaper::default();
    let names = names_mapper.map(&table_def.fields, &escaper).to_string();
    assert_eq!(names, r#""a,b,c,d,e,f,g,h,i""#);

    let names = names_mapper
        .connect(&table_def.fields, &escaper)
        .to_string();
    assert_eq!(names, "{ let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d\") ; } if self . e . is_some () { s . push_str (\",e\") ; } s . push_str (\",f,g\") ; if self . h . is_some () { s . push_str (\",h\") ; } s . push_str (\",i\") ; ; s }");
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
            let mut s = String::default();
            let mut has_prev = false;
            s.push_str("a,b,c");
            has_prev = true;
            if self.d.is_some() {
                s.push_str(",d");
            }
            if self.e.is_some() {
                s.push_str(",e");
            }
            s.push_str(",f,g");
            if self.h.is_some() {
                s.push_str(",h");
            }
            s.push_str(",i");
            s
        };
    }
}
