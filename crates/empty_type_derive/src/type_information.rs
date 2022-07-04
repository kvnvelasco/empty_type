/*
 * Copyright [2022] [Kevin Velasco]
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::attribute::filter_attrs_by_own;
use crate::fields::{create_unwraped_fields, create_unwrapped_default_fields};
use crate::{find_path_of_attribute, ContainerFlags};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Data, DeriveInput, FieldValue, Fields, GenericParam, Generics, Token, TypeParam,
    WhereClause,
};

pub struct TypeInformation {
    pub wrapped_struct_name: Ident,
    pub prefix_generics: Generics,
    pub postfix_generics: Generics,
    pub where_clause: Option<WhereClause>,
    pub fields: Fields,
    pub container_attributes: Vec<Attribute>,
}

impl TypeInformation {
    pub fn new(input: DeriveInput) -> Self {
        // Prefix generics contain type definitions and type bounds but no where clauses.
        // Found in impl blocks like `impl<'a, T: 'a>
        let prefix_generics = {
            let mut generics = input.generics.clone();
            generics.where_clause = None;
            generics
        };

        let postfix_generics = Self::create_postfix_generics(input.generics.clone());
        let fields = Self::extract_fields(input.data);

        Self {
            prefix_generics,
            postfix_generics,
            wrapped_struct_name: input.ident,
            fields,
            where_clause: input.generics.where_clause,
            container_attributes: input.attrs,
        }
    }

    #[cfg(feature = "serde")]
    pub fn only_serde_attributes(&self) -> Vec<Attribute> {
        let attrs = self.container_attributes.clone();
        attrs
            .into_iter()
            .filter_map(|attr| {
                if attr.path.is_ident("serde") {
                    Some(attr)
                } else {
                    None
                }
            })
            .collect()
    }

    fn extract_fields(data: Data) -> Fields {
        match data {
            Data::Struct(str) => str.fields,
            Data::Enum(_) => {
                panic!("")
            }
            Data::Union(_) => {
                panic!("")
            }
        }
    }

    pub fn fields_unwrapped(&self) -> Punctuated<FieldValue, Token![,]> {
        create_unwraped_fields(&self.fields)
    }

    pub fn fields_uwnrapped_default(&self) -> Punctuated<FieldValue, Token![,]> {
        create_unwrapped_default_fields(&self.fields)
    }

    pub(crate) fn fields_wrapped_in_options(
        &self,
        container_attributes: &ContainerFlags,
    ) -> Fields {
        let mut fields = self.fields.clone();
        match &mut fields {
            Fields::Named(named_field) => named_field.named.iter_mut().for_each(|f| {
                crate::fields::wrap_field_in_option(f);

                if container_attributes.fail_safe
                    || find_path_of_attribute(&f.attrs, "fail_safe").is_some()
                {
                    crate::fields::wrap_option_in_fallable(f);
                }

                // filter the field's attributes.
                f.attrs = std::mem::take(&mut f.attrs)
                    .into_iter()
                    .filter(|f| !filter_attrs_by_own(f))
                    .collect();
                if !container_attributes.deserialize {
                    f.attrs.clear()
                }
            }),
            Fields::Unnamed(unnamed_field) => unnamed_field.unnamed.iter_mut().for_each(|f| {
                crate::fields::wrap_field_in_option(f);

                if container_attributes.fail_safe
                    || find_path_of_attribute(&f.attrs, "fail_safe").is_some()
                {
                    crate::fields::wrap_option_in_fallable(f);
                }

                // filter the field's attributes.
                f.attrs = std::mem::take(&mut f.attrs)
                    .into_iter()
                    .filter(|f| !filter_attrs_by_own(f))
                    .collect();

                if !container_attributes.deserialize {
                    f.attrs.clear()
                }
            }),
            Fields::Unit => {}
        }

        fields
    }

    pub fn is_tuple_struct(&self) -> bool {
        matches!(&self.fields, &Fields::Unnamed(_))
    }

    pub fn fully_qualified_wrapped_struct_name(&self) -> TokenStream {
        let ref postfix_generics = self.postfix_generics;
        let name = &self.wrapped_struct_name;
        quote! { #name#postfix_generics }
    }

    pub fn derived_struct_name(&self) -> Ident {
        format_ident!("___Empty{}", self.wrapped_struct_name)
    }

    pub fn fully_qualified_derived_struct_name(&self) -> TokenStream {
        let ref postfix_generics = self.postfix_generics;
        let name = self.derived_struct_name();
        quote! { #name#postfix_generics }
    }

    fn create_postfix_generics(mut generics: Generics) -> Generics {
        generics.params.iter_mut().for_each(|field| match field {
            GenericParam::Type(ty) => {
                ty.colon_token = None;
                ty.bounds.clear();
                ty.eq_token = None;
                ty.default = None;
            }
            GenericParam::Lifetime(li) => {
                li.colon_token = None;
                li.bounds.clear();
            }
            GenericParam::Const(c) => {
                let mut sentinel = TypeParam {
                    attrs: vec![],
                    ident: Ident::new("", Span::call_site()),
                    colon_token: None,
                    bounds: Default::default(),
                    eq_token: None,
                    default: None,
                };
                std::mem::swap(&mut sentinel.ident, &mut c.ident);
                std::mem::swap(&mut sentinel.attrs, &mut c.attrs);
                *field = GenericParam::Type(sentinel)
            }
        });

        generics
    }
}
