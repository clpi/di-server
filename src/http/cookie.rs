pub struct Cookie<'a> {
    key: &'a str,
    value: Option<&'a str>,
}
