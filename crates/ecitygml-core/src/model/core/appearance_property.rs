use crate::model::core::AppearanceKind;

#[derive(Debug, Clone, PartialEq)]
pub struct AppearanceProperty {
    pub object: Option<AppearanceKind>,
    pub href: Option<String>,
}

impl AppearanceProperty {
    pub fn new(object: AppearanceKind) -> Self {
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
