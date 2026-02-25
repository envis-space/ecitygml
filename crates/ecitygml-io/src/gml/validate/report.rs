use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub(crate) statistics: ReportStatistics,
    pub(crate) xml_elements: Vec<ReportElement>,
    pub(crate) gml_id_duplicates: Vec<(String, usize)>,
    pub(crate) broken_object_relations: Vec<String>,
    pub(crate) broken_predecessor_hrefs: Vec<String>,
    pub(crate) broken_successor_hrefs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatistics {
    pub(crate) number_of_gml_duplicates: usize,
    pub(crate) number_of_broken_object_relations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportElement {
    pub(crate) element_name: String,
    pub(crate) number_total: usize,
    pub(crate) number_with_gml_id: usize,
    pub(crate) number_without_gml_id: usize,
}

impl Report {
    pub fn write(&self, path: impl AsRef<Path>) {
        let yaml = serde_yaml::to_string(&self).unwrap();
        fs::write(path, yaml).expect("Unable to write file");
    }
}
