use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, Door, FillingElementKind, Window,
};

#[derive(Debug, Clone, Copy)]
pub enum FillingElementKindRef<'a> {
    Door(&'a Door),
    Window(&'a Window),
}

impl<'a> From<&'a FillingElementKind> for FillingElementKindRef<'a> {
    fn from(item: &'a FillingElementKind) -> Self {
        match item {
            FillingElementKind::Door(x) => Self::Door(x),
            FillingElementKind::Window(x) => Self::Window(x),
        }
    }
}

impl<'a> AsAbstractFillingElement for FillingElementKindRef<'a> {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        match self {
            Self::Door(x) => x.abstract_filling_element(),
            Self::Window(x) => x.abstract_filling_element(),
        }
    }
}
crate::impl_abstract_filling_element_traits!(FillingElementKindRef<'_>);

impl<'a> HasFeatureType for FillingElementKindRef<'a> {
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
        impl<'a> From<&'a $type> for $crate::model::construction::refs::FillingElementKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::FillingElementKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(FillingElementKind, $type);
    };
}
impl_from_filling_element_kind_ref!(Door);
impl_from_filling_element_kind_ref!(Window);

#[macro_export]
macro_rules! impl_try_from_filling_element_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::FillingElementKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::FillingElementKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::FillingElementKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(FillingElementKind, $type);
    };
}
impl_try_from_filling_element_kind_ref!(Door);
impl_try_from_filling_element_kind_ref!(Window);
impl_try_from_occupied_space_kind_ref_for_enum!(FillingElementKind, FillingElementKindRef);
