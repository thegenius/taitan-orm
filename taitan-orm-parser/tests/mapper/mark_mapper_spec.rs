use std::borrow::Cow;
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::MultiFieldMapper;
use taitan_orm_parser::MySqlKeywordEscaper;
use taitan_orm_parser::{Connector, MarksMapper};
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
    let marks_mapper = MarksMapper::default();
    let escaper = MySqlKeywordEscaper::default();
    let marks = marks_mapper.map(&table_def.fields, &escaper).to_string();
    assert_eq!(marks, r#""?,?,?,?,?,?,?,?,?""#);

    let marks = marks_mapper
        .connect(&table_def.fields, &escaper)
        .to_string();
    assert_eq!(marks, "{ let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",?\") ; } if self . e . is_some () { s . push_str (\",?\") ; } s . push_str (\",?,?\") ; if self . h . is_some () { s . push_str (\",?\") ; } s . push_str (\",?\") ; ; s }");

    let marks = marks_mapper
        .connect_indexed(&table_def.fields, &escaper)
        .to_string();
    assert_eq!(marks, "{ let mut s = String :: default () ; let mut has_prev = false ; let mut index = 1 ; s . push_str (\"$1,$2,$3\") ; has_prev = true ; index = index + 3usize ; if self . d . is_some () { s . push_str (format ! (\",${}\" , index) . as_ref ()) ; } if self . e . is_some () { s . push_str (format ! (\",${}\" , index) . as_ref ()) ; } s . push_str (format ! (\",${}\" , index) . as_ref ()) ; index = index + 1 ; s . push_str (format ! (\",${}\" , index) . as_ref ()) ; index = index + 1 ; if self . h . is_some () { s . push_str (format ! (\",${}\" , index) . as_ref ()) ; } s . push_str (format ! (\",${}\" , index) . as_ref ()) ; index = index + 1 ; ; s }");
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
            s.push_str("?,?,?");
            has_prev = true;
            if self.d.is_some() {
                s.push_str(",?");
            }
            if self.e.is_some() {
                s.push_str(",?");
            }
            s.push_str(",?,?");
            if self.h.is_some() {
                s.push_str(",?");
            }
            s.push_str(",?");
            s
        };
        let s = {
            let mut s = String::default();
            let mut has_prev = false;
            let mut index = 1;
            s.push_str("$1,$2,$3");
            has_prev = true;
            index = index + 3usize;
            if self.d.is_some() {
                s.push_str(format!(",${}", index).as_ref());
            }
            if self.e.is_some() {
                s.push_str(format!(",${}", index).as_ref());
            }
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            if self.h.is_some() {
                s.push_str(format!(",${}", index).as_ref());
            }
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            s
        };
    }
}
