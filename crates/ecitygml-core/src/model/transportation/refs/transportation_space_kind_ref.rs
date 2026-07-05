use crate::impl_try_from_unoccupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, Intersection, Road, Section,
    TransportationSpaceKind,
};

#[derive(Debug, Clone, Copy)]
pub enum TransportationSpaceKindRef<'a> {
    Section(&'a Section),
    Intersection(&'a Intersection),
    Road(&'a Road),
}

impl<'a> From<&'a TransportationSpaceKind> for TransportationSpaceKindRef<'a> {
    fn from(item: &'a TransportationSpaceKind) -> Self {
        match item {
            TransportationSpaceKind::Section(x) => Self::Section(x),
            TransportationSpaceKind::Intersection(x) => Self::Intersection(x),
            TransportationSpaceKind::Road(x) => Self::Road(x),
        }
    }
}

impl<'a> AsAbstractTransportationSpace for TransportationSpaceKindRef<'a> {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        match self {
            Self::Section(x) => x.abstract_transportation_space(),
            Self::Intersection(x) => x.abstract_transportation_space(),
            Self::Road(x) => x.abstract_transportation_space(),
        }
    }
}
crate::impl_abstract_transportation_space_traits!(TransportationSpaceKindRef<'_>);

impl<'a> HasFeatureType for TransportationSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::Section(_) => FeatureType::Section,
            Self::Intersection(_) => FeatureType::Intersection,
            Self::Road(_) => FeatureType::Road,
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type>
            for $crate::model::transportation::refs::TransportationSpaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::transportation::refs::TransportationSpaceKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_unoccupied_space_kind_ref!(TransportationSpaceKind, $type);
    };
}
impl_from_transportation_space_kind_ref!(Section);
impl_from_transportation_space_kind_ref!(Intersection);
impl_from_transportation_space_kind_ref!(Road);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::transportation::refs::TransportationSpaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::refs::TransportationSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::refs::TransportationSpaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind_ref!(TransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind_ref!(Section);
impl_try_from_transportation_space_kind_ref!(Intersection);
impl_try_from_transportation_space_kind_ref!(Road);
impl_try_from_unoccupied_space_kind_ref_for_enum!(
    TransportationSpaceKind,
    TransportationSpaceKindRef
);
