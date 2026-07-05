use crate::Error;
use crate::gml::codec::vegetation::abstract_vegetation_object::{
    deserialize_abstract_vegetation_object, serialize_abstract_vegetation_object,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::vegetation::{AsAbstractVegetationObject, SolitaryVegetationObject};
use egml::io::{GmlCode, GmlMeasure};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_solitary_vegetation_object(
    xml_document: &[u8],
) -> Result<SolitaryVegetationObject, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_vegetation_object_result, parsed_result) = rayon::join(
        || deserialize_abstract_vegetation_object(xml_document, &spans),
        || de::from_reader::<_, GmlSolitaryVegetationObject>(xml_document).map_err(Error::from),
    );
    let abstract_vegetation_object = abstract_vegetation_object_result?;
    let parsed = parsed_result?;
    let mut solitary_vegetation_object =
        SolitaryVegetationObject::from_abstract_vegetation_object(abstract_vegetation_object);

    solitary_vegetation_object.set_class(parsed.class.map(Into::into));
    solitary_vegetation_object
        .set_functions(parsed.functions.into_iter().map(Into::into).collect());
    solitary_vegetation_object.set_usages(parsed.usages.into_iter().map(Into::into).collect());
    solitary_vegetation_object.set_species(parsed.species.map(Into::into));
    solitary_vegetation_object.set_height(parsed.height.map(Into::into));
    solitary_vegetation_object.set_trunk_diameter(parsed.trunk_diameter.map(Into::into));
    solitary_vegetation_object.set_crown_diameter(parsed.crown_diameter.map(Into::into));
    solitary_vegetation_object.set_root_ball_diameter(parsed.root_ball_diameter.map(Into::into));
    solitary_vegetation_object.set_max_root_ball_depth(parsed.max_root_ball_depth.map(Into::into));

    Ok(solitary_vegetation_object)
}

pub fn serialize_solitary_vegetation_object(
    solitary_vegetation_object: &SolitaryVegetationObject,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_vegetation_object(
        solitary_vegetation_object.abstract_vegetation_object(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlSolitaryVegetationObject::from(solitary_vegetation_object),
        formatting,
    )? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::SolitaryVegetationObject, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolitaryVegetationObject {
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
        rename(serialize = "veg:species", deserialize = "species"),
        skip_serializing_if = "Option::is_none"
    )]
    pub species: Option<GmlCode>,

    #[serde(
        rename(serialize = "veg:height", deserialize = "height"),
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:trunkDiameter", deserialize = "trunkDiameter"),
        skip_serializing_if = "Option::is_none"
    )]
    pub trunk_diameter: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:crownDiameter", deserialize = "crownDiameter"),
        skip_serializing_if = "Option::is_none"
    )]
    pub crown_diameter: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:rootBallDiameter", deserialize = "rootBallDiameter"),
        skip_serializing_if = "Option::is_none"
    )]
    pub root_ball_diameter: Option<GmlMeasure>,

    #[serde(
        rename(serialize = "veg:maxRootBallDepth", deserialize = "maxRootBallDepth"),
        skip_serializing_if = "Option::is_none"
    )]
    pub max_root_ball_depth: Option<GmlMeasure>,
}

impl From<&SolitaryVegetationObject> for GmlSolitaryVegetationObject {
    fn from(item: &SolitaryVegetationObject) -> Self {
        Self {
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
            species: item.species().map(Into::into),
            height: item.height().map(Into::into),
            trunk_diameter: item.trunk_diameter().map(Into::into),
            crown_diameter: item.crown_diameter().map(Into::into),
            root_ball_diameter: item.root_ball_diameter().map(Into::into),
            max_root_ball_depth: item.max_root_ball_depth().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_solitary_vegetation_object() {
        let xml_document = b"<veg:SolitaryVegetationObject gml:id=\"UUID_cd516eff-0302-379f-a635-791ebe618098\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>opendrive_roadObject_type</gen:name>
          <gen:value>TREE</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <lod1ImplicitRepresentation>
        <ImplicitGeometry>
          <transformationMatrix>-0.9092656859988072 -0.4162161845304895 0.0 0.0 0.4162161845304895 -0.9092656859988072 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
          <referencePoint>
            <gml:Point>
              <gml:pos srsDimension=\"3\">678133.7625855505 5403776.088934055 367.0738274934606</gml:pos>
            </gml:Point>
          </referencePoint>
        </ImplicitGeometry>
      </lod1ImplicitRepresentation>
      <veg:height uom=\"#m\">12.1966</veg:height>
      <veg:trunkDiameter uom=\"#m\">0.4</veg:trunkDiameter>
    </veg:SolitaryVegetationObject>";

        let solitary_vegetation_object =
            deserialize_solitary_vegetation_object(xml_document).expect("should work");

        assert_eq!(
            solitary_vegetation_object.id(),
            &Id::try_from("UUID_cd516eff-0302-379f-a635-791ebe618098").expect("should work")
        );
        assert!(
            solitary_vegetation_object
                .lod1_implicit_representation()
                .is_some()
        );
        assert_eq!(solitary_vegetation_object.generic_attributes().len(), 1);

        assert_eq!(
            solitary_vegetation_object
                .height()
                .expect("height should be read")
                .value,
            12.1966
        );
        assert_eq!(
            solitary_vegetation_object
                .height()
                .expect("height should be read")
                .uom,
            "#m"
        );
        assert_eq!(
            solitary_vegetation_object
                .trunk_diameter()
                .expect("trunk_diameter should be read")
                .value,
            0.4
        );
        assert_eq!(
            solitary_vegetation_object
                .trunk_diameter()
                .expect("trunk_diameter should be read")
                .uom,
            "#m"
        );
    }
}
