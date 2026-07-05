use crate::impl_try_from_unoccupied_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    Intersection, Road, Section, TransportationSpaceKind,
};

#[derive(Debug)]
pub enum TransportationSpaceKindRefMut<'a> {
    Section(&'a mut Section),
    Intersection(&'a mut Intersection),
    Road(&'a mut Road),
}

impl<'a> From<&'a mut TransportationSpaceKind> for TransportationSpaceKindRefMut<'a> {
    fn from(item: &'a mut TransportationSpaceKind) -> Self {
        match item {
            TransportationSpaceKind::Section(x) => Self::Section(x),
            TransportationSpaceKind::Intersection(x) => Self::Intersection(x),
            TransportationSpaceKind::Road(x) => Self::Road(x),
        }
    }
}

impl<'a> AsAbstractTransportationSpace for TransportationSpaceKindRefMut<'a> {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        match self {
            Self::Section(x) => x.abstract_transportation_space(),
            Self::Intersection(x) => x.abstract_transportation_space(),
            Self::Road(x) => x.abstract_transportation_space(),
        }
    }
}

impl<'a> AsAbstractTransportationSpaceMut for TransportationSpaceKindRefMut<'a> {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        match self {
            Self::Section(x) => x.abstract_transportation_space_mut(),
            Self::Intersection(x) => x.abstract_transportation_space_mut(),
            Self::Road(x) => x.abstract_transportation_space_mut(),
        }
    }
}
crate::impl_abstract_transportation_space_traits!(TransportationSpaceKindRefMut<'_>);
crate::impl_abstract_transportation_space_mut_traits!(TransportationSpaceKindRefMut<'_>);

impl<'a> TransportationSpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Section(x) => x.recompute_bounding_shape(),
            Self::Intersection(x) => x.recompute_bounding_shape(),
            Self::Road(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for TransportationSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Section(x) => x.feature_type(),
            Self::Intersection(x) => x.feature_type(),
            Self::Road(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::transportation::refs::TransportationSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::transportation::refs::TransportationSpaceKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_unoccupied_space_kind_ref_mut!(TransportationSpaceKind, $type);
    };
}
impl_from_transportation_space_kind_ref_mut!(Section);
impl_from_transportation_space_kind_ref_mut!(Intersection);
impl_from_transportation_space_kind_ref_mut!(Road);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::transportation::refs::TransportationSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::refs::TransportationSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::refs::TransportationSpaceKindRefMut::$type(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind_ref_mut!(TransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind_ref_mut!(Section);
impl_try_from_transportation_space_kind_ref_mut!(Intersection);
impl_try_from_transportation_space_kind_ref_mut!(Road);
impl_try_from_unoccupied_space_kind_ref_mut_for_enum!(
    TransportationSpaceKind,
    TransportationSpaceKindRefMut
);
