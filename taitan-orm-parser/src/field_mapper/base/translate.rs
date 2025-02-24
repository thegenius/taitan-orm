use crate::field_mapper::base::single_field_mapper::FieldSeg;
use crate::field_mapper::base::LeadingCommaType;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn translate(
    field_seg: &FieldSeg,
    leading_comma_type: LeadingCommaType,
    field_ident: Option<Ident>,
) -> TokenStream {
    let origin = match field_seg {
        FieldSeg::Seg(seg) => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    #seg.to_string()
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!(",{}", seg);
                quote! {
                    #comma_seg.to_string()
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!(",{}", seg);
                quote! {
                    if has_prev {
                        #comma_seg.to_string()
                    } else {
                        has_next = true;
                        #seg.to_string()
                    }
                }
            }
        },
        FieldSeg::IndexedSeg(seg) => match leading_comma_type {
            LeadingCommaType::NoLeading => {
                quote! {
                    {
                        index += 1;
                        format!(#seg, index)
                    }
                }
            }
            LeadingCommaType::Leading => {
                let comma_seg = format!(",{}", seg);
                quote! {
                    {
                        index += 1;
                        format!(#comma_seg, index)
                    }
                }
            }
            LeadingCommaType::CheckedLeading => {
                let comma_seg = format!(",{}", seg);
                quote! {
                    {
                        if has_prev {
                            index += 1;
                            format!(#comma_seg, index)
                        } else {
                            has_next = true;
                            index += 1;
                            format!(#seg, index)
                        }
                    }
                }
            }
        },
    };

    if let Some(field_ident) = field_ident {
        quote! {
            if !self.#field_ident.is_none() {
                #origin
            } else {
                "".to_string()
            }
        }
    } else {
        origin
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use quote::format_ident;
    use crate::field_mapper::base::LeadingCommaType;
    use crate::field_mapper::base::single_field_mapper::FieldSeg;
    use super::translate;

    #[test]
    fn test_translate() {
        // there is 12-Cases
        // TODO: check all 12 cases
        let field_seg = FieldSeg::Seg(Cow::Borrowed("hello"));
        let stream = translate(&field_seg, LeadingCommaType::Leading, None).to_string();
        assert_eq!(stream, r#"",hello" . to_string ()"#);

        let field_seg = FieldSeg::IndexedSeg(Cow::Borrowed("hello=${}"));
        let stream = translate(&field_seg, LeadingCommaType::Leading, None).to_string();
        assert_eq!(stream, r#"{ index += 1 ; format ! (",hello=${}" , index) }"#);

        let field_seg = FieldSeg::IndexedSeg(Cow::Borrowed("name=${}"));
        let field_ident = format_ident!("{}", "name");
        let stream = translate(&field_seg, LeadingCommaType::Leading, Some(field_ident.clone())).to_string();
        assert_eq!(stream, r#"if ! self . name . is_none () { { index += 1 ; format ! (",name=${}" , index) } } else { "" . to_string () }"#);

        let stream = translate(&field_seg, LeadingCommaType::CheckedLeading, Some(field_ident)).to_string();
        assert_eq!(stream, r#"if ! self . name . is_none () { { if has_prev { index += 1 ; format ! (",name=${}" , index) } else { has_next = true ; index += 1 ; format ! ("name=${}" , index) } } } else { "" . to_string () }"#);
    }
}