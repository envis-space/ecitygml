use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractTunnel_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/tunnel/2.0/_AbstractTunnel_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTunnelUsageValue {
    RailwayTunnel,
    RoadwayTunnel,
    CanalTunnel,
    PedestrianTunnel,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTunnelUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/tunnel/2.0/_AbstractTunnel_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::RailwayTunnel,
            "1010" => Self::RoadwayTunnel,
            "1020" => Self::CanalTunnel,
            "1030" => Self::PedestrianTunnel,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::RailwayTunnel => "1000",
            Self::RoadwayTunnel => "1010",
            Self::CanalTunnel => "1020",
            Self::PedestrianTunnel => "1030",
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
            ("1000", Sig3dTunnelUsageValue::RailwayTunnel),
            ("1030", Sig3dTunnelUsageValue::PedestrianTunnel),
        ] {
            assert_eq!(Sig3dTunnelUsageValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTunnelUsageValue::from_code_value("999999");
        assert_eq!(value, Sig3dTunnelUsageValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }
}
