use crate::model::transportation::Intersection;

#[derive(Debug, Clone, PartialEq)]
pub struct IntersectionProperty {
    pub object: Option<Intersection>,
    pub href: Option<String>,
}

impl IntersectionProperty {
    pub fn new(object: Intersection) -> Self {
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
