use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut, Door,
    FillingElementKind, Window,
};

#[derive(Debug)]
pub enum FillingElementKindRefMut<'a> {
    Door(&'a mut Door),
    Window(&'a mut Window),
}

impl<'a> From<&'a mut FillingElementKind> for FillingElementKindRefMut<'a> {
    fn from(item: &'a mut FillingElementKind) -> Self {
        match item {
            FillingElementKind::Door(x) => Self::Door(x),
            FillingElementKind::Window(x) => Self::Window(x),
        }
    }
}

impl<'a> AsAbstractFillingElement for FillingElementKindRefMut<'a> {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        match self {
            Self::Door(x) => x.abstract_filling_element(),
            Self::Window(x) => x.abstract_filling_element(),
        }
    }
}

impl<'a> AsAbstractFillingElementMut for FillingElementKindRefMut<'a> {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        match self {
            Self::Door(x) => x.abstract_filling_element_mut(),
            Self::Window(x) => x.abstract_filling_element_mut(),
        }
    }
}
crate::impl_abstract_filling_element_traits!(FillingElementKindRefMut<'_>);
crate::impl_abstract_filling_element_mut_traits!(FillingElementKindRefMut<'_>);

impl<'a> FillingElementKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Door(x) => x.recompute_bounding_shape(),
            Self::Window(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for FillingElementKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Door(x) => x.feature_type(),
            Self::Window(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_element_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::FillingElementKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::FillingElementKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(FillingElementKind, $type);
    };
}
impl_from_filling_element_kind_ref_mut!(Door);
impl_from_filling_element_kind_ref_mut!(Window);

#[macro_export]
macro_rules! impl_try_from_filling_element_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::FillingElementKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::FillingElementKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::FillingElementKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(FillingElementKind, $type);
    };
}
impl_try_from_filling_element_kind_ref_mut!(Door);
impl_try_from_filling_element_kind_ref_mut!(Window);
impl_try_from_occupied_space_kind_ref_mut_for_enum!(FillingElementKind, FillingElementKindRefMut);
