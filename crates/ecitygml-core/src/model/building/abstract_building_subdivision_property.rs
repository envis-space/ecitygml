use crate::model::building::AbstractBuildingSubdivisionKind;
use crate::model::common::arena::{ArenaProperties, HasArenaProperties, HasArenaPropertiesMut};
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuildingSubdivisionProperty {
    object: Option<AbstractBuildingSubdivisionKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
    arena: ArenaProperties,
}

impl AbstractBuildingSubdivisionProperty {
    pub fn new(
        object: Option<AbstractBuildingSubdivisionKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
            arena: ArenaProperties::default(),
        }
    }

    pub fn from_object(object: AbstractBuildingSubdivisionKind) -> Self {
        Self {
            object: Some(object),
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
            arena: ArenaProperties::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            object: None,
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
            arena: ArenaProperties::default(),
        }
    }

    pub fn object(&self) -> Option<&AbstractBuildingSubdivisionKind> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut AbstractBuildingSubdivisionKind> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<AbstractBuildingSubdivisionKind> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: AbstractBuildingSubdivisionKind) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<AbstractBuildingSubdivisionKind>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for AbstractBuildingSubdivisionProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractBuildingSubdivisionProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractBuildingSubdivisionProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractBuildingSubdivisionProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

impl HasArenaProperties for AbstractBuildingSubdivisionProperty {
    fn arena_properties(&self) -> &ArenaProperties {
        &self.arena
    }
}

impl HasArenaPropertiesMut for AbstractBuildingSubdivisionProperty {
    fn arena_properties_mut(&mut self) -> &mut ArenaProperties {
        &mut self.arena
    }
}
