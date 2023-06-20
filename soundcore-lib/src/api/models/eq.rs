pub struct EQConfig<const T: usize> {
    pub profile: Option<String>,
    pub values: [i8; T],
}
