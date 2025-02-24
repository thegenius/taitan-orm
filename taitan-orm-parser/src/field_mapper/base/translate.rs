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
