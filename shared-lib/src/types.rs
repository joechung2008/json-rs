/// Box token to enable recursive enum types.
#[derive(Debug)]
pub struct Json {
    pub skip: usize,
    pub token: Box<ValueToken>,
}

#[derive(Debug)]
pub enum ValueToken {
    ArrayToken { skip: usize, token: Array },
    FalseToken { skip: usize, token: bool },
    NullToken { skip: usize },
    NumberToken { skip: usize, token: Number },
    ObjectToken { skip: usize, token: Object },
    PairToken { skip: usize, token: Pair },
    StringToken { skip: usize, token: String },
    TrueToken { skip: usize, token: bool },
}

#[derive(Debug)]
pub struct Array {
    /// Boxing is required because ValueToken is a recursive type.
    #[allow(clippy::vec_box)]
    pub values: Vec<Box<ValueToken>>,
}

#[derive(Debug)]
pub struct Number {
    pub value: f64,
    pub value_as_string: String,
}

#[derive(Debug)]
pub struct Object {
    pub members: Vec<Pair>,
}

#[derive(Debug)]
pub struct Pair {
    pub key: String,
    /// Box value to enable recursive enum types.
    pub value: Box<ValueToken>,
}
