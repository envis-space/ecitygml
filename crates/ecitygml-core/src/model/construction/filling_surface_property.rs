use crate::model::construction::FillingSurfaceKind;

#[derive(Debug, Clone, PartialEq)]
pub struct FillingSurfaceProperty {
    pub object: Option<FillingSurfaceKind>,
    pub href: Option<String>,
}

impl FillingSurfaceProperty {
    pub fn new(object: FillingSurfaceKind) -> Self {
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
