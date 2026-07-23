use crate::Error;
use crate::gml::codec::vegetation::abstract_vegetation_object::{
    deserialize_abstract_vegetation_object, serialize_abstract_vegetation_object,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::vegetation::values::{
    PlantCoverClassValue, PlantCoverFunctionValue, PlantCoverUsageValue,
};
use ecitygml_core::model::vegetation::{AsAbstractVegetationObject, PlantCover};
use egml::io::codec::basic::{GmlCode, GmlMeasure};
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_plant_cover(xml_document: &[u8]) -> Result<PlantCover, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_vegetation_object_result, parsed_result) = rayon::join(
        || deserialize_abstract_vegetation_object(xml_document, &spans),
        || de::from_reader::<_, GmlPlantCover>(xml_document).map_err(Error::from),
    );
    let abstract_vegetation_object = abstract_vegetation_object_result?;
    let parsed = parsed_result?;
    let mut plant_cover = PlantCover::from_abstract_vegetation_object(abstract_vegetation_object);

    plant_cover.set_class_opt(parsed.class.map(Code::from).map(PlantCoverClassValue::from));
    plant_cover.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(PlantCoverFunctionValue::from)
            .collect(),
    );
    plant_cover.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(PlantCoverUsageValue::from)
            .collect(),
    );
    plant_cover.set_average_height_opt(parsed.average_height.map(Into::into));
    plant_cover.set_min_height_opt(parsed.min_height.map(Into::into));
    plant_cover.set_max_height_opt(parsed.max_height.map(Into::into));

    Ok(plant_cover)
}

pub fn serialize_plant_cover(
    plant_cover: &PlantCover,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts =
        serialize_abstract_vegetation_object(plant_cover.abstract_vegetation_object(), formatting)?;

    if let Some(raw) = serialize_inner(GmlPlantCover::from(plant_cover), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(CityGmlElement::PlantCover.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPlantCover {
    #[serde(
        rename(serialize = "veg:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "veg:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "veg:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

    #[serde(
        rename(serialize = "veg:averageHeight", deserialize = "averageHeight"),
        skip_serializing_if = "Option::is_none"
    )]
    pub average_height: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:minHeight", deserialize = "minHeight"),
        skip_serializing_if = "Option::is_none"
    )]
    pub min_height: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:maxHeight", deserialize = "maxHeight"),
        skip_serializing_if = "Option::is_none"
    )]
    pub max_height: Option<GmlMeasure>,
}

impl From<&PlantCover> for GmlPlantCover {
    fn from(item: &PlantCover) -> Self {
        Self {
            class: item.class().map(PlantCoverClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(PlantCoverFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(PlantCoverUsageValue::code)
                .map(Into::into)
                .collect(),
            average_height: item.average_height().map(Into::into),
            min_height: item.min_height().map(Into::into),
            max_height: item.max_height().map(Into::into),
        }
    }
}
