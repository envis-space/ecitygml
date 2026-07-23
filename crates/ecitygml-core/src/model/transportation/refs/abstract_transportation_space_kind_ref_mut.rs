use crate::impl_try_from_unoccupied_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::transportation::{
    AbstractTransportationSpace, AbstractTransportationSpaceKind, AsAbstractTransportationSpace,
    AsAbstractTransportationSpaceMut, Intersection, Railway, Road, Section, Square, Track,
    Waterway,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractTransportationSpaceKindRefMut<'a> {
    Section(&'a mut Section),
    Intersection(&'a mut Intersection),
    Road(&'a mut Road),
    Track(&'a mut Track),
    Railway(&'a mut Railway),
    Waterway(&'a mut Waterway),
    Square(&'a mut Square),
}

impl<'a> From<&'a mut AbstractTransportationSpaceKind>
    for AbstractTransportationSpaceKindRefMut<'a>
{
    fn from(item: &'a mut AbstractTransportationSpaceKind) -> Self {
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

impl<'a> AsAbstractTransportationSpace for AbstractTransportationSpaceKindRefMut<'a> {
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

impl<'a> AsAbstractTransportationSpaceMut for AbstractTransportationSpaceKindRefMut<'a> {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        match self {
            Self::Section(x) => x.abstract_transportation_space_mut(),
            Self::Intersection(x) => x.abstract_transportation_space_mut(),
            Self::Road(x) => x.abstract_transportation_space_mut(),
            Self::Track(x) => x.abstract_transportation_space_mut(),
            Self::Railway(x) => x.abstract_transportation_space_mut(),
            Self::Waterway(x) => x.abstract_transportation_space_mut(),
            Self::Square(x) => x.abstract_transportation_space_mut(),
        }
    }
}
crate::impl_abstract_transportation_space_traits!(AbstractTransportationSpaceKindRefMut<'_>);
crate::impl_abstract_transportation_space_mut_traits!(AbstractTransportationSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractTransportationSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Section(x) => x.feature_type(),
            Self::Intersection(x) => x.feature_type(),
            Self::Road(x) => x.feature_type(),
            Self::Track(x) => x.feature_type(),
            Self::Railway(x) => x.feature_type(),
            Self::Waterway(x) => x.feature_type(),
            Self::Square(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut::$type(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_unoccupied_space_kind_ref_mut!(
            AbstractTransportationSpaceKind,
            $type
        );
    };
}
impl_from_transportation_space_kind_ref_mut!(Section);
impl_from_transportation_space_kind_ref_mut!(Intersection);
impl_from_transportation_space_kind_ref_mut!(Road);
impl_from_transportation_space_kind_ref_mut!(Track);
impl_from_transportation_space_kind_ref_mut!(Railway);
impl_from_transportation_space_kind_ref_mut!(Waterway);
impl_from_transportation_space_kind_ref_mut!(Square);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut::$type(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind_ref_mut!(AbstractTransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind_ref_mut!(Section);
impl_try_from_transportation_space_kind_ref_mut!(Intersection);
impl_try_from_transportation_space_kind_ref_mut!(Road);
impl_try_from_transportation_space_kind_ref_mut!(Track);
impl_try_from_transportation_space_kind_ref_mut!(Railway);
impl_try_from_transportation_space_kind_ref_mut!(Waterway);
impl_try_from_transportation_space_kind_ref_mut!(Square);
impl_try_from_unoccupied_space_kind_ref_mut_for_enum!(
    AbstractTransportationSpaceKind,
    AbstractTransportationSpaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractTransportationSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Section(x) => x.recompute_bounding_shape(),
            Self::Intersection(x) => x.recompute_bounding_shape(),
            Self::Road(x) => x.recompute_bounding_shape(),
            Self::Track(x) => x.recompute_bounding_shape(),
            Self::Railway(x) => x.recompute_bounding_shape(),
            Self::Waterway(x) => x.recompute_bounding_shape(),
            Self::Square(x) => x.recompute_bounding_shape(),
        }
    }
}
