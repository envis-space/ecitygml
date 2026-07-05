use crate::model::transportation::Section;

#[derive(Debug, Clone, PartialEq)]
pub struct SectionProperty {
    pub object: Option<Section>,
    pub href: Option<String>,
}

impl SectionProperty {
    pub fn new(object: Section) -> Self {
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
