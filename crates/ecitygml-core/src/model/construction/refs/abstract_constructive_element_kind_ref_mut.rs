use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingConstructiveElement;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractConstructiveElement, AbstractConstructiveElementKind, AsAbstractConstructiveElement,
    AsAbstractConstructiveElementMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractConstructiveElementKindRefMut<'a> {
    BuildingConstructiveElement(&'a mut BuildingConstructiveElement),
}

impl<'a> From<&'a mut AbstractConstructiveElementKind>
    for AbstractConstructiveElementKindRefMut<'a>
{
    fn from(item: &'a mut AbstractConstructiveElementKind) -> Self {
        match item {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                Self::BuildingConstructiveElement(x)
            }
        }
    }
}

impl<'a> AsAbstractConstructiveElement for AbstractConstructiveElementKindRefMut<'a> {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        match self {
            Self::BuildingConstructiveElement(x) => x.abstract_constructive_element(),
        }
    }
}

impl<'a> AsAbstractConstructiveElementMut for AbstractConstructiveElementKindRefMut<'a> {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        match self {
            Self::BuildingConstructiveElement(x) => x.abstract_constructive_element_mut(),
        }
    }
}
crate::impl_abstract_constructive_element_traits!(AbstractConstructiveElementKindRefMut<'_>);
crate::impl_abstract_constructive_element_mut_traits!(AbstractConstructiveElementKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractConstructiveElementKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingConstructiveElement(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_constructive_element_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::AbstractConstructiveElementKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::AbstractConstructiveElementKindRefMut::$type(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(AbstractConstructiveElementKind, $type);
    };
}
impl_from_constructive_element_kind_ref_mut!(BuildingConstructiveElement);

#[macro_export]
macro_rules! impl_try_from_constructive_element_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractConstructiveElementKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractConstructiveElementKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractConstructiveElementKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(AbstractConstructiveElementKind, $type);
    };
}
impl_try_from_constructive_element_kind_ref_mut!(BuildingConstructiveElement);
impl_try_from_occupied_space_kind_ref_mut_for_enum!(
    AbstractConstructiveElementKind,
    AbstractConstructiveElementKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractConstructiveElementKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingConstructiveElement(x) => x.recompute_bounding_shape(),
        }
    }
}
