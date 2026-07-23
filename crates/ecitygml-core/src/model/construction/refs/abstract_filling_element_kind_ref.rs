use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingElement, AbstractFillingElementKind, AsAbstractFillingElement, Door, Window,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractFillingElementKindRef<'a> {
    Door(&'a Door),
    Window(&'a Window),
}

impl<'a> From<&'a AbstractFillingElementKind> for AbstractFillingElementKindRef<'a> {
    fn from(item: &'a AbstractFillingElementKind) -> Self {
        match item {
            AbstractFillingElementKind::Door(x) => Self::Door(x),
            AbstractFillingElementKind::Window(x) => Self::Window(x),
        }
    }
}

impl<'a> AsAbstractFillingElement for AbstractFillingElementKindRef<'a> {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        match self {
            Self::Door(x) => x.abstract_filling_element(),
            Self::Window(x) => x.abstract_filling_element(),
        }
    }
}
crate::impl_abstract_filling_element_traits!(AbstractFillingElementKindRef<'_>);

impl<'a> HasFeatureType for AbstractFillingElementKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::Door(_) => FeatureType::Door,
            Self::Window(_) => FeatureType::Window,
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_element_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type>
            for $crate::model::construction::refs::AbstractFillingElementKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::AbstractFillingElementKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(AbstractFillingElementKind, $type);
    };
}
impl_from_filling_element_kind_ref!(Door);
impl_from_filling_element_kind_ref!(Window);

#[macro_export]
macro_rules! impl_try_from_filling_element_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractFillingElementKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractFillingElementKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractFillingElementKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(AbstractFillingElementKind, $type);
    };
}
impl_try_from_filling_element_kind_ref!(Door);
impl_try_from_filling_element_kind_ref!(Window);
impl_try_from_occupied_space_kind_ref_for_enum!(
    AbstractFillingElementKind,
    AbstractFillingElementKindRef
);
