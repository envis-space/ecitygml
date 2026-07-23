use crate::model::core::CityObjectRelation;
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct CityObjectRelationProperty {
    object: Option<CityObjectRelation>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl CityObjectRelationProperty {
    pub fn new(
        object: Option<CityObjectRelation>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: CityObjectRelation) -> Self {
        Self {
            object: Some(object),
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            object: None,
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn object(&self) -> Option<&CityObjectRelation> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut CityObjectRelation> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<CityObjectRelation> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: CityObjectRelation) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<CityObjectRelation>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for CityObjectRelationProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for CityObjectRelationProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for CityObjectRelationProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for CityObjectRelationProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
