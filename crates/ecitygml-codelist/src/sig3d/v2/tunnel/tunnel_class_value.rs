use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractTunnel_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/tunnel/2.0/_AbstractTunnel_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTunnelClassValue {
    Traffic,
    Supply,
    Historical,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTunnelClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/tunnel/2.0/_AbstractTunnel_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Traffic,
            "1010" => Self::Supply,
            "1020" => Self::Historical,
            "1030" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Traffic => "1000",
            Self::Supply => "1010",
            Self::Historical => "1020",
            Self::Others => "1030",
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
            ("1000", Sig3dTunnelClassValue::Traffic),
            ("1030", Sig3dTunnelClassValue::Others),
        ] {
            assert_eq!(Sig3dTunnelClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTunnelClassValue::from_code_value("999999");
        assert_eq!(value, Sig3dTunnelClassValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }
}
