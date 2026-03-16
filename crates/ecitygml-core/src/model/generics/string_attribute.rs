#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct StringAttribute {
    pub name: String,
    pub value: String,
}

impl StringAttribute {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
