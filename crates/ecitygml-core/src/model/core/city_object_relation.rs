use crate::model::core::values::RelationTypeValue;
use crate::model::generics::GenericAttributeKind;
use egml::model::base::Reference;

#[derive(Debug, Clone, PartialEq)]
pub struct CityObjectRelation {
    relation_type: RelationTypeValue,
    relation_to: Reference,
    generic_attributes: Vec<GenericAttributeKind>,
}

impl CityObjectRelation {
    pub fn new(relation_type: RelationTypeValue, relation_to: Reference) -> Self {
        Self {
            relation_type,
            relation_to,
            generic_attributes: Vec::new(),
        }
    }

    pub fn relation_type(&self) -> &RelationTypeValue {
        &self.relation_type
    }

    pub fn relation_to(&self) -> &Reference {
        &self.relation_to
    }

    pub fn generic_attributes(&self) -> &[GenericAttributeKind] {
        &self.generic_attributes
    }

    pub fn generic_attributes_mut(&mut self) -> &mut [GenericAttributeKind] {
        &mut self.generic_attributes
    }

    pub fn set_generic_attributes(&mut self, generic_attributes: Vec<GenericAttributeKind>) {
        self.generic_attributes = generic_attributes;
    }

    pub fn push_generic_attribute(&mut self, generic_attribute: GenericAttributeKind) {
        self.generic_attributes.push(generic_attribute);
    }

    pub fn extend_generic_attributes(
        &mut self,
        generic_attributes: impl IntoIterator<Item = GenericAttributeKind>,
    ) {
        self.generic_attributes.extend(generic_attributes);
    }
}
