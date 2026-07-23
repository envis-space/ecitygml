use crate::Error;
use crate::gml::codec::core::abstract_appearance_property::{
    deserialize_abstract_appearance_property, serialize_abstract_appearance_member_property,
};
use crate::gml::codec::core::abstract_city_object_property::{
    deserialize_abstract_city_object_property, serialize_abstract_city_object_member_property,
};
use crate::gml::codec::core::{
    deserialize_abstract_feature_with_lifespan, serialize_abstract_feature_with_lifespan,
};
use crate::gml::util::{CityGmlElement, deserializer_pool, get_namespace_pairs};
use ecitygml_core::model::core::{AsAbstractFeatureWithLifespan, CityModel};
use egml::io::util::{Formatting, extract_xml_element_spans};
use egml::io::util::{XmlElement, collect_children};
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::io::Write;

pub fn deserialize_city_model(xml_document: &[u8]) -> Result<CityModel, Error> {
    deserializer_pool().install(|| {
        let spans = extract_xml_element_spans(xml_document)?;

        let mut abstract_feature_with_lifespan_result = None;
        let mut city_object_members_result = None;
        let mut appearance_members_result = None;

        rayon::scope(|s| {
            s.spawn(|_| {
                abstract_feature_with_lifespan_result = Some(
                    deserialize_abstract_feature_with_lifespan(xml_document, &spans),
                );
            });
            s.spawn(|_| {
                city_object_members_result = Some(collect_children(
                    xml_document,
                    &spans,
                    CityGmlElement::CityObjectMemberProperty.into(),
                    deserialize_abstract_city_object_property,
                ));
            });
            s.spawn(|_| {
                appearance_members_result = Some(collect_children(
                    xml_document,
                    &spans,
                    CityGmlElement::AppearanceMemberProperty.into(),
                    deserialize_abstract_appearance_property,
                ));
            });
        });

        let abstract_feature_with_lifespan = abstract_feature_with_lifespan_result
            .expect("rayon::scope guarantees all spawns complete")?;
        let city_object_members =
            city_object_members_result.expect("rayon::scope guarantees all spawns complete")?;
        let appearance_members =
            appearance_members_result.expect("rayon::scope guarantees all spawns complete")?;

        let mut city_model =
            CityModel::from_abstract_feature_with_lifespan(abstract_feature_with_lifespan);
        city_model.set_city_object_members(city_object_members);
        city_model.set_appearance_members(appearance_members);

        Ok(city_model)
    })
}

pub fn serialize_city_model<W: Write>(
    xml_writer: &mut Writer<W>,
    city_model: CityModel,
    formatting: Formatting,
) -> Result<(), Error> {
    let mut xml_node_parts = serialize_abstract_feature_with_lifespan(
        city_model.abstract_feature_with_lifespan(),
        formatting,
    )?;
    xml_node_parts.attributes.extend(get_namespace_pairs());

    let elem = BytesStart::new(CityGmlElement::CityModel.as_str()).with_attributes(
        xml_node_parts
            .attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str())),
    );
    xml_writer.write_event(Event::Start(elem))?;
    xml_node_parts.write_contents_to_at_depth(xml_writer, formatting, 1)?;

    let xml_nodes = city_model
        .city_object_members()
        .par_iter()
        .map(|x| serialize_abstract_city_object_member_property(x, formatting))
        .collect::<Result<Vec<_>, _>>()?;
    for current_xml_node in xml_nodes {
        current_xml_node.write_to_at_depth(xml_writer, formatting, 1)?;
    }

    let xml_nodes = city_model
        .appearance_members()
        .par_iter()
        .map(|x| serialize_abstract_appearance_member_property(x, formatting))
        .collect::<Result<Vec<_>, _>>()?;
    for current_xml_node in xml_nodes {
        current_xml_node.write_to_at_depth(xml_writer, formatting, 1)?;
    }

    xml_writer.write_event(Event::End(BytesEnd::new(
        CityGmlElement::CityModel.as_str(),
    )))?;

    Ok(())
}
