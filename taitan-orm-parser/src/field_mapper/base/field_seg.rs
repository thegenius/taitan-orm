use crate::field_mapper::base::single_field_mapper::ConnectOp;
use crate::field_mapper::base::LeadingCommaType;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;

pub enum FieldValSeg<'a> {
    // name, name=VALUES(name), ?, name=?
    Seg {
        val: Cow<'a, str>,
        ident: Option<Ident>,
    },
    // ${}, name=${},
    IndexedSeg {
        val: Cow<'a, str>,
        ident: Option<Ident>,
    },
}
impl<'a> FieldValSeg<'a> {
    pub fn is_indexed(&self) -> bool {
        matches!(self, Self::IndexedSeg { .. })
    }
    pub fn get_ident(&self) -> Option<&Ident> {
        match self {
            FieldValSeg::Seg { ident, .. } => ident.as_ref(),
            FieldValSeg::IndexedSeg { ident, .. } => ident.as_ref(),
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            FieldValSeg::Seg { val, .. } => val,
            FieldValSeg::IndexedSeg { val, .. } => val,
        }
    }
}

pub enum FieldExprSeg<'a> {
    // name{}?
    Seg { val: Cow<'a, str>, ident: Ident },
    // name{}${}
    IndexedSeg { val: Cow<'a, str>, ident: Ident },
}

impl<'a> FieldExprSeg<'a> {
    pub fn is_indexed(&self) -> bool {
        matches!(self, Self::IndexedSeg { .. })
    }
    pub fn get_ident(&self) -> &Ident {
        match self {
            FieldExprSeg::Seg { ident, .. } => ident,
            FieldExprSeg::IndexedSeg { ident, .. } => ident,
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            FieldExprSeg::Seg { val, .. } => val,
            FieldExprSeg::IndexedSeg { val, .. } => val,
        }
    }
}

pub enum FieldSeg<'a> {
    Val(FieldValSeg<'a>),
    Expr(FieldExprSeg<'a>),
}

impl<'a> FieldSeg<'a> {
    pub fn from<T: Into<Cow<'a, str>>>(
        seg: T,
        ident: Option<Ident>,
        indexed: bool,
        is_expr: bool,
    ) -> Self {
        let seg = seg.into();
        if is_expr {
            assert_eq!(ident.is_some(), true);
            if indexed {
                FieldSeg::Expr(FieldExprSeg::IndexedSeg {
                    val: seg,
                    ident: ident.unwrap(),
                })
            } else {
                FieldSeg::Expr(FieldExprSeg::Seg {
                    val: seg,
                    ident: ident.unwrap(),
                })
            }
        } else {
            if indexed {
                FieldSeg::Val(FieldValSeg::IndexedSeg { val: seg, ident })
            } else {
                FieldSeg::Val(FieldValSeg::Seg { val: seg, ident })
            }
        }
    }

    pub fn from_seg<T: Into<Cow<'a, str>>>(seg: T, indexed: bool) -> Self {
        Self::from(seg, None, indexed, false)
    }

    pub fn is_expr(&self) -> bool {
        matches!(self, FieldSeg::Expr(_))
    }
    pub fn is_indexed(&self) -> bool {
        match self {
            FieldSeg::Val(val) => val.is_indexed(),
            FieldSeg::Expr(expr) => expr.is_indexed(),
        }
    }
    pub fn get_ident(&self) -> Option<&Ident> {
        match self {
            FieldSeg::Val(val) => val.get_ident(),
            FieldSeg::Expr(expr) => Some(expr.get_ident()),
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            FieldSeg::Val(val) => val.get_value(),
            FieldSeg::Expr(expr) => expr.get_value(),
        }
    }
    pub fn translate(
        &self,
        leading_comma_type: LeadingCommaType,
        connect_op: ConnectOp,
        is_option: bool,
        is_enum: bool,
    ) -> TokenStream {
        match self {
            FieldSeg::Val(seg) => translate_val_seg(seg, leading_comma_type, is_option),
            FieldSeg::Expr(expr) => {
                translate_expr_seg(expr, leading_comma_type, connect_op, is_option, is_enum)
            }
        }
    }
}

