use crate::model::generics::generic_attribute_set::GenericAttributeSet;
use crate::model::generics::string_attribute::StringAttribute;
use crate::model::generics::{DoubleAttribute, IntAttribute, MeasureAttribute};

#[derive(Debug, Clone, PartialEq)]
pub enum GenericAttributeKind {
    StringAttribute(StringAttribute),
    IntAttribute(IntAttribute),
    DoubleAttribute(DoubleAttribute),
    MeasureAttribute(MeasureAttribute),
    GenericAttributeSet(GenericAttributeSet),
}

impl GenericAttributeKind {
    pub fn name(&self) -> &str {
        match self {
            GenericAttributeKind::StringAttribute(x) => &x.name,
            GenericAttributeKind::IntAttribute(x) => &x.name,
            GenericAttributeKind::DoubleAttribute(x) => &x.name,
            GenericAttributeKind::MeasureAttribute(x) => &x.name,
            GenericAttributeKind::GenericAttributeSet(x) => &x.name,
        }
    }

    pub fn as_string(&self) -> Option<&StringAttribute> {
        if let GenericAttributeKind::StringAttribute(attr) = self {
            Some(attr)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<&IntAttribute> {
        if let GenericAttributeKind::IntAttribute(attr) = self {
            Some(attr)
        } else {
            None
        }
    }

    pub fn as_double(&self) -> Option<&DoubleAttribute> {
        if let GenericAttributeKind::DoubleAttribute(attr) = self {
            Some(attr)
        } else {
            None
        }
    }
}

impl From<StringAttribute> for GenericAttributeKind {
    fn from(attr: StringAttribute) -> Self {
        GenericAttributeKind::StringAttribute(attr)
    }
}

impl From<IntAttribute> for GenericAttributeKind {
    fn from(attr: IntAttribute) -> Self {
        GenericAttributeKind::IntAttribute(attr)
    }
}

impl From<DoubleAttribute> for GenericAttributeKind {
    fn from(attr: DoubleAttribute) -> Self {
        GenericAttributeKind::DoubleAttribute(attr)
    }
}

impl From<MeasureAttribute> for GenericAttributeKind {
    fn from(attr: MeasureAttribute) -> Self {
        GenericAttributeKind::MeasureAttribute(attr)
    }
}

impl From<GenericAttributeSet> for GenericAttributeKind {
    fn from(attr: GenericAttributeSet) -> Self {
        GenericAttributeKind::GenericAttributeSet(attr)
    }
}
