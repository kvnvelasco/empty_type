use syn::{Attribute, Lit, Meta, NestedMeta, Path};

const ATTRIBUTE_NAME: &str = "empty";

fn filter_attrs_by_own(attribute: &Attribute) -> bool {
    attribute.path.is_ident(ATTRIBUTE_NAME)
}

pub fn find_path_of_attribute(attributes: &[Attribute], name: &'static str) -> Option<Path> {
    attributes
        .iter()
        .filter(|x| filter_attrs_by_own(*x))
        .find_map(|attr| find_path_in_attribute(attr, name))
}

fn find_path_in_attribute(attr: &Attribute, name: &'static str) -> Option<Path> {
    let meta = attr.parse_meta().expect("Expected arguments to attribute");
    find_path_in_meta(&meta, name)
}

fn find_path_in_meta(meta: &Meta, name: &'static str) -> Option<Path> {
    match meta {
        Meta::Path(path) if path.is_ident(name) => Some(path.clone()),
        Meta::List(list) => list
            .nested
            .iter()
            .filter_map(|nested| {
                if let NestedMeta::Meta(meta) = nested {
                    Some(meta)
                } else {
                    None
                }
            })
            .find_map(|meta| find_path_in_meta(meta, name)),
        Meta::NameValue(named_value) if named_value.path.is_ident(name) => {
            Some(named_value.path.clone())
        }
        _ => None,
    }
}

#[allow(dead_code)]
fn get_meta_value(meta: &Meta, key: &Path) -> Option<Lit> {
    match meta {
        Meta::Path(_) => None,
        Meta::List(list) => list
            .nested
            .iter()
            .filter_map(|nested| {
                if let NestedMeta::Meta(meta) = nested {
                    Some(meta)
                } else {
                    None
                }
            })
            .find_map(move |meta| get_meta_value(meta, &key)),

        Meta::NameValue(named_value) if named_value.path.get_ident() == key.get_ident() => {
            Some(named_value.lit.clone())
        }
        _ => None,
    }
}
#[allow(dead_code)]
fn get_value_in_attribute(attribute: &Attribute, name: &Path) -> Option<Lit> {
    let meta = attribute
        .parse_meta()
        .expect("Expected attribute to have meta");
    get_meta_value(&meta, &name)
}

#[allow(dead_code)]
pub fn get_attribute_value(attributes: &[Attribute], name: &'static str) -> Option<Lit> {
    let path = find_path_of_attribute(attributes, name)?;

    Some(
        attributes
            .iter()
            .filter(|x| filter_attrs_by_own(*x))
            .find_map(move |attr| get_value_in_attribute(attr, &path))
            .expect(concat!(stringify!(name), " should have a value")),
    )
}
