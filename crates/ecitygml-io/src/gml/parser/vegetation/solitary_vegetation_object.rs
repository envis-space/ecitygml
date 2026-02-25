use crate::Error;
use crate::gml::parser::core::parse_abstract_occupied_space;
use ecitygml_core::model::vegetation::SolitaryVegetationObject;
use egml::io::GmlMeasure;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_solitary_vegetation_object(
    xml_document: &[u8],
) -> Result<SolitaryVegetationObject, Error> {
    let occupied_space = parse_abstract_occupied_space(xml_document)?;
    let mut solitary_vegetation_object = SolitaryVegetationObject::new(occupied_space);
    let parsed_result: GmlSolitaryVegetationObject = de::from_reader(xml_document)?;

    if let Some(height) = parsed_result.height {
        solitary_vegetation_object.set_height(Some(height.into()));
    }

    if let Some(trunk_diameter) = parsed_result.trunk_diameter {
        solitary_vegetation_object.set_trunk_diameter(Some(trunk_diameter.into()));
    }

    Ok(solitary_vegetation_object)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolitaryVegetationObject {
    #[serde(rename = "height")]
    pub height: Option<GmlMeasure>,

    #[serde(rename = "trunkDiameter")]
    pub trunk_diameter: Option<GmlMeasure>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_solitary_vegetation_object() {
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
            parse_solitary_vegetation_object(xml_document).expect("should work");

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
