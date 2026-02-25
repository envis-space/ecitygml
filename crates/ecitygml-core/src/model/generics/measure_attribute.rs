use egml::model::basic::Measure;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MeasureAttribute {
    pub name: String,
    pub value: Measure,
}
