use std::borrow::Cow;
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::{TableDef, FieldMapper, DatabaseType};

#[test]
fn set_mapper_spec() {
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
    let mapper = FieldMapper::default();

    let sets = mapper.gen_sets(&table_def.fields, &DatabaseType::MySql).to_string();
    assert_eq!(sets, "{ let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a=?,b=?,c=?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d=?\") ; } if self . e . is_some () { s . push_str (\",e=?\") ; } s . push_str (\",f=?,g=?\") ; if self . h . is_some () { s . push_str (\",h=?\") ; } s . push_str (\",i=?\") ; ; s }");

    let sets = mapper
        .gen_sets(&table_def.fields, &DatabaseType::MySql)
        .to_string();
    assert_eq!(sets, "{ let mut s = String :: default () ; let mut has_prev = false ; let mut index = 1 ; s . push_str (\"a=$1,b=$2,c=$3\") ; has_prev = true ; index = index + 3usize ; if self . d . is_some () { s . push_str (format ! (\",d=${}\" , index) . as_ref ()) ; index = index + 1 ; } if self . e . is_some () { s . push_str (format ! (\",e=${}\" , index) . as_ref ()) ; index = index + 1 ; } s . push_str (format ! (\",f=${}\" , index) . as_ref ()) ; index = index + 1 ; s . push_str (format ! (\",g=${}\" , index) . as_ref ()) ; index = index + 1 ; if self . h . is_some () { s . push_str (format ! (\",h=${}\" , index) . as_ref ()) ; index = index + 1 ; } s . push_str (format ! (\",i=${}\" , index) . as_ref ()) ; index = index + 1 ; ; s }");
}

#[test]
fn set_mapper_spec_2() {
    let input: DeriveInput = parse_quote! {
        #[primary(a, b)]
        struct Foo2<'a, 'b> {
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>,
            f: String,
            g: String,
            h: Optional<Cow<'b, str>>,
            i: String
        }
    };

    let table_def = TableDef::parse(&input);
    let sets_mapper = FieldMapper::default();

    let sets = sets_mapper.gen_sets(&table_def.fields, &DatabaseType::MySql).to_string();
    assert_eq!(sets, "{ let mut s = String :: default () ; let mut has_prev = false ; if self . d . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"d=?\") ; } if self . e . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"e=?\") ; } if has_prev { s . push (',') } else { has_prev = true ; } s . push_str (\"f=?,g=?\") ; if self . h . is_some () { s . push_str (\",h=?\") ; } s . push_str (\",i=?\") ; ; s }");
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
            s.push_str("a=?,b=?,c=?");
            has_prev = true;
            if self.d.is_some() {
                s.push_str(",d=?");
            }
            if self.e.is_some() {
                s.push_str(",e=?");
            }
            s.push_str(",f=?,g=?");
            if self.h.is_some() {
                s.push_str(",h=?");
            }
            s.push_str(",i=?");
            s
        };

        let s = {
            let mut s = String::default();
            let mut has_prev = false;
            if self.d.is_some() {
                if has_prev {
                    s.push(',');
                } else {
                    has_prev = true;
                }
                s.push_str("d=?");
            }
            if self.e.is_some() {
                if has_prev {
                    s.push(',');
                } else {
                    has_prev = true;
                }
                s.push_str("e=?");
            }
            if has_prev {
                s.push(',')
            } else {
                has_prev = true;
            }
            s.push_str("f=?,g=?");
            if self.h.is_some() {
                s.push_str(",h=?");
            }
            s.push_str(",i=?");
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
                index = index + 1;
            }
            if self.e.is_some() {
                s.push_str(format!(",${}", index).as_ref());
                index = index + 1;
            }
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            if self.h.is_some() {
                s.push_str(format!(",${}", index).as_ref());
                index = index + 1;
            }
            s.push_str(format!(",${}", index).as_ref());
            index = index + 1;
            s
        };
    }
}
