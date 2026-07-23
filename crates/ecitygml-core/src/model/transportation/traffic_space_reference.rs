use egml::model::base::Reference;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrafficSpaceReference(Reference);

impl From<Reference> for TrafficSpaceReference {
    fn from(reference: Reference) -> Self {
        Self(reference)
    }
}

impl From<TrafficSpaceReference> for Reference {
    fn from(reference: TrafficSpaceReference) -> Self {
        reference.0
    }
}

impl From<&TrafficSpaceReference> for Reference {
    fn from(reference: &TrafficSpaceReference) -> Self {
        reference.0.clone()
    }
}
