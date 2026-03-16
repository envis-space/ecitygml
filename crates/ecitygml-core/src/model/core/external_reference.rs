#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalReference {
    pub target_resource: String,
    pub information_system: Option<String>,
    pub relation_type: Option<String>,
}
