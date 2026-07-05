use crate::impl_try_from_space_boundary_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, ReliefComponentKind, TinRelief,
};

#[derive(Debug, Clone, Copy)]
pub enum ReliefComponentKindRef<'a> {
    TinRelief(&'a TinRelief),
}

impl<'a> From<&'a ReliefComponentKind> for ReliefComponentKindRef<'a> {
    fn from(item: &'a ReliefComponentKind) -> Self {
        match item {
            ReliefComponentKind::TinRelief(x) => Self::TinRelief(x),
        }
    }
}

impl<'a> AsAbstractReliefComponent for ReliefComponentKindRef<'a> {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        match self {
            Self::TinRelief(x) => x.abstract_relief_component(),
        }
    }
}
crate::impl_abstract_relief_component_traits!(ReliefComponentKindRef<'_>);

impl<'a> HasFeatureType for ReliefComponentKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::TinRelief(_) => FeatureType::TinRelief,
        }
    }
}

#[macro_export]
macro_rules! impl_from_relief_component_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::relief::refs::ReliefComponentKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::relief::refs::ReliefComponentKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref!(ReliefComponentKind, $type);
    };
}
impl_from_relief_component_kind_ref!(TinRelief);

#[macro_export]
macro_rules! impl_try_from_relief_component_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::relief::refs::ReliefComponentKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::relief::refs::ReliefComponentKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::relief::refs::ReliefComponentKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref!(ReliefComponentKind, $type);
    };
}
impl_try_from_relief_component_kind_ref!(TinRelief);
impl_try_from_space_boundary_kind_ref_for_enum!(ReliefComponentKind, ReliefComponentKindRef);
