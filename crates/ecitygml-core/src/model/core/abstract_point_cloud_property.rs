use crate::model::common::arena::{ArenaProperties, HasArenaProperties, HasArenaPropertiesMut};
use crate::model::core::AbstractPointCloudKind;
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractPointCloudProperty {
    object: Option<AbstractPointCloudKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
    arena: ArenaProperties,
}

impl AbstractPointCloudProperty {
    pub fn new(
        object: Option<AbstractPointCloudKind>,
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

    pub fn from_object(object: AbstractPointCloudKind) -> Self {
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

    pub fn object(&self) -> Option<&AbstractPointCloudKind> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut AbstractPointCloudKind> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<AbstractPointCloudKind> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: AbstractPointCloudKind) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<AbstractPointCloudKind>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for AbstractPointCloudProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractPointCloudProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractPointCloudProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractPointCloudProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

impl HasArenaProperties for AbstractPointCloudProperty {
    fn arena_properties(&self) -> &ArenaProperties {
        &self.arena
    }
}

impl HasArenaPropertiesMut for AbstractPointCloudProperty {
    fn arena_properties_mut(&mut self) -> &mut ArenaProperties {
        &mut self.arena
    }
}
