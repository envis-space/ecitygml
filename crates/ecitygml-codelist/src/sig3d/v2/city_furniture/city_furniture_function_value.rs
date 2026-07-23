use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `CityFurniture_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/cityfurniture/2.0/CityFurniture_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dCityFurnitureFunctionValue {
    CommunicationFixture,
    TelephoneBox,
    Postbox,
    EmergencyCallFixture,
    FireDetector,
    PoliceCallPost,
    SwitchingUnit,
    RoadSign,
    TrafficLight,
    FreeStandingSign,
    FreeStandingWarningSign,
    BusStop,
    Milestone,
    RailLevelCrossing,
    Gate,
    StreetlampLaternOrCandelabra,
    Column,
    LampPost,
    Flagpole,
    StreetSinkBox,
    RubbishBin,
    Clock,
    DirectionalSpotLight,
    FloodlightMast,
    Windmill,
    SolarCell,
    WaterWheel,
    Pole,
    RadioMast,
    Aerial,
    RadioTelescope,
    Chimney,
    Marker,
    Hydrant,
    UpperCorridorFireHydrant,
    LowerFloorPanelFireHydrant,
    SlidegateValveCap,
    EntranceShaft,
    Converter,
    Stair,
    OutsideStaircase,
    Escalator,
    Ramp,
    Patio,
    Fence,
    MemorialMonument,
    WaysideShrine,
    Crossroads,
    CrossOnTheSummitOfAMountain,
    Fountain,
    BlockMark,
    BoundaryPost,
    Bench,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dCityFurnitureFunctionValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/cityfurniture/2.0/CityFurniture_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::CommunicationFixture,
            "1010" => Self::TelephoneBox,
            "1020" => Self::Postbox,
            "1030" => Self::EmergencyCallFixture,
            "1040" => Self::FireDetector,
            "1050" => Self::PoliceCallPost,
            "1060" => Self::SwitchingUnit,
            "1070" => Self::RoadSign,
            "1080" => Self::TrafficLight,
            "1090" => Self::FreeStandingSign,
            "1100" => Self::FreeStandingWarningSign,
            "1110" => Self::BusStop,
            "1120" => Self::Milestone,
            "1130" => Self::RailLevelCrossing,
            "1140" => Self::Gate,
            "1150" => Self::StreetlampLaternOrCandelabra,
            "1160" => Self::Column,
            "1170" => Self::LampPost,
            "1180" => Self::Flagpole,
            "1190" => Self::StreetSinkBox,
            "1200" => Self::RubbishBin,
            "1210" => Self::Clock,
            "1220" => Self::DirectionalSpotLight,
            "1230" => Self::FloodlightMast,
            "1240" => Self::Windmill,
            "1250" => Self::SolarCell,
            "1260" => Self::WaterWheel,
            "1270" => Self::Pole,
            "1280" => Self::RadioMast,
            "1290" => Self::Aerial,
            "1300" => Self::RadioTelescope,
            "1310" => Self::Chimney,
            "1320" => Self::Marker,
            "1330" => Self::Hydrant,
            "1340" => Self::UpperCorridorFireHydrant,
            "1350" => Self::LowerFloorPanelFireHydrant,
            "1360" => Self::SlidegateValveCap,
            "1370" => Self::EntranceShaft,
            "1380" => Self::Converter,
            "1390" => Self::Stair,
            "1400" => Self::OutsideStaircase,
            "1410" => Self::Escalator,
            "1420" => Self::Ramp,
            "1430" => Self::Patio,
            "1440" => Self::Fence,
            "1450" => Self::MemorialMonument,
            "1470" => Self::WaysideShrine,
            "1480" => Self::Crossroads,
            "1490" => Self::CrossOnTheSummitOfAMountain,
            "1500" => Self::Fountain,
            "1510" => Self::BlockMark,
            "1520" => Self::BoundaryPost,
            "1530" => Self::Bench,
            "1540" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::CommunicationFixture => "1000",
            Self::TelephoneBox => "1010",
            Self::Postbox => "1020",
            Self::EmergencyCallFixture => "1030",
            Self::FireDetector => "1040",
            Self::PoliceCallPost => "1050",
            Self::SwitchingUnit => "1060",
            Self::RoadSign => "1070",
            Self::TrafficLight => "1080",
            Self::FreeStandingSign => "1090",
            Self::FreeStandingWarningSign => "1100",
            Self::BusStop => "1110",
            Self::Milestone => "1120",
            Self::RailLevelCrossing => "1130",
            Self::Gate => "1140",
            Self::StreetlampLaternOrCandelabra => "1150",
            Self::Column => "1160",
            Self::LampPost => "1170",
            Self::Flagpole => "1180",
            Self::StreetSinkBox => "1190",
            Self::RubbishBin => "1200",
            Self::Clock => "1210",
            Self::DirectionalSpotLight => "1220",
            Self::FloodlightMast => "1230",
            Self::Windmill => "1240",
            Self::SolarCell => "1250",
            Self::WaterWheel => "1260",
            Self::Pole => "1270",
            Self::RadioMast => "1280",
            Self::Aerial => "1290",
            Self::RadioTelescope => "1300",
            Self::Chimney => "1310",
            Self::Marker => "1320",
            Self::Hydrant => "1330",
            Self::UpperCorridorFireHydrant => "1340",
            Self::LowerFloorPanelFireHydrant => "1350",
            Self::SlidegateValveCap => "1360",
            Self::EntranceShaft => "1370",
            Self::Converter => "1380",
            Self::Stair => "1390",
            Self::OutsideStaircase => "1400",
            Self::Escalator => "1410",
            Self::Ramp => "1420",
            Self::Patio => "1430",
            Self::Fence => "1440",
            Self::MemorialMonument => "1450",
            Self::WaysideShrine => "1470",
            Self::Crossroads => "1480",
            Self::CrossOnTheSummitOfAMountain => "1490",
            Self::Fountain => "1500",
            Self::BlockMark => "1510",
            Self::BoundaryPost => "1520",
            Self::Bench => "1530",
            Self::Others => "1540",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::city_furniture::values::CityFurnitureFunctionValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            (
                "1000",
                Sig3dCityFurnitureFunctionValue::CommunicationFixture,
            ),
            ("1540", Sig3dCityFurnitureFunctionValue::Others),
        ] {
            assert_eq!(
                Sig3dCityFurnitureFunctionValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dCityFurnitureFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dCityFurnitureFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = CityFurnitureFunctionValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dCityFurnitureFunctionValue>(),
            Some(Sig3dCityFurnitureFunctionValue::CommunicationFixture)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = CityFurnitureFunctionValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dCityFurnitureFunctionValue>(), None);
    }
}
