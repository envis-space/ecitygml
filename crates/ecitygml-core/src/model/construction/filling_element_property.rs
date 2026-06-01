use crate::model::construction::FillingElementKind;

#[derive(Debug, Clone, PartialEq)]
pub struct FillingElementProperty {
    pub object: Option<FillingElementKind>,
    pub href: Option<String>,
}

impl FillingElementProperty {
    pub fn new(object: FillingElementKind) -> Self {
        Self {
            object: Some(object),
            href: None,
        }
    }

    pub fn new_href(href: String) -> Self {
        Self {
            object: None,
            href: Some(href),
        }
    }
}
