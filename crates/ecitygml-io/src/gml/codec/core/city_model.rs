use crate::Error;
use crate::gml::codec::core::city_object_property::deserialize_city_object_property;
use crate::gml::codec::core::deserialize_abstract_feature_with_lifespan;
use crate::gml::util::{XmlElement, collect_children, extract_xml_element_spans};
use ecitygml_core::model::core::CityModel;

pub fn deserialize_city_model(xml_document: &[u8]) -> Result<CityModel, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let (abstract_feature_with_lifespan_result, city_object_members_result) = rayon::join(
        || deserialize_abstract_feature_with_lifespan(xml_document, &spans),
        || {
            collect_children(
                xml_document,
                &spans,
                XmlElement::CityObjectMemberProperty,
                deserialize_city_object_property,
            )
        },
    );

    let mut city_model = CityModel::new(abstract_feature_with_lifespan_result?);
    city_model.set_city_object_members(city_object_members_result?);

    Ok(city_model)
}
