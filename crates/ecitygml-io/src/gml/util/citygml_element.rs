use egml::io::util::XmlElement;
use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum CityGmlElement {
    AbstractAppearanceProperty,
    AbstractBuildingSubdivisionProperty,
    AbstractFillingElementProperty,
    AbstractFillingSurfaceProperty,
    AbstractPointCloudProperty,
    AbstractReliefComponentProperty,
    AbstractSurfaceDataProperty,
    Appearance,
    AppearanceMemberProperty,
    AuxiliaryTrafficArea,
    AuxiliaryTrafficSpace,
    AuxiliaryTrafficSpaceProperty,
    BoundaryProperty,
    Building,
    BuildingConstructiveElement,
    BuildingConstructiveElementProperty,
    BuildingInstallation,
    BuildingInstallationProperty,
    BuildingPart,
    BuildingPartProperty,
    BuildingRoom,
    BuildingRoomProperty,
    BuildingUnit,
    CeilingSurface,
    CityFurniture,
    CityModel,
    CityObjectMemberProperty,
    CityObjectRelation,
    ClearanceSpace,
    ClearanceSpaceProperty,
    ClosureSurface,
    Door,
    DoorSurface,
    FloorSurface,
    GenericAttributeProperty,
    GenericOccupiedSpace,
    GenericThematicSurface,
    GeoreferencedTexture,
    GroundSurface,
    Hole,
    HoleProperty,
    HoleSurface,
    ImplicitGeometry,
    InteriorWallSurface,
    Intersection,
    IntersectionProperty,
    LandUse,
    Lod0MultiCurveProperty,
    Lod0MultiSurfaceProperty,
    Lod1ImplicitRepresentationProperty,
    Lod1MultiCurveProperty,
    Lod1MultiSurfaceProperty,
    Lod1SolidProperty,
    Lod1TerrainIntersectionCurveProperty,
    Lod2ImplicitRepresentationProperty,
    Lod2MultiCurveProperty,
    Lod2MultiSurfaceProperty,
    Lod2SolidProperty,
    Lod2TerrainIntersectionCurveProperty,
    Lod3ImplicitRepresentationProperty,
    Lod3MultiCurveProperty,
    Lod3MultiSurfaceProperty,
    Lod3SolidProperty,
    Lod3TerrainIntersectionCurveProperty,
    Marking,
    MarkingProperty,
    OuterCeilingSurface,
    OuterFloorSurface,
    ParameterizedTexture,
    PlantCover,
    PointCloud,
    PointsProperty,
    QualifiedVolume,
    Railway,
    ReferencePointProperty,
    RelatedToProperty,
    RelativeGeometryProperty,
    ReliefFeature,
    Road,
    RoofSurface,
    Section,
    SectionProperty,
    SolitaryVegetationObject,
    Square,
    Storey,
    TINRelief,
    TargetProperty,
    TexCoordList,
    TinProperty,
    Track,
    TrafficArea,
    TrafficSpace,
    TrafficSpaceProperty,
    WallSurface,
    WaterBody,
    WaterGroundSurface,
    WaterSurface,
    Waterway,
    Window,
    WindowSurface,
    X3DMaterial,
}

