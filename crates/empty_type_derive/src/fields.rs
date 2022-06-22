use quote::ToTokens;
use syn::punctuated::Punctuated;

use syn::{parse_quote, Field, FieldValue, Fields, Index, Member, PathArguments, Token};

pub fn field_type_is_literally(field: &Field, literally: &'static str) -> bool {
    field.ty.to_token_stream().to_string() == literally
}

pub fn wrap_option_in_fallable(field: &mut Field) {
    let ty = field.ty.clone();
    field.ty = parse_quote!( empty_type::Fallible<#ty> );
}

pub fn wrap_field_in_option(field: &mut Field) {
    // we don't do anything to bools. Option<bool> makes little sense to be honest
    if field_type_is_literally(&field, "bool") {
        return;
    }

    // If the field is already an option, we wrap it in a special optional type
    // which is able to unwrap nested options
    if let syn::Type::Path(type_path) = field.ty.clone() {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                let ty = if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    args
                } else {
                    unreachable!()
                };

                field.ty = parse_quote!( empty_type::Optional#ty );
                return;
            }
        }
    }

    let ty = field.ty.clone();
    field.ty = parse_quote! { std::option::Option<#ty>}
}

pub fn create_unwraped_fields(fields: &Fields) -> Punctuated<FieldValue, Token![,]> {
    map_fields_to_values(fields, |field, member| FieldValue {
        attrs: vec![],
        colon_token: field.colon_token.clone(),
        expr: parse_quote! {
            empty_type::Container::try_open_with_meta(&mut self.#member, stringify!(#member))?
        },
        member,
    })
}

pub fn create_unwrapped_default_fields(fields: &Fields) -> Punctuated<FieldValue, Token![,]> {
    map_fields_to_values(fields, |field, member| FieldValue {
        attrs: vec![],
        colon_token: field.colon_token.clone(),
        expr: parse_quote! {
            empty_type::Container::open_or_default(&mut self.#member)
        },
        member,
    })
}

pub fn map_fields_to_values(
    fields: &Fields,
    func: impl Fn(&Field, Member) -> FieldValue,
) -> Punctuated<FieldValue, Token![,]> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.clone().unwrap();
                let member = Member::Named(ident);
                func(field, member)
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let index = Index::from(index);
                let member = Member::Unnamed(index);

                func(field, member)
            })
            .collect(),
        Fields::Unit => Default::default(),
    }
}
