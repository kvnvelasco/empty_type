mod test {
    use empty_type_derive::EmptyType;

    #[derive(EmptyType)]
    pub struct TestStruct {
        value: Option<&'static str>,
    }
}
