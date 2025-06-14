use strum_macros::EnumIter;

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, EnumIter)]
pub enum LevelOfDetail {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum CityObjectClass {
    AuxiliaryTrafficArea,
    AuxiliaryTrafficSpace,
    Bridge,
    BridgeConstructiveElement,
    BridgeFurniture,
    BridgeInstallation,
    BridgePart,
    BridgeRoom,
    Building,
    BuildingConstructiveElement,
    BuildingFurniture,
    BuildingInstallation,
    BuildingPart,
    BuildingRoom,
    BuildingUnit,
    CeilingSurface,
    CityFurniture,
    CityObjectGroup,
    ClearanceSpace,
    Door,
    DoorSurface,
    FloorSurface,
    GenericLogicalSpace,
    GenericOccupiedSpace,
    GenericThematicSurface,
    GenericUnoccupiedSpace,
    GroundSurface,
    Hole,
    HoleSurface,
    HollowSpace,
    InteriorWallSurface,
    Intersection,
    Marking,
    OtherConstruction,
    OuterCeilingSurface,
    OuterFloorSurface,
    PlantCover,
    Railway,
    Road,
    RoofSurface,
    Section,
    SolitaryVegetationObject,
    Square,
    Story,
    Track,
    TrafficArea,
    TrafficSpace,
    Tunnel,
    TunnelConstructiveElement,
    TunnelFurniture,
    TunnelInstallation,
    TunnelPart,
    WallSurface,
    WaterBody,
    WaterGroundSurface,
    WaterSurface,
    Waterway,
    Window,
    WindowSurface,
}
