use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractBridge_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/bridge/2.0/_AbstractBridge_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBridgeClassValue {
    ArcedBridge,
    CableStayedBridge,
    DeckBridge,
    CableStayedOverpass,
    TrussBridge,
    PontoonBridge,
    SuspensionBridge,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBridgeClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/bridge/2.0/_AbstractBridge_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::ArcedBridge,
            "1010" => Self::CableStayedBridge,
            "1020" => Self::DeckBridge,
            "1030" => Self::CableStayedOverpass,
            "1040" => Self::TrussBridge,
            "1050" => Self::PontoonBridge,
            "1060" => Self::SuspensionBridge,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::ArcedBridge => "1000",
            Self::CableStayedBridge => "1010",
            Self::DeckBridge => "1020",
            Self::CableStayedOverpass => "1030",
            Self::TrussBridge => "1040",
            Self::PontoonBridge => "1050",
            Self::SuspensionBridge => "1060",
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
            ("1000", Sig3dBridgeClassValue::ArcedBridge),
            ("1060", Sig3dBridgeClassValue::SuspensionBridge),
        ] {
            assert_eq!(Sig3dBridgeClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBridgeClassValue::from_code_value("999999");
        assert_eq!(value, Sig3dBridgeClassValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }
}
