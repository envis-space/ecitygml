use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractBridge_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/bridge/2.0/_AbstractBridge_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBridgeFunctionValue {
    RailwayBridge,
    RoadwayBridge,
    CableLink,
    CanalBridge,
    Aqueduct,
    FootBridge,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBridgeFunctionValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/bridge/2.0/_AbstractBridge_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::RailwayBridge,
            "1010" => Self::RoadwayBridge,
            "1030" => Self::CableLink,
            "1040" => Self::CanalBridge,
            "1050" => Self::Aqueduct,
            "1060" => Self::FootBridge,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::RailwayBridge => "1000",
            Self::RoadwayBridge => "1010",
            Self::CableLink => "1030",
            Self::CanalBridge => "1040",
            Self::Aqueduct => "1050",
            Self::FootBridge => "1060",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dBridgeFunctionValue::RailwayBridge),
            ("1060", Sig3dBridgeFunctionValue::FootBridge),
        ] {
            assert_eq!(Sig3dBridgeFunctionValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBridgeFunctionValue::from_code_value("999999");
        assert_eq!(value, Sig3dBridgeFunctionValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }
}
