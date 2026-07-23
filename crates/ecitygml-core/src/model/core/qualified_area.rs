use crate::model::core::values::QualifiedAreaTypeValue;
use egml::model::measures::Area;

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedArea {
    area: Area,
    type_of_area: QualifiedAreaTypeValue,
}

impl QualifiedArea {
    pub fn new(area: Area, type_of_area: QualifiedAreaTypeValue) -> Self {
        Self { area, type_of_area }
    }

    pub fn area(&self) -> &Area {
        &self.area
    }

    pub fn set_area(&mut self, area: Area) {
        self.area = area;
    }

    pub fn type_of_area(&self) -> &QualifiedAreaTypeValue {
        &self.type_of_area
    }

    pub fn set_type_of_area(&mut self, type_of_area: QualifiedAreaTypeValue) {
        self.type_of_area = type_of_area;
    }
}