impl XmlElement for CityGmlElement {
    fn from_local_name(local_name: &[u8]) -> Option<Self> {
        match local_name {
            b"Appearance" => Some(Self::Appearance),
            b"AuxiliaryTrafficArea" => Some(Self::AuxiliaryTrafficArea),
            b"AuxiliaryTrafficSpace" => Some(Self::AuxiliaryTrafficSpace),
            b"Building" => Some(Self::Building),
            b"BuildingConstructiveElement" => Some(Self::BuildingConstructiveElement),
            b"BuildingInstallation" => Some(Self::BuildingInstallation),
            b"BuildingPart" => Some(Self::BuildingPart),
            b"BuildingRoom" => Some(Self::BuildingRoom),
            b"BuildingUnit" => Some(Self::BuildingUnit),
            b"CeilingSurface" => Some(Self::CeilingSurface),
            b"CityFurniture" => Some(Self::CityFurniture),
            b"CityObjectRelation" => Some(Self::CityObjectRelation),
            b"ClearanceSpace" => Some(Self::ClearanceSpace),
            b"ClosureSurface" => Some(Self::ClosureSurface),
            b"Door" => Some(Self::Door),
            b"DoorSurface" => Some(Self::DoorSurface),
            b"FloorSurface" => Some(Self::FloorSurface),
            b"GenericOccupiedSpace" => Some(Self::GenericOccupiedSpace),
            b"GenericThematicSurface" => Some(Self::GenericThematicSurface),
            b"GeoreferencedTexture" => Some(Self::GeoreferencedTexture),
            b"GroundSurface" => Some(Self::GroundSurface),
            b"Hole" => Some(Self::Hole),
            b"HoleSurface" => Some(Self::HoleSurface),
            b"ImplicitGeometry" => Some(Self::ImplicitGeometry),
            b"InteriorWallSurface" => Some(Self::InteriorWallSurface),
            b"Intersection" => Some(Self::Intersection),
            b"LandUse" => Some(Self::LandUse),
            b"Marking" => Some(Self::Marking),
            b"OuterCeilingSurface" => Some(Self::OuterCeilingSurface),
            b"OuterFloorSurface" => Some(Self::OuterFloorSurface),
            b"ParameterizedTexture" => Some(Self::ParameterizedTexture),
            b"PlantCover" => Some(Self::PlantCover),
            b"PointCloud" => Some(Self::PointCloud),
            b"QualifiedVolume" => Some(Self::QualifiedVolume),
            b"Railway" => Some(Self::Railway),
            b"ReliefFeature" => Some(Self::ReliefFeature),
            b"Road" => Some(Self::Road),
            b"RoofSurface" => Some(Self::RoofSurface),
            b"Section" => Some(Self::Section),
            b"SolitaryVegetationObject" => Some(Self::SolitaryVegetationObject),
            b"Square" => Some(Self::Square),
            b"Storey" => Some(Self::Storey),
            b"TINRelief" => Some(Self::TINRelief),
            b"target" => Some(Self::TargetProperty),
            b"TexCoordList" => Some(Self::TexCoordList),
            b"Track" => Some(Self::Track),
            b"TrafficArea" => Some(Self::TrafficArea),
            b"TrafficSpace" => Some(Self::TrafficSpace),
            b"WallSurface" => Some(Self::WallSurface),
            b"WaterBody" => Some(Self::WaterBody),
            b"WaterGroundSurface" => Some(Self::WaterGroundSurface),
            b"WaterSurface" => Some(Self::WaterSurface),
            b"Waterway" => Some(Self::Waterway),
            b"Window" => Some(Self::Window),
            b"WindowSurface" => Some(Self::WindowSurface),
            b"X3DMaterial" => Some(Self::X3DMaterial),
            b"appearance" => Some(Self::AbstractAppearanceProperty),
            b"appearanceMember" => Some(Self::AppearanceMemberProperty),
            b"auxiliaryTrafficSpace" => Some(Self::AuxiliaryTrafficSpaceProperty),
            b"boundary" => Some(Self::BoundaryProperty),
            b"buildingConstructiveElement" => Some(Self::BuildingConstructiveElementProperty),
            b"buildingInstallation" => Some(Self::BuildingInstallationProperty),
            b"buildingPart" => Some(Self::BuildingPartProperty),
            b"buildingRoom" => Some(Self::BuildingRoomProperty),
            b"buildingSubdivision" => Some(Self::AbstractBuildingSubdivisionProperty),
            b"cityObjectMember" => Some(Self::CityObjectMemberProperty),
            b"clearanceSpace" => Some(Self::ClearanceSpaceProperty),
            b"filling" => Some(Self::AbstractFillingElementProperty),
            b"fillingSurface" => Some(Self::AbstractFillingSurfaceProperty),
            b"genericAttribute" => Some(Self::GenericAttributeProperty),
            b"hole" => Some(Self::HoleProperty),
            b"intersection" => Some(Self::IntersectionProperty),
            b"lod0MultiCurve" => Some(Self::Lod0MultiCurveProperty),
            b"lod0MultiSurface" => Some(Self::Lod0MultiSurfaceProperty),
            b"lod1ImplicitRepresentation" => Some(Self::Lod1ImplicitRepresentationProperty),
            b"lod1MultiCurve" => Some(Self::Lod1MultiCurveProperty),
            b"lod1MultiSurface" => Some(Self::Lod1MultiSurfaceProperty),
            b"lod1Solid" => Some(Self::Lod1SolidProperty),
            b"lod1TerrainIntersectionCurveProperty" => {
                Some(Self::Lod1TerrainIntersectionCurveProperty)
            }
            b"lod2ImplicitRepresentation" => Some(Self::Lod2ImplicitRepresentationProperty),
            b"lod2MultiCurve" => Some(Self::Lod2MultiCurveProperty),
            b"lod2MultiSurface" => Some(Self::Lod2MultiSurfaceProperty),
            b"lod2Solid" => Some(Self::Lod2SolidProperty),
            b"lod2TerrainIntersectionCurveProperty" => {
                Some(Self::Lod2TerrainIntersectionCurveProperty)
            }
            b"lod3ImplicitRepresentation" => Some(Self::Lod3ImplicitRepresentationProperty),
            b"lod3MultiCurve" => Some(Self::Lod3MultiCurveProperty),
            b"lod3MultiSurface" => Some(Self::Lod3MultiSurfaceProperty),
            b"lod3Solid" => Some(Self::Lod3SolidProperty),
            b"lod3TerrainIntersectionCurveProperty" => {
                Some(Self::Lod3TerrainIntersectionCurveProperty)
            }
            b"marking" => Some(Self::MarkingProperty),
            b"pointCloud" => Some(Self::AbstractPointCloudProperty),
            b"points" => Some(Self::PointsProperty),
            b"referencePoint" => Some(Self::ReferencePointProperty),
            b"relativeGeometry" => Some(Self::RelativeGeometryProperty),
            b"reliefComponent" => Some(Self::AbstractReliefComponentProperty),
            b"section" => Some(Self::SectionProperty),
            b"surfaceData" => Some(Self::AbstractSurfaceDataProperty),
            b"tin" => Some(Self::TinProperty),
            b"trafficSpace" => Some(Self::TrafficSpaceProperty),
            b"relatedTo" => Some(Self::RelatedToProperty),
            _ => {
                tracing::debug!(
                    "unknown XML element: {}",
                    String::from_utf8_lossy(local_name)
                );
                None
            }
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            CityGmlElement::AbstractAppearanceProperty => "app:appearance",
            CityGmlElement::AbstractBuildingSubdivisionProperty => "bldg:buildingSubdivision",
            CityGmlElement::AbstractFillingElementProperty => "filling",
            CityGmlElement::AbstractFillingSurfaceProperty => "fillingSurface",
            CityGmlElement::AbstractPointCloudProperty => "pointCloud",
            CityGmlElement::AbstractReliefComponentProperty => "dem:reliefComponent",
            CityGmlElement::AbstractSurfaceDataProperty => "app:surfaceData",
            CityGmlElement::Appearance => "app:Appearance",
            CityGmlElement::AppearanceMemberProperty => "appearanceMember",
            CityGmlElement::AuxiliaryTrafficArea => "tran:AuxiliaryTrafficArea",
            CityGmlElement::AuxiliaryTrafficSpace => "tran:AuxiliaryTrafficSpace",
            CityGmlElement::AuxiliaryTrafficSpaceProperty => "tran:auxiliaryTrafficSpace",
            CityGmlElement::BoundaryProperty => "boundary",
            CityGmlElement::Building => "bldg:Building",
            CityGmlElement::BuildingConstructiveElement => "bldg:BuildingConstructiveElement",
            CityGmlElement::BuildingConstructiveElementProperty => {
                "bldg:buildingConstructiveElement"
            }
            CityGmlElement::BuildingInstallation => "bldg:BuildingInstallation",
            CityGmlElement::BuildingInstallationProperty => "bldg:buildingInstallation",
            CityGmlElement::BuildingPart => "bldg:BuildingPart",
            CityGmlElement::BuildingPartProperty => "bldg:buildingPart",
            CityGmlElement::BuildingRoom => "bldg:BuildingRoom",
            CityGmlElement::BuildingRoomProperty => "bldg:buildingRoom",
            CityGmlElement::BuildingUnit => "bldg:BuildingUnit",
            CityGmlElement::CeilingSurface => "con:CeilingSurface",
            CityGmlElement::CityFurniture => "frn:CityFurniture",
            CityGmlElement::CityModel => "CityModel",
            CityGmlElement::CityObjectMemberProperty => "cityObjectMember",
            CityGmlElement::CityObjectRelation => "CityObjectRelation",
            CityGmlElement::ClearanceSpace => "tran:ClearanceSpace",
            CityGmlElement::ClearanceSpaceProperty => "tran:clearanceSpace",
            CityGmlElement::ClosureSurface => "ClosureSurface",
            CityGmlElement::Door => "con:Door",
            CityGmlElement::DoorSurface => "con:DoorSurface",
            CityGmlElement::FloorSurface => "con:FloorSurface",
            CityGmlElement::GenericAttributeProperty => "genericAttribute",
            CityGmlElement::GenericOccupiedSpace => "gen:GenericOccupiedSpace",
            CityGmlElement::GenericThematicSurface => "gen:GenericThematicSurface",
            CityGmlElement::GeoreferencedTexture => "app:GeoreferencedTexture",
            CityGmlElement::GroundSurface => "con:GroundSurface",
            CityGmlElement::Hole => "tran:Hole",
            CityGmlElement::HoleProperty => "tran:hole",
            CityGmlElement::HoleSurface => "tran:HoleSurface",
            CityGmlElement::ImplicitGeometry => "ImplicitGeometry",
            CityGmlElement::InteriorWallSurface => "con:InteriorWallSurface",
            CityGmlElement::Intersection => "tran:Intersection",
            CityGmlElement::IntersectionProperty => "tran:intersection",
            CityGmlElement::LandUse => "luse:LandUse",
            CityGmlElement::Lod0MultiCurveProperty => "lod0MultiCurve",
            CityGmlElement::Lod0MultiSurfaceProperty => "lod0MultiSurface",
            CityGmlElement::Lod1ImplicitRepresentationProperty => "lod1ImplicitRepresentation",
            CityGmlElement::Lod1MultiCurveProperty => "lod1MultiCurve",
            CityGmlElement::Lod1MultiSurfaceProperty => "lod1MultiSurface",
            CityGmlElement::Lod1SolidProperty => "lod1Solid",
            CityGmlElement::Lod1TerrainIntersectionCurveProperty => {
                "lod1TerrainIntersectionCurveProperty"
            }
            CityGmlElement::Lod2ImplicitRepresentationProperty => "lod2ImplicitRepresentation",
            CityGmlElement::Lod2MultiCurveProperty => "lod2MultiCurve",
            CityGmlElement::Lod2MultiSurfaceProperty => "lod2MultiSurface",
            CityGmlElement::Lod2SolidProperty => "lod2Solid",
            CityGmlElement::Lod2TerrainIntersectionCurveProperty => {
                "lod2TerrainIntersectionCurveProperty"
            }
            CityGmlElement::Lod3ImplicitRepresentationProperty => "lod3ImplicitRepresentation",
            CityGmlElement::Lod3MultiCurveProperty => "lod3MultiCurve",
            CityGmlElement::Lod3MultiSurfaceProperty => "lod3MultiSurface",
            CityGmlElement::Lod3SolidProperty => "lod3Solid",
            CityGmlElement::Lod3TerrainIntersectionCurveProperty => {
                "lod3TerrainIntersectionCurveProperty"
            }
            CityGmlElement::Marking => "tran:Marking",
            CityGmlElement::MarkingProperty => "tran:marking",
            CityGmlElement::OuterCeilingSurface => "con:OuterCeilingSurface",
            CityGmlElement::OuterFloorSurface => "con:OuterFloorSurface",
            CityGmlElement::ParameterizedTexture => "app:ParameterizedTexture",
            CityGmlElement::PlantCover => "veg:PlantCover",
            CityGmlElement::PointCloud => "pcl:PointCloud",
            CityGmlElement::PointsProperty => "pcl:points",
            CityGmlElement::QualifiedVolume => "QualifiedVolume",
            CityGmlElement::Railway => "tran:Railway",
            CityGmlElement::ReferencePointProperty => "referencePoint",
            CityGmlElement::RelatedToProperty => "relatedTo",
            CityGmlElement::RelativeGeometryProperty => "relativeGeometry",
            CityGmlElement::ReliefFeature => "dem:ReliefFeature",
            CityGmlElement::Road => "tran:Road",
            CityGmlElement::RoofSurface => "con:RoofSurface",
            CityGmlElement::Section => "tran:Section",
            CityGmlElement::SectionProperty => "tran:section",
            CityGmlElement::SolitaryVegetationObject => "veg:SolitaryVegetationObject",
            CityGmlElement::Square => "tran:Square",
            CityGmlElement::Storey => "bldg:Storey",
            CityGmlElement::TINRelief => "dem:TINRelief",
            CityGmlElement::TargetProperty => "app:target",
            CityGmlElement::TexCoordList => "app:TexCoordList",
            CityGmlElement::TinProperty => "dem:tin",
            CityGmlElement::Track => "tran:Track",
            CityGmlElement::TrafficArea => "tran:TrafficArea",
            CityGmlElement::TrafficSpace => "tran:TrafficSpace",
            CityGmlElement::TrafficSpaceProperty => "tran:trafficSpace",
            CityGmlElement::WallSurface => "con:WallSurface",
            CityGmlElement::WaterBody => "wtr:WaterBody",
            CityGmlElement::WaterGroundSurface => "wtr:WaterGroundSurface",
            CityGmlElement::WaterSurface => "wtr:WaterSurface",
            CityGmlElement::Waterway => "tran:Waterway",
            CityGmlElement::Window => "con:Window",
            CityGmlElement::WindowSurface => "con:WindowSurface",
            CityGmlElement::X3DMaterial => "app:X3DMaterial",
        }
    }
}

