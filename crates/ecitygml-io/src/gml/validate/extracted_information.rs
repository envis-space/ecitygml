use crate::gml::validate::report::{Report, ReportElement, ReportStatistics};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ExtractedInformation {
    pub gml_id_count_per_element_type: HashMap<String, GmlIdCount>,

    pub gml_id_count: HashMap<String, usize>,
    pub city_object_relations: HashSet<CityObjectRelation>,
    pub predecessor_hrefs: HashMap<String, usize>,
    pub successor_hrefs: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct GmlIdCount {
    pub with_gml_id_count: usize,
    pub without_gml_id_count: usize,
}

impl GmlIdCount {
    pub fn increment(&mut self, gml_id_existent: bool) {
        if gml_id_existent {
            self.with_gml_id_count += 1;
        } else {
            self.without_gml_id_count += 1;
        }
    }
}

impl ExtractedInformation {
    pub fn get_gml_id_duplicates(&self) -> Vec<(&String, &usize)> {
        let filtered: Vec<_> = self
            .gml_id_count
            .iter()
            .filter(|&i| *i.1 > 1usize)
            .collect();
        filtered
    }

    pub fn contains_gml_id_duplicates(&self) -> bool {
        self.gml_id_count.values().any(|x| *x > 1usize)
    }

    pub fn find_broken_object_relations(&self) -> Vec<&CityObjectRelation> {
        let broken_object_relations: Vec<&CityObjectRelation> = self
            .city_object_relations
            .iter()
            .filter(|x| {
                !self
                    .gml_id_count
                    .contains_key(&x.related_to.xlink_href.as_str()[1..])
            })
            .collect();
        broken_object_relations
    }

    pub fn find_broken_predecessor_hrefs(&self) -> Vec<String> {
        let broken_hrefs: Vec<String> = self
            .predecessor_hrefs
            .iter()
            .filter(|x| !self.gml_id_count.contains_key(&x.0.as_str()[1..]))
            .map(|x| x.0.clone())
            .collect();
        broken_hrefs
    }

    pub fn find_broken_successor_hrefs(&self) -> Vec<String> {
        let broken_hrefs: Vec<String> = self
            .successor_hrefs
            .iter()
            .filter(|x| !self.gml_id_count.contains_key(&x.0.as_str()[1..]))
            .map(|x| x.0.clone())
            .collect();
        broken_hrefs
    }

    pub fn compile_report(&self) -> Report {
        let gml_id_duplicates: Vec<(String, usize)> = self
            .get_gml_id_duplicates()
            .iter()
            .map(|x| (x.0.clone(), *x.1))
            .collect();

        let broken_object_relations: Vec<String> = self
            .find_broken_object_relations()
            .iter()
            .map(|x| x.related_to.xlink_href.clone())
            .collect();

        let mut xml_elements: Vec<ReportElement> = self
            .gml_id_count_per_element_type
            .iter()
            .map(|x| ReportElement {
                element_name: x.0.clone(),
                number_total: x.1.with_gml_id_count + x.1.without_gml_id_count,
                number_with_gml_id: x.1.with_gml_id_count,
                number_without_gml_id: x.1.without_gml_id_count,
            })
            .collect();
        xml_elements.sort_by_key(|x| x.element_name.clone().to_lowercase());

        let statistics = ReportStatistics {
            number_of_gml_duplicates: gml_id_duplicates.len(),
            number_of_broken_object_relations: broken_object_relations.len(),
        };

        Report {
            statistics,
            xml_elements,
            gml_id_duplicates,
            broken_object_relations,
            broken_predecessor_hrefs: self.find_broken_predecessor_hrefs(),
            broken_successor_hrefs: self.find_broken_successor_hrefs(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct CityObjectRelation {
    #[serde(rename = "relationType")]
    pub related_type: RelatedType,
    #[serde(rename = "relatedTo")]
    pub related_to: RelatedTo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "relationType")]
pub struct RelatedType {
    #[serde(rename = "$value", default)]
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "relatedTo")]
pub struct RelatedTo {
    #[serde(rename = "@href", default)]
    pub xlink_href: String,
}
