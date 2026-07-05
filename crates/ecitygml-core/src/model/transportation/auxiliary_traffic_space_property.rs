use crate::model::transportation::AuxiliaryTrafficSpace;

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryTrafficSpaceProperty {
    pub object: Option<AuxiliaryTrafficSpace>,
    pub href: Option<String>,
}

impl AuxiliaryTrafficSpaceProperty {
    pub fn new(object: AuxiliaryTrafficSpace) -> Self {
        Self {
            object: Some(object),
            href: None,
        }
    }

    pub fn new_href(href: impl Into<String>) -> Self {
        Self {
            object: None,
            href: Some(href.into()),
        }
    }
}
