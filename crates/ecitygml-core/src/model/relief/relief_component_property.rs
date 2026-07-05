use crate::model::relief::ReliefComponentKind;

#[derive(Debug, Clone, PartialEq)]
pub struct ReliefComponentProperty {
    pub object: Option<ReliefComponentKind>,
    pub href: Option<String>,
}

impl ReliefComponentProperty {
    pub fn new(object: ReliefComponentKind) -> Self {
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
