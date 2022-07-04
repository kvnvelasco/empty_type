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

#![cfg_attr(docs_rs, feature(doc_cfg))]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

use crate::attribute::find_path_of_attribute;
use crate::type_information::TypeInformation;

mod fields;
mod type_information;

#[proc_macro_derive(EmptyType, attributes(empty))]
#[cfg_attr(docs_rs, doc(cfg(feature = "derive")))]
#[doc = include_str!("../README.md")]
pub fn empty_type(input: TokenStream) -> TokenStream {
    create_struct_tokens(input)
}

#[derive(Default)]
struct ContainerFlags {
    fail_safe: bool,
    deserialize: bool,
}

mod attribute;

fn create_struct_tokens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let viz = input.vis.clone();
    // get the required bounds for serde destructuring '
    let container_attributes = ContainerFlags {
        fail_safe: find_path_of_attribute(&input.attrs, "fail_safe").is_some(),
        deserialize: find_path_of_attribute(&input.attrs, "deserialize").is_some(),
    };

    let type_information = crate::type_information::TypeInformation::new(input);

    let output_impls = create_impl_for_output(&type_information, &container_attributes);

    let end_punctuation = if matches!(&type_information.fields, &Fields::Unnamed(_)) {
        Some(syn::token::Semi::default())
    } else {
        None
    };

    let input_impls = { Some(create_input_impls(&type_information)) };

    #[cfg(feature = "serde")]
    let derive = if container_attributes.deserialize {
        quote! { #[derive(serde::Deserialize, Default )]}
    } else {
        quote! {#[derive(Default)]}
    };

    #[cfg(not(feature = "serde"))]
    let derive = quote! {#[derive(Default)]};

    #[cfg(not(feature = "serde"))]
    let attrs: Vec<syn::Attribute> = vec![];

    #[cfg(feature = "serde")]
    let attrs: Vec<_> = if container_attributes.deserialize {
        type_information.only_serde_attributes()
    } else {
        vec![]
    };

    let full_name = type_information.fully_qualified_derived_struct_name();
    let fields = type_information.fields_wrapped_in_options(&container_attributes);

    let tokens = quote! {
            #derive
            #(#attrs)*
            #viz struct #full_name#fields#end_punctuation

            #input_impls
            #output_impls
    };

    tokens.into()
}

fn create_input_impls(type_information: &TypeInformation) -> proc_macro2::TokenStream {
    let prefix_generics = &type_information.prefix_generics;
    let full_known_name = type_information.fully_qualified_wrapped_struct_name();
    let full_maybe_name = type_information.fully_qualified_derived_struct_name();
    let where_clause = &type_information.where_clause;

    quote! {
        impl#prefix_generics empty_type::EmptyType for #full_known_name#where_clause {
            type Container = #full_maybe_name;
        }
    }
}

fn create_impl_for_output(
    type_information: &TypeInformation,
    container_flags: &ContainerFlags,
) -> proc_macro2::TokenStream {
    let field_unwrapping = if container_flags.fail_safe {
        type_information.fields_uwnrapped_default()
    } else {
        type_information.fields_unwrapped()
    };

    let field_unwrapping = if type_information.is_tuple_struct() {
        let exprs = field_unwrapping.into_iter().map(|f| f.expr);
        quote! { ( #(#exprs),* ) }
    } else {
        quote! { { #field_unwrapping } }
    };

    let prefix_generics = &type_information.prefix_generics;
    let fully_qualified_derive_name = type_information.fully_qualified_derived_struct_name();
    let fully_qualified_wrapped_name = type_information.fully_qualified_wrapped_struct_name();
    let where_clause = &type_information.where_clause;
    let wrapped_name = &type_information.wrapped_struct_name;

    quote! {
        impl#prefix_generics empty_type::Container for #fully_qualified_derive_name#where_clause {
            type Value = #fully_qualified_wrapped_name;

            fn try_open(&mut self) -> Result<Self::Value, Box<dyn std::error::Error>> {
                return Ok(#wrapped_name#field_unwrapping)
            }
        }
    }
}
