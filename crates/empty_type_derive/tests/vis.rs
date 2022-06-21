mod test {
    use empty_type_derive::EmptyType;
    use empty_type_traits as empty_type;
    #[derive(EmptyType)]
    pub struct TestStruct {
        value: Option<&'static str>,
    }
}
