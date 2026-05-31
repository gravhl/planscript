use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitType {
    M,
    Meters,
    Cm,
    Mm,
    Ft,
    In,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AxisDirection {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Hemisphere {
    North,
    South,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitsDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub unit: UnitType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub point: Point,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub x: AxisDirection,
    pub y: AxisDirection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub size: f64,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultsDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdoor_floor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderMode {
    Color,
    Draft,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub mode: RenderMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DimensionRoomSelection {
    None,
    All,
}

impl Default for DimensionRoomSelection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DimensionWallTarget {
    WallId { id: String },
    RoomEdge { room: String, edge: EdgeSide },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DimensionWallSelection {
    None,
    All,
    Only { targets: Vec<DimensionWallTarget> },
}

impl Default for DimensionWallSelection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DimensionFixtureSelection {
    None,
    All,
    Only { names: Vec<String> },
}

impl Default for DimensionFixtureSelection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DimensionUnitSystem {
    Metric,
    Standard,
}

impl Default for DimensionUnitSystem {
    fn default() -> Self {
        Self::Metric
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionUnitsDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub units: DimensionUnitSystem,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_system: Option<DimensionUnitSystem>,
    #[serde(default)]
    pub rooms: DimensionRoomSelection,
    #[serde(default)]
    pub footprint: bool,
    #[serde(default)]
    pub walls: DimensionWallSelection,
    #[serde(default)]
    pub fixtures: DimensionFixtureSelection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub street: CardinalDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hemisphere: Option<Hemisphere>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Footprint {
    FootprintPolygon { points: Vec<Point> },
    FootprintRect { p1: Point, p2: Point },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionValue {
    Number(f64),
    Auto(String),
}

impl DimensionValue {
    pub fn auto() -> Self {
        Self::Auto("auto".to_string())
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Auto(_) => None,
        }
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto(_))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeValue {
    pub x: DimensionValue,
    pub y: DimensionValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RoomGeometry {
    RoomPolygon {
        points: Vec<Point>,
    },
    RoomRectDiagonal {
        p1: Point,
        p2: Point,
    },
    RoomRectAtSize {
        at: Point,
        size: Point,
    },
    RoomRectCenterSize {
        center: Point,
        size: Point,
    },
    RoomRectSizeOnly {
        size: SizeValue,
    },
    RoomFill {
        between: [String; 2],
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<f64>,
    },
    RoomRectSpan {
        #[serde(rename = "spanX")]
        span_x: SpanX,
        #[serde(rename = "spanY")]
        span_y: SpanY,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelativeDirection {
    NorthOf,
    SouthOf,
    EastOf,
    WestOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlignmentType {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlignEdge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachDirective {
    #[serde(rename = "type")]
    pub node_type: String,
    pub direction: RelativeDirection,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AlignDirective {
    AlignDirective {
        #[serde(skip_serializing_if = "Option::is_none")]
        alignment: Option<AlignmentType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        my_edge: Option<AlignEdge>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_edge: Option<AlignEdge>,
    },
}

impl AlignDirective {
    pub fn simple(alignment: AlignmentType) -> Self {
        Self::AlignDirective {
            alignment: Some(alignment),
            my_edge: None,
            with_room: None,
            with_edge: None,
        }
    }

    pub fn explicit(my_edge: AlignEdge, with_room: String, with_edge: AlignEdge) -> Self {
        Self::AlignDirective {
            alignment: None,
            my_edge: Some(my_edge),
            with_room: Some(with_room),
            with_edge: Some(with_edge),
        }
    }

    pub fn alignment(&self) -> Option<AlignmentType> {
        match self {
            Self::AlignDirective { alignment, .. } => *alignment,
        }
    }

    pub fn explicit_parts(&self) -> Option<(AlignEdge, &str, AlignEdge)> {
        match self {
            Self::AlignDirective {
                my_edge: Some(my_edge),
                with_room: Some(with_room),
                with_edge: Some(with_edge),
                ..
            } => Some((*my_edge, with_room.as_str(), *with_edge)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GapDirective {
    #[serde(rename = "type")]
    pub node_type: String,
    pub distance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeRefSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeReference {
    pub room: String,
    pub edge: EdgeRefSide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendDirective {
    #[serde(rename = "type")]
    pub node_type: String,
    pub axis: Axis,
    pub from: EdgeReference,
    pub to: EdgeReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanX {
    #[serde(rename = "type")]
    pub node_type: String,
    pub from: EdgeReference,
    pub to: EdgeReference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpanY {
    #[serde(rename = "type")]
    pub node_type: String,
    pub from: f64,
    pub to: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub geometry: RoomGeometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<AttachDirective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignDirective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<GapDirective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend: Option<ExtendDirective>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutdoorAreaKind {
    Deck,
    Patio,
    Porch,
    CoveredPorch,
    Walkway,
    Driveway,
    Terrace,
    Yard,
    Garden,
}

impl OutdoorAreaKind {
    pub fn from_token(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "deck" => Some(Self::Deck),
            "patio" => Some(Self::Patio),
            "porch" => Some(Self::Porch),
            "covered_porch" | "covered-porch" => Some(Self::CoveredPorch),
            "walkway" | "walk" => Some(Self::Walkway),
            "driveway" | "drive" => Some(Self::Driveway),
            "terrace" => Some(Self::Terrace),
            "yard" => Some(Self::Yard),
            "garden" => Some(Self::Garden),
            _ => None,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Deck => "Deck",
            Self::Patio => "Patio",
            Self::Porch => "Porch",
            Self::CoveredPorch => "Covered Porch",
            Self::Walkway => "Walkway",
            Self::Driveway => "Driveway",
            Self::Terrace => "Terrace",
            Self::Yard => "Yard",
            Self::Garden => "Garden",
        }
    }

    pub fn has_overhead_cover(self) -> bool {
        matches!(self, Self::CoveredPorch)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutdoorGeometry {
    OutdoorPolygon { points: Vec<Point> },
    OutdoorRect { p1: Point, p2: Point },
    OutdoorRectAtSize { at: Point, size: Point },
    OutdoorRectCenterSize { center: Point, size: Point },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutdoorAreaDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub kind: OutdoorAreaKind,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub geometry: OutdoorGeometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FloorMaterialLegendMode {
    Auto,
    Show,
    Hide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendDeclaration {
    #[serde(rename = "type")]
    pub node_type: String,
    pub floor_materials: FloorMaterialLegendMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeSide {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Position {
    #[serde(rename = "absolute")]
    Absolute { value: f64 },
    #[serde(rename = "percentage")]
    Percentage { value: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoorOpening {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub between: Option<[String; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge: Option<EdgeSide>,
    pub at: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing: Option<DoorSwing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing_room: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub double: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorSwing {
    #[serde(rename = "lh")]
    LeftHand,
    #[serde(rename = "rh")]
    RightHand,
    #[serde(rename = "lhr")]
    LeftHandReverse,
    #[serde(rename = "rhr")]
    RightHandReverse,
}

impl DoorSwing {
    pub fn from_token(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "lh" | "left" | "left_hand" | "left-hand" => Some(Self::LeftHand),
            "rh" | "right" | "right_hand" | "right-hand" => Some(Self::RightHand),
            "lhr" | "left_reverse" | "left_hand_reverse" | "left-hand-reverse" => {
                Some(Self::LeftHandReverse)
            }
            "rhr" | "right_reverse" | "right_hand_reverse" | "right-hand-reverse" => {
                Some(Self::RightHandReverse)
            }
            _ => None,
        }
    }

    pub fn opens_to_outside(self) -> bool {
        matches!(self, Self::LeftHandReverse | Self::RightHandReverse)
    }

    pub fn hinge_is_left(self) -> bool {
        matches!(self, Self::LeftHand | Self::LeftHandReverse)
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOpening {
    pub name: String,
    pub room: String,
    pub edge: EdgeSide,
    pub at: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sill: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Opening {
    DoorOpening(DoorOpening),
    WindowOpening(WindowOpening),
}

impl Opening {
    pub fn name(&self) -> &str {
        match self {
            Opening::DoorOpening(d) => &d.name,
            Opening::WindowOpening(w) => &w.name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClearanceSide {
    Front,
    Back,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MirrorAxis {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ObjectPosition {
    Point { point: Point },
    Distance { position: Position },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectWallAttachment {
    #[serde(rename = "type")]
    pub node_type: String,
    pub edge: EdgeSide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectClearanceOverride {
    #[serde(rename = "type")]
    pub node_type: String,
    pub side: ClearanceSide,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    pub catalog_id: String,
    pub room: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<ObjectPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<ObjectWallAttachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facing: Option<EdgeSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror: Option<MirrorAxis>,
    pub clearance_overrides: Vec<ObjectClearanceOverride>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WallThicknessOverride {
    #[serde(rename = "type")]
    pub node_type: String,
    pub room: String,
    pub edge: EdgeSide,
    pub thickness: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Assertion {
    AssertionInsideFootprint {
        target: String,
    },
    AssertionNoOverlap {
        target: String,
    },
    AssertionOpeningsOnWalls,
    AssertionMinRoomArea {
        room: String,
        min_area: f64,
    },
    AssertionRoomsConnected,
    AssertionObjectsInsideRooms,
    AssertionObjectNoOverlap,
    AssertionObjectClearances,
    AssertionOrientationHasWindow {
        room: String,
        target: OrientationTarget,
    },
    AssertionOrientationNearStreet {
        room: String,
    },
    AssertionOrientationAwayFromStreet {
        room: String,
    },
    AssertionOrientationGardenView {
        room: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationTarget {
    MorningSun,
    AfternoonSun,
    GoodSun,
    Street,
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub rooms: Vec<RoomDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<AttachDirective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignDirective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<GapDirective>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CourtyardGeometry {
    CourtyardRect { p1: Point, p2: Point },
    CourtyardPolygon { points: Vec<Point> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourtyardDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub geometry: CourtyardGeometry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    pub footprint: Footprint,
    pub zones: Vec<ZoneDefinition>,
    pub rooms: Vec<RoomDefinition>,
    pub courtyards: Vec<CourtyardDefinition>,
    pub outdoor_areas: Vec<OutdoorAreaDefinition>,
    pub objects: Vec<ObjectDefinition>,
    pub openings: Vec<Opening>,
    pub wall_overrides: Vec<WallThicknessOverride>,
    pub assertions: Vec<Assertion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<RenderDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<UnitsDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<OriginDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<AxisDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<GridDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<DefaultsDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<RenderDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_units: Option<DimensionUnitsDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteDeclaration>,
    pub catalogs: Vec<CatalogDeclaration>,
    pub plan: PlanDefinition,
}

pub fn node_type(name: &str) -> String {
    name.to_string()
}
