use crate::Error;
use crate::gml::util::CityGmlElement;
use egml::io::util::{GmlElement, XmlElement, XmlElementSpans, extract_xml_element_spans};
use std::collections::HashMap;
use std::ops::Range;
use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombinedCityGmlElement {
    CityGml(CityGmlElement),
    Gml(GmlElement),
}

impl XmlElement for CombinedCityGmlElement {
    fn from_local_name(local_name: &[u8]) -> Option<Self> {
        CityGmlElement::from_local_name(local_name)
            .map(Self::CityGml)
            .or_else(|| GmlElement::from_local_name(local_name).map(Self::Gml))
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::CityGml(e) => e.as_str(),
            Self::Gml(e) => e.as_str(),
        }
    }
}

impl From<CityGmlElement> for CombinedCityGmlElement {
    fn from(e: CityGmlElement) -> Self {
        Self::CityGml(e)
    }
}

impl From<GmlElement> for CombinedCityGmlElement {
    fn from(e: GmlElement) -> Self {
        Self::Gml(e)
    }
}

pub fn collect_gml_child<T>(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
    element: CombinedCityGmlElement,
    deserializer: fn(&[u8], &XmlElementSpans<GmlElement>) -> Result<T, egml::io::Error>,
) -> Result<Option<T>, Error> {
    let all_spans = spans.get(element);
    match all_spans.first() {
        None => Ok(None),
        Some(span) => {
            let slice = &xml_document[span.start..span.end];
            let child_spans: XmlElementSpans<CombinedCityGmlElement> =
                extract_xml_element_spans(slice).map_err(Error::from)?;
            let gml_spans = filter_to_gml_element_spans(&child_spans);
            deserializer(slice, &gml_spans)
                .map(Some)
                .map_err(Error::from)
        }
    }
}

/// Like [`collect_gml_child`], but never fails: a child element that is present yet fails to
/// deserialize (e.g. a `gml:MultiSurface` with zero `surfaceMember`s, invalid per OGC 07-036
/// §11.3.4.1) is dropped and logged instead of failing the feature that carries it. Use this for
/// optional LOD geometry representations, where losing one representation shouldn't sink the
/// whole feature; keep [`collect_gml_child`] for properties the feature can't meaningfully exist
/// without.
pub fn collect_gml_child_lenient<T>(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
    element: CombinedCityGmlElement,
    deserializer: fn(&[u8], &XmlElementSpans<GmlElement>) -> Result<T, egml::io::Error>,
) -> Option<T> {
    match collect_gml_child(xml_document, spans, element, deserializer) {
        Ok(value) => value,
        Err(error) => {
            warn!(?element, ?error, "dropping invalid geometry property");
            None
        }
    }
}

pub fn filter_to_gml_element_spans(
    combined: &XmlElementSpans<CombinedCityGmlElement>,
) -> XmlElementSpans<GmlElement> {
    let spans: HashMap<GmlElement, Vec<Range<usize>>> = combined
        .spans()
        .iter()
        .filter_map(|(k, v)| match k {
            CombinedCityGmlElement::Gml(e) => Some((*e, v.clone())),
            CombinedCityGmlElement::CityGml(_) => None,
        })
        .collect();
    XmlElementSpans::new(spans)
}
