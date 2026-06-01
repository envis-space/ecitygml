use crate::model::transportation::Marking;

#[derive(Debug, Clone, PartialEq)]
pub struct MarkingProperty {
    pub object: Option<Marking>,
    pub href: Option<String>,
}

impl MarkingProperty {
    pub fn new(object: Marking) -> Self {
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
