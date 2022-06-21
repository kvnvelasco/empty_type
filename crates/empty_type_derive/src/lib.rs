use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

use crate::attribute::find_path_of_attribute;
use crate::type_information::TypeInformation;
use quote::ToTokens;

mod fields;
mod type_information;

#[proc_macro_derive(EmptyType, attributes(empty))]
pub fn empty_type(input: TokenStream) -> TokenStream {
    create_struct_tokens(input)
}

#[derive(Default)]
struct ContainerFlags {
    #[cfg(feature = "serde")]
    bounds: Option<syn::Lifetime>,
    default: bool,
    deserialize: bool,
}

mod attribute;

fn create_struct_tokens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let viz = input.vis.clone();
    // get the required bounds for serde destructuring '
    let container_attributes = ContainerFlags {
        default: find_path_of_attribute(&input.attrs, "default").is_some(),
        deserialize: find_path_of_attribute(&input.attrs, "deserialize").is_some(),

        #[cfg(feature = "serde")]
        bounds: attribute::get_attribute_value(&input.attrs, "bounds").map(|v| {
            if let syn::Lit::Str(lit_str) = v {
                lit_str.parse().unwrap()
            } else {
                panic!("")
            }
        }),
    };

    let type_information = crate::type_information::TypeInformation::new(input);

    let output_impls = create_impl_for_output(&type_information, &container_attributes);

    let end_punctuation = if matches!(&type_information.fields, &Fields::Unnamed(_)) {
        Some(syn::token::Semi::default())
    } else {
        None
    };

    let input_impls = { Some(create_input_impls(&type_information, &container_attributes)) };

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

fn create_input_impls(
    type_information: &TypeInformation,
    container_attributes: &ContainerFlags,
) -> proc_macro2::TokenStream {
    #[cfg(not(feature = "serde"))]
    let prefix_lifetime: Option<proc_macro2::TokenStream> = None;

    #[cfg(not(feature = "serde"))]
    let infix_lifetime = quote! {'static};

    #[cfg(feature = "serde")]
    let prefix_lifetime = if let Some(lifetime) = &container_attributes.bounds {
        Some(quote! { 'de: #lifetime  })
    } else if container_attributes.deserialize {
        Some(quote! { 'de })
    } else {
        None
    };

    #[cfg(feature = "serde")]
    let infix_lifetime = if container_attributes.deserialize {
        quote! {'de }
    } else {
        quote! { 'static }
    };

    let mut with_de_prefix_generics = type_information.prefix_generics.clone();
    if let Some(prefix_lifetime) = prefix_lifetime {
        with_de_prefix_generics
            .params
            .push(syn::parse_quote! { #prefix_lifetime })
    };

    let mut noned_fields = type_information.empty_fields().to_token_stream();

    let derived_name = type_information.derived_struct_name();
    let full_known_name = type_information.fully_qualified_wrapped_struct_name();
    let full_maybe_name = type_information.fully_qualified_derived_struct_name();
    let where_clause = &type_information.where_clause;

    if type_information.is_tuple_struct() {
        noned_fields = quote! { (#noned_fields) }
    } else {
        noned_fields = quote! { {#noned_fields} }
    }

    let deserialize_empty_impl = if container_attributes.deserialize {
        quote! {
            fn deserialize_empty<D>(deserializer: D) -> Result<empty_type::Empty<#full_maybe_name, #full_known_name>, D::Error>
            where D: serde::Deserializer<'de > {
                let value =  serde::Deserialize::deserialize(deserializer)?;
                return Ok(empty_type::Empty(value, Default::default()));
            }

        }
    } else {
        quote! {}
    };

    quote! {
        impl#with_de_prefix_generics empty_type::EmptyType<#infix_lifetime, #full_known_name> for #full_known_name#where_clause {
         type Container = #full_maybe_name;

         fn new_empty() -> empty_type::Empty<Self::Container, #full_known_name> {
            return empty_type::Empty(#derived_name#noned_fields, Default::default());
         }

         #deserialize_empty_impl
        }
    }
}

fn create_impl_for_output(
    type_information: &TypeInformation,
    container_flags: &ContainerFlags,
) -> proc_macro2::TokenStream {
    let mut field_unwrapping = type_information.fields_unwrapped().to_token_stream();

    if type_information.is_tuple_struct() {
        field_unwrapping = quote! { ( #field_unwrapping ) }
    } else {
        field_unwrapping = quote! { { #field_unwrapping } }
    }

    let prefix_generics = &type_information.prefix_generics;
    let fully_qualified_derive_name = type_information.fully_qualified_derived_struct_name();
    let fully_qualified_wrapped_name = type_information.fully_qualified_wrapped_struct_name();
    let where_clause = &type_information.where_clause;
    let wrapped_name = &type_information.wrapped_struct_name;

    let unwrap_default_impl = if container_flags.default {
        Some(quote! {
            fn unwrap_or_default(self) -> Self::Value
            where
                Self: Sized,
                Self::Value: std::default::Default {
                return #wrapped_name::default();
            }
        })
    } else {
        None
    };
    quote! {
        impl#prefix_generics empty_type::Unwrap for #fully_qualified_derive_name#where_clause {
            type Value = #fully_qualified_wrapped_name;

            fn unwrap(self) -> Self::Value {
                return #wrapped_name#field_unwrapping
            }

            fn unwrap_with_hint(self, _hint: &'static str) -> Self::Value {
                return self.unwrap();
            }

            #unwrap_default_impl
        }
    }
}
