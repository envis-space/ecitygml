use crate::impl_try_from_unoccupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::transportation::{
    AbstractTransportationSpace, AbstractTransportationSpaceKind, AsAbstractTransportationSpace,
    Intersection, Railway, Road, Section, Square, Track, Waterway,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractTransportationSpaceKindRef<'a> {
    Section(&'a Section),
    Intersection(&'a Intersection),
    Road(&'a Road),
    Track(&'a Track),
    Railway(&'a Railway),
    Waterway(&'a Waterway),
    Square(&'a Square),
}

impl<'a> From<&'a AbstractTransportationSpaceKind> for AbstractTransportationSpaceKindRef<'a> {
    fn from(item: &'a AbstractTransportationSpaceKind) -> Self {
        match item {
            AbstractTransportationSpaceKind::Section(x) => Self::Section(x),
            AbstractTransportationSpaceKind::Intersection(x) => Self::Intersection(x),
            AbstractTransportationSpaceKind::Road(x) => Self::Road(x),
            AbstractTransportationSpaceKind::Track(x) => Self::Track(x),
            AbstractTransportationSpaceKind::Railway(x) => Self::Railway(x),
            AbstractTransportationSpaceKind::Waterway(x) => Self::Waterway(x),
            AbstractTransportationSpaceKind::Square(x) => Self::Square(x),
        }
    }
}

impl<'a> AsAbstractTransportationSpace for AbstractTransportationSpaceKindRef<'a> {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        match self {
            Self::Section(x) => x.abstract_transportation_space(),
            Self::Intersection(x) => x.abstract_transportation_space(),
            Self::Road(x) => x.abstract_transportation_space(),
            Self::Track(x) => x.abstract_transportation_space(),
            Self::Railway(x) => x.abstract_transportation_space(),
            Self::Waterway(x) => x.abstract_transportation_space(),
            Self::Square(x) => x.abstract_transportation_space(),
        }
    }
}
crate::impl_abstract_transportation_space_traits!(AbstractTransportationSpaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractTransportationSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::Section(_) => FeatureType::Section,
            Self::Intersection(_) => FeatureType::Intersection,
            Self::Road(_) => FeatureType::Road,
            Self::Track(_) => FeatureType::Track,
            Self::Railway(_) => FeatureType::Railway,
            Self::Waterway(_) => FeatureType::Waterway,
            Self::Square(_) => FeatureType::Square,
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type>
            for $crate::model::transportation::refs::AbstractTransportationSpaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::transportation::refs::AbstractTransportationSpaceKindRef::$type(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_unoccupied_space_kind_ref!(AbstractTransportationSpaceKind, $type);
    };
}
impl_from_transportation_space_kind_ref!(Section);
impl_from_transportation_space_kind_ref!(Intersection);
impl_from_transportation_space_kind_ref!(Road);
impl_from_transportation_space_kind_ref!(Track);
impl_from_transportation_space_kind_ref!(Railway);
impl_from_transportation_space_kind_ref!(Waterway);
impl_from_transportation_space_kind_ref!(Square);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::transportation::refs::AbstractTransportationSpaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::refs::AbstractTransportationSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::refs::AbstractTransportationSpaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind_ref!(AbstractTransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind_ref!(Section);
impl_try_from_transportation_space_kind_ref!(Intersection);
impl_try_from_transportation_space_kind_ref!(Road);
impl_try_from_transportation_space_kind_ref!(Track);
impl_try_from_transportation_space_kind_ref!(Railway);
impl_try_from_transportation_space_kind_ref!(Waterway);
impl_try_from_transportation_space_kind_ref!(Square);
impl_try_from_unoccupied_space_kind_ref_for_enum!(
    AbstractTransportationSpaceKind,
    AbstractTransportationSpaceKindRef
);