impl From<CityGmlElement> for &'static str {
    fn from(item: CityGmlElement) -> Self {
        item.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml::io::util::extract_xml_element_spans;

    #[test]
    fn test_deserialize_basic_abstract_thematic_surface() {
        let xml_document = b"<bldg:Building gml:id=\"DEBY_LOD2_59772\">
      <creationDate >2023-06-05T00:00:00+02:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTy70E</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691478.01 5334825.81 516.71 691478.01 5334825.81 528.425 691473.38 5334813.05 528.425 691473.38 5334813.05 516.71 691478.01 5334825.81 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691474.24 5334826.79 516.71 691474.24 5334826.79 528.425 691475.29 5334827.01 528.425 691475.29 5334827.01 516.71 691474.24 5334826.79 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691472.46 5334820.87 516.71 691472.46 5334820.87 528.425 691472.6 5334821.26 528.425 691472.6 5334821.26 516.71 691472.46 5334820.87 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:RoofSurface gml:id=\"DEBY_LOD2_59772_a13e523a-7269-4637-88fa-e57eed6d9265_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_a13e523a-7269-4637-88fa-e57eed6d9265_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.72 5334813.99 528.425 691473.38 5334813.05 528.425 691478.01 5334825.81 528.425 691475.29 5334827.01 528.425 691474.24 5334826.79 528.425 691472.6 5334821.26 528.425 691472.46 5334820.87 528.425 691470.53 5334815.81 528.425 691470.29 5334815.2 528.425 691470.72 5334813.99 528.425</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:RoofSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_bacdfeda-2181-42c2-ac94-bf086ec95291_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_bacdfeda-2181-42c2-ac94-bf086ec95291_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.72 5334813.99 516.71 691470.72 5334813.99 528.425 691470.29 5334815.2 528.425 691470.29 5334815.2 516.71 691470.72 5334813.99 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_e76604b3-3834-4420-a1e5-c660f32ab045_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_e76604b3-3834-4420-a1e5-c660f32ab045_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.29 5334815.2 516.71 691470.29 5334815.2 528.425 691470.53 5334815.81 528.425 691470.53 5334815.81 516.71 691470.29 5334815.2 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c8be8650-e40d-4c7d-b491-d892085763aa_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c8be8650-e40d-4c7d-b491-d892085763aa_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691473.38 5334813.05 516.71 691473.38 5334813.05 528.425 691470.72 5334813.99 528.425 691470.72 5334813.99 516.71 691473.38 5334813.05 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_30ce7949-9c18-4a98-bdae-afeb9a0b6252_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_30ce7949-9c18-4a98-bdae-afeb9a0b6252_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691472.6 5334821.26 516.71 691472.6 5334821.26 528.425 691474.24 5334826.79 528.425 691474.24 5334826.79 516.71 691472.6 5334821.26 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_27c754d1-d65b-4b5e-a17a-8a24080009e1_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_27c754d1-d65b-4b5e-a17a-8a24080009e1_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691475.29 5334827.01 516.71 691475.29 5334827.01 528.425 691478.01 5334825.81 528.425 691478.01 5334825.81 516.71 691475.29 5334827.01 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:GroundSurface gml:id=\"DEBY_LOD2_59772_bae63dbe-80b8-4e3a-a78e-755230366512_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_bae63dbe-80b8-4e3a-a78e-755230366512_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.29 5334815.2 516.71 691470.53 5334815.81 516.71 691472.46 5334820.87 516.71 691472.6 5334821.26 516.71 691474.24 5334826.79 516.71 691475.29 5334827.01 516.71 691478.01 5334825.81 516.71 691473.38 5334813.05 516.71 691470.72 5334813.99 516.71 691470.29 5334815.2 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:GroundSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c21df86c-d952-4aa9-a5e2-42418b07a2a9_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c21df86c-d952-4aa9-a5e2-42418b07a2a9_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.53 5334815.81 516.71 691470.53 5334815.81 528.425 691472.46 5334820.87 528.425 691472.46 5334820.87 516.71 691470.53 5334815.81 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <bldg:function>31001_9998</bldg:function>
      <bldg:roofType>1000</bldg:roofType>
      <bldg:storeysAboveGround>3</bldg:storeysAboveGround>
    </bldg:Building>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        let boundary_spans = xml_element_spans.get(CityGmlElement::BoundaryProperty);
        assert_eq!(boundary_spans.len(), 11);

        /*for (gml_element, spans) in gml_element_spans {
            for current_span in spans {
                let snippet = &xml_document[current_span.start..current_span.end];
                let a = String::from_utf8_lossy(snippet);

                println!("abc: {:?}", a);
            }
        }*/
    }

    #[test]
    fn test_deserialize_basic() {
        let xml_document = b"<bldg:Building gml:id=\"DEBY_LOD2_59772\">
      <creationDate >2023-06-05T00:00:00+02:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTy70E</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>
    </bldg:Building>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        let boundary_span = xml_element_spans
            .first(CityGmlElement::BoundaryProperty)
            .unwrap();
        let xml_snippet = &xml_document[boundary_span.start..boundary_span.end];
        let xml_snippet_utf = String::from_utf8_lossy(xml_snippet);

        assert_eq!(
            xml_snippet_utf,
            "<boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>"
        );
    }

    #[test]
    fn test_deserialize_with_tw() {
        let xml_document = b"<cityObjectMember>
        <bldg:Building gml:id=\"DEBY_LOD2_59772\">
        <creationDate >2023-06-05T00:00:00+02:00</creationDate>
        <boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>
    </bldg:Building>
    </cityObjectMember>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        assert_eq!(
            xml_element_spans
                .get(CityGmlElement::BoundaryProperty)
                .len(),
            0
        );
    }
}
