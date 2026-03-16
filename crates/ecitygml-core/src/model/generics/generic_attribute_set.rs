use crate::model::generics::GenericAttributeKind;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttributeSet {
    pub name: String,
    pub generic_attributes: Vec<GenericAttributeKind>,
}

impl GenericAttributeSet {
    pub fn new(name: String, generic_attributes: Vec<GenericAttributeKind>) -> Self {
        Self {
            name,
            generic_attributes,
        }
    }
}