fn translate_val_seg(
    field_seg: &FieldValSeg,
    leading_comma_type: LeadingCommaType,
    is_option: bool,
) -> TokenStream {
    let origin = match field_seg {
        FieldValSeg::Seg { val, .. } => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    s.push_str(#val);
                    has_prev = true;
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!(",{}", val);
                quote! {
                    s.push_str(#comma_seg);
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!(",{}", val);
                quote! {
                    if has_prev {
                        s.push_str(#comma_seg);
                    } else {
                        has_prev = true;
                        s.push_str(#val);
                    }
                }
            }
        },
        FieldValSeg::IndexedSeg { val, .. } => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    {
                        index += 1;
                        s.push_str(format!(#val, index).as_ref())
                    }
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!(",{}", val);
                quote! {
                    {
                        index += 1;
                        s.push_str(format!(#comma_seg, index).as_ref())
                    }
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!(",{}", val);
                quote! {
                    {
                        if has_prev {
                            index += 1;
                            s.push_str(format!(#comma_seg, index).as_ref());
                        } else {
                            has_prev = true;
                            index += 1;
                            s.s.push_str(format!(#val, index).as_ref());
                        }
                    }
                }
            }
        },
    };

    let field_ident = field_seg.get_ident();
    if is_option {
        quote! {
            if !self.#field_ident.is_none() {
                #origin
            }
        }
    } else {
        origin
    }
}

fn translate_expr_seg(
    field_seg: &FieldExprSeg,
    leading_comma_type: LeadingCommaType,
    connect_op: ConnectOp,
    is_option: bool,
    is_enum: bool,
) -> TokenStream {
    let field_ident = field_seg.get_ident();
    let cmp_stream = if is_option || is_enum {
        quote! { #field_ident.get_cmp_sql() }
    } else {
        quote! { self.#field_ident.get_cmp_sql() }
    };

    let origin = match field_seg {
        FieldExprSeg::Seg { val, .. } => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    s.push_str(format!(#val, #cmp_stream).as_ref());
                    has_prev = true;
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!("{}{}", connect_op.as_str(), val);
                quote! {
                    s.push_str(format!(#comma_seg, #cmp_stream).as_ref());
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!("{}{}", connect_op.as_str(), val);
                quote! {
                    if has_prev {
                        s.push_str(format!(#comma_seg, #cmp_stream).as_ref());
                    } else {
                        has_prev = true;
                        s.push_str(format!(#val, #cmp_stream).as_ref());
                    }
                }
            }
        },
        FieldExprSeg::IndexedSeg { val, .. } => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    {
                        index += 1;
                        s.push_str(format!(#val, #cmp_stream, index).as_ref());
                    }
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!("{}{}", connect_op.as_str(), val);
                quote! {
                    {
                        index += 1;
                        s.push_str(format!(#comma_seg, #cmp_stream, index).as_ref());
                    }
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!("{}{}", connect_op.as_str(), val);
                quote! {
                    {
                        if has_prev {
                            index += 1;
                            s.push_str(format!(#comma_seg, #cmp_stream, index).as_ref());
                        } else {
                            has_prev = true;
                            index += 1;
                            s.push_str(format!(#val, #cmp_stream, index).as_ref());
                        }
                    }
                }
            }
        },
    };

    if is_option {
        quote! {
            if let Some(#field_ident) = &self.#field_ident {
                #origin
            }
        }
    } else {
        origin
    }
}

#[cfg(test)]
mod tests {
    use super::translate_val_seg;

    use crate::field_mapper::base::{FieldValSeg, LeadingCommaType};
    use quote::format_ident;
    use std::borrow::Cow;

    #[test]
    fn test_translate() {
        // there is 12-Cases
        // TODO: check all 12 cases
        let field_ident = format_ident!("{}", "hello");
        let field_seg = FieldValSeg::Seg {
            val: Cow::Borrowed("hello"),
            ident: Some(field_ident.clone()),
        };

        let stream = translate_val_seg(&field_seg, LeadingCommaType::Leading, false).to_string();
        assert_eq!(stream, r#"s . push_str (",hello") ;"#);

        let field_seg = FieldValSeg::IndexedSeg {
            val: Cow::Borrowed("hello=${}"),
            ident: Some(field_ident.clone()),
        };
        let stream = translate_val_seg(&field_seg, LeadingCommaType::Leading, false).to_string();
        assert_eq!(
            stream,
            r#"{ index += 1 ; s . push_str (format ! (",hello=${}" , index) . as_ref ()) }"#
        );

        let field_ident = format_ident!("{}", "name");
        let field_seg = FieldValSeg::IndexedSeg {
            val: Cow::Borrowed("name=${}"),
            ident: Some(field_ident.clone()),
        };
        let stream = translate_val_seg(&field_seg, LeadingCommaType::Leading, true).to_string();
        assert_eq!(
            stream,
            r#"if ! self . name . is_none () { { index += 1 ; s . push_str (format ! (",name=${}" , index) . as_ref ()) } }"#
        );

        let stream =
            translate_val_seg(&field_seg, LeadingCommaType::CheckedLeading, true).to_string();
        assert_eq!(
            stream,
            r#"if ! self . name . is_none () { { if has_prev { index += 1 ; s . push_str (format ! (",name=${}" , index) . as_ref ()) ; } else { has_prev = true ; index += 1 ; s . s . push_str (format ! ("name=${}" , index) . as_ref ()) ; } } }"#
        );
    }
}
