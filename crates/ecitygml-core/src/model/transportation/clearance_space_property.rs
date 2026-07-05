use crate::model::transportation::ClearanceSpace;

#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceSpaceProperty {
    pub object: Option<ClearanceSpace>,
    pub href: Option<String>,
}

impl ClearanceSpaceProperty {
    pub fn new(object: ClearanceSpace) -> Self {
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
