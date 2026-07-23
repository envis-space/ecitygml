use crate::impl_try_from_space_boundary_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::relief::{
    AbstractReliefComponent, AbstractReliefComponentKind, AsAbstractReliefComponent,
    AsAbstractReliefComponentMut, TinRelief,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractReliefComponentKindRefMut<'a> {
    TinRelief(&'a mut TinRelief),
}

impl<'a> From<&'a mut AbstractReliefComponentKind> for AbstractReliefComponentKindRefMut<'a> {
    fn from(item: &'a mut AbstractReliefComponentKind) -> Self {
        match item {
            AbstractReliefComponentKind::TinRelief(x) => Self::TinRelief(x),
        }
    }
}

impl<'a> AsAbstractReliefComponent for AbstractReliefComponentKindRefMut<'a> {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        match self {
            Self::TinRelief(x) => x.abstract_relief_component(),
        }
    }
}

impl<'a> AsAbstractReliefComponentMut for AbstractReliefComponentKindRefMut<'a> {
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent {
        match self {
            Self::TinRelief(x) => x.abstract_relief_component_mut(),
        }
    }
}
crate::impl_abstract_relief_component_traits!(AbstractReliefComponentKindRefMut<'_>);
crate::impl_abstract_relief_component_mut_traits!(AbstractReliefComponentKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractReliefComponentKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::TinRelief(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_relief_component_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::relief::refs::AbstractReliefComponentKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::relief::refs::AbstractReliefComponentKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref_mut!(AbstractReliefComponentKind, $type);
    };
}
impl_from_relief_component_kind_ref_mut!(TinRelief);

#[macro_export]
macro_rules! impl_try_from_relief_component_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::relief::refs::AbstractReliefComponentKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::relief::refs::AbstractReliefComponentKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::relief::refs::AbstractReliefComponentKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref_mut!(AbstractReliefComponentKind, $type);
    };
}
impl_try_from_relief_component_kind_ref_mut!(TinRelief);
impl_try_from_space_boundary_kind_ref_mut_for_enum!(
    AbstractReliefComponentKind,
    AbstractReliefComponentKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractReliefComponentKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::TinRelief(x) => x.recompute_bounding_shape(),
        }
    }
}
