use crate::model::core::CityObjectKind;

#[derive(Debug, Clone, PartialEq)]
pub struct CityObjectProperty {
    pub object: Option<CityObjectKind>,
    pub href: Option<String>,
}

impl CityObjectProperty {
    pub fn new(object: CityObjectKind) -> Self {
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
