// Copyright (c) IxMilia.  All Rights Reserved.  Licensed under the Apache License, Version 2.0.  See License.txt in the project root for license information.

extern crate num;

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dwf3DPrecision {
    Deviation_1 = 1,
    Deviation_0_5 = 2,
    Deviation_0_2 = 3,
    Deviation_0_1 = 4,
    Deviation_0_01 = 5,
    Deviation_0_001 = 6,
}
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AcadVersion {
    Version_1_0,
    Version_1_2,
    Version_1_40,
    Version_2_05,
    Version_2_10,
    Version_2_21,
    Version_2_22,
    Version_2_5,
    Version_2_6,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R2000,
    R2004,
    R2007,
    R2010,
    R2013,
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AngleDirection {
    CounterClockwise = 0,
    Clockwise = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AngleFormat {
    DecimalDegrees = 0,
    DegreesMinutesSeconds = 1,
    Gradians = 2,
    Radians = 3,
    SurveyorsUnits = 4,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachmentPoint {
    TopLeft = 1,
    TopCenter = 2,
    TopRight = 3,
    MiddleLeft = 4,
    MiddleCenter = 5,
    MiddleRight = 6,
    BottomLeft = 7,
    BottomCenter = 8,
    BottomRight = 9,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeVisibility {
    None = 0,
    Normal = 1,
    All = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackgroundFillSetting {
    Off = 0,
    UseBackgroundFillColor = 1,
    UseDrawingWindowColor = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BottomTextAttachmentDirection {
    Center = 9,
    UnderlineAndCenter = 10,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoordinateDisplay {
    Static = 0,
    ContinuousUpdate = 1,
    DistanceAngleFormat = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DefaultLightingType
{
    OneDistantLight = 0,
    TwoDistantLights = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionArcSymbolDisplayMode {
    SymbolBeforeText = 0,
    SymbolAboveText = 1,
    Suppress = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionAssociativity {
    NoAssociationExploded = 0,
    NonAssociativeObjects = 1,
    AssociativeObjects = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionFit {
    TextAndArrowsOutsideLines = 0,
    MoveArrowsFirst = 1,
    MoveTextFirst = 2,
    MoveEitherForBestFit = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionFractionFormat {
    HorizontalStacking = 0,
    DiagonalStacking = 1,
    NotStacked = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionTextBackgroundColorMode {
    None = 0,
    UseDrawingBackground = 1,
    Custom = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionTextJustification {
    AboveLineCenter = 0,
    AboveLineNextToFirstExtension = 1,
    AboveLineNextToSecondExtension = 2,
    AboveLineCenteredOnFirstExtension = 3,
    AboveLineCenteredOnSecondExtension = 4,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionTextMovementRule {
    MoveLineWithText = 0,
    AddLeaderWhenTextIsMoved = 1,
    MoveTextFreely = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionType {
    RotatedHorizontalOrVertical = 0,
    Aligned = 1,
    Angular = 2,
    Diameter = 3,
    Radius = 4,
    AngularThreePoint = 5,
    Ordinate = 6,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DragMode {
    Off = 0,
    On = 1,
    Auto = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawingDirection {
    LeftToRight = 1,
    TopToBottom = 3,
    ByStyle = 5,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawingUnits {
    English = 0,
    Metric = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndCapSetting {
    None = 0,
    Round = 1,
    Angle = 2,
    Square = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontType {
    TTF = 0,
    SHX = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HelixConstraint {
    ConstrainTurnHeight = 0,
    ConstrainTurns = 1,
    ConstrainHeight = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HorizontalTextJustification {
    Left = 0,
    Center = 1,
    Right = 2,
    Aligned = 3,
    Middle = 4,
    Fit = 5,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageClippingBoundaryType {
    Rectangular = 1,
    Polygonal = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JoinStyle {
    None = 0,
    Round = 1,
    Angle = 2,
    Flat = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Justification {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayerAndSpatialIndexSaveMode {
    None = 0,
    LayerIndex = 1,
    SpatialIndex = 2,
    LayerAndSpatialIndex = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeaderCreationAnnotationType {
    WithTextAnnotation = 0,
    WithToleranceAnnotation = 1,
    WithBlockReferenceAnnotation = 2,
    NoAnnotation = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeaderHooklineDirection {
    OppositeFromHorizontalVector = 0,
    SameAsHorizontalVector = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeaderPathType {
    StraightLineSegments = 0,
    Spline = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightAttenuationType {
    None = 0,
    InverseLinear = 1,
    InverseSquare = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightType {
    Distant = 1,
    Point = 2,
    Spot = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LinetypeStyle {
    Off = 0,
    Solid = 1,
    Dashed = 2,
    Dotted = 3,
    ShortDash = 4,
    MediumDash = 5,
    LongDash = 6,
    DoubleShortDash = 7,
    DoubleMediumDash = 8,
    DoubleLongDash = 9,
    MediumLongDash = 10,
    SparseDot = 11,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoftedObjectNormalMode {
    Ruled = 0,
    SmoothFit = 1,
    StartCrossSection = 2,
    EndCrossSection = 3,
    StartAndEndCrossSections = 4,
    AllCrossSections = 5,
    UseDraftAngleAndMagnitude = 6,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTextFlag {
    MultilineAttribute = 2,
    ConstantMultilineAttributeDefinition = 4,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTextLineSpacingStyle {
    AtLeast = 1,
    Exact = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NonAngularUnits {
    Scientific = 1,
    Decimal = 2,
    Engineering = 3,
    Architectural = 4,
    Fractional = 5,
    WindowsDesktop = 6,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OleObjectType {
    Link = 1,
    Embedded = 2,
    Static = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrthographicViewType {
    None = 0,
    Top = 1,
    Bottom = 2,
    Front = 3,
    Back = 4,
    Left = 5,
    Right = 6,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PickStyle {
    None = 0,
    Group = 1,
    AssociativeHatch = 2,
    GroupAndAssociativeHatch = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlotStyle {
    ByLayer = 0,
    ByBlock = 1,
    ByDictionaryDefault = 2,
    ByObjectId = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PolylineCurvedAndSmoothSurfaceType {
    None = 0,
    QuadraticBSpline = 5,
    CubicBSpline = 6,
    Bezier = 8,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PolySketchMode {
    SketchLines = 0,
    SketchPolylines = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShadeEdgeMode {
    FacesShadedEdgeNotHighlighted = 0,
    FacesShadedEdgesHighlightedInBlack = 1,
    FacesNotFilledEdgesInEntityColor = 2,
    FacesInEntityColorEdgesInBlack = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShadowMode {
    CastsAndReceivesShadows = 0,
    CastsShadows = 1,
    ReceivesShadows = 2,
    IgnoresShadows = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShadowType {
    RayTraced = 0,
    ShadowMaps = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnapIsometricPlane {
    Left = 0,
    Top = 1,
    Right = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnapStyle {
    Standard = 0,
    Isometric = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SolidHistoryMode {
    None = 0,
    DoesNotOverride = 1,
    Override = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAttachmentDirection {
    Horizontal = 0,
    Vertical = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextDirection {
    LeftToRight = 0,
    RightToLeft = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextLineSpacingStyle {
    AtLeast = 1,
    Exact = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileModeDescriptor {
    InTiledViewport = 0,
    InNonTiledViewport = 1,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DrawingTimeZone {
    InternationalDateLineWest = -12000,
    MidwayIsland_Samoa = -11000,
    Hawaii = -10000,
    Alaska = -9000,
    PacificTime_US_Canada_SanFrancisco_Vancouver = -8000,
    Arizona = -7000,
    //Chihuahua_LaPaz_Mazatlan = -7000,
    //MountainTime_US_Canada = -7000,
    Mazatlan = -7002,
    CentralAmerica = -6000,
    CentralTime_US_Canada = -6001,
    Guadalajara_MexicoCity_Monterrey = -6002,
    Saskatchewan = -6003,
    EasternTime_US_Canada_ = -5000,
    Indiana_East_ = -5001,
    Bogota_Lima_Quito = -5002,
    AtlanticTime_Canada_ = -4000,
    Caracas_LaPaz = -4001,
    Santiago = -4002,
    Newfoundland = -3300,
    Brasilia = -3000,
    BuenosAires_Georgetown = -3001,
    Greenland = -3002,
    MidAtlantic = -2000,
    Azores = -1000,
    CapeVerdeIs = -1001,
    UniversalCoordinatedTime = 0,
    GreenwichMeanTime = 1,
    Casablanca_Monrovia = 2,
    Amsterdam_Berlin_Bern_Rome_Stockholm = 1000,
    Brussels_Madrid_Copenhagen_Paris = 1001,
    Belgrade_Bratislava_Budapest_Ljubljana_Prague = 1002,
    Sarajevo_Skopje_Warsaw_Zagreb = 1003,
    WestCentralAfrica = 1004,
    Athens_Beirut_Istanbul_Minsk = 2000,
    Bucharest = 2001,
    Cairo = 2002,
    Harare_Pretoria = 2003,
    Helsinki_Kyiv_Sofia_Talinn_Vilnius = 2004,
    Jerusalem = 2005,
    Moscow_StPetersburg_Volograd = 3000,
    Kuwait_Riyadh = 3001,
    Baghdad = 3002,
    Nairobi = 3003,
    Tehran = 3300,
    AbuDhabi_Muscat = 4000,
    Baku_Tbilisi_Yerevan = 4001,
    Kabul = 4300,
    Ekaterinburg = 5000,
    Islamabad_Karachi_Tashkent = 5001,
    Chennai_Kolkata_Mumbai_NewDelhi = 5300,
    Kathmandu = 5450,
    Almaty_Novosibirsk = 6000,
    Astana_Dhaka = 6001,
    SriJayawardenepura = 6002,
    Rangoon = 6300,
    Bangkok_Hanoi_Jakarta = 7000,
    Krasnoyarsk = 7001,
    Beijing_Chongqing_HongKong_Urumqi = 8000,
    KualaLumpur_Singapore = 8001,
    Taipei = 8002,
    Irkutsk_UlaanBataar = 8003,
    Perth = 8004,
    Osaka_Sapporo_Tokyo = 9000,
    Seoul = 9001,
    Yakutsk = 9002,
    Adelaide = 9300,
    Darwin = 9301,
    Canberra_Melbourne_Sydney = 10000,
    Guam_PortMoresby = 10001,
    Brisbane = 10002,
    Hobart = 10003,
    Vladivostok = 10004,
    Magadan_SolomonIs_NewCaledonia = 11000,
    Auckland_Wellington = 12000,
    Fiji_Kamchatka_MarshallIs = 12001,
    Nukualofa_Tonga = 13000,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopTextAttachmentDirection {
    Center = 9,
    OverlineAndCenter = 10,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnderlayFrameMode {
    None = 0,
    DisplayAndPlot = 1,
    DisplayNoPlot = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitFormat {
    Scientific = 1,
    Decimal = 2,
    Engineering = 3,
    ArchitecturalStacked = 4,
    FractionalStacked = 5,
    Architectural = 6,
    Fractional = 7,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Units {
    Unitless = 0,
    Inches = 1,
    Feet = 2,
    Miles = 3,
    Millimeters = 4,
    Centimeters = 5,
    Meters = 6,
    Kilometers = 7,
    Microinches = 8,
    Mils = 9,
    Yards = 10,
    Angstroms = 11,
    Nanometers = 12,
    Microns = 13,
    Decimeters = 14,
    Decameters = 15,
    Hectometers = 16,
    Gigameters = 17,
    AstronomicalUnits = 18,
    LightYears = 19,
    Parsecs = 20,
}
}

pub struct ViewMode {
    flags: i32,
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode {
            flags: 0,
        }
    }
}

impl ViewMode {
    pub fn from_i16(val: i16) -> Self {
        ViewMode {
            flags: val as i32,
        }
    }
    pub fn get_raw(&self) -> i32 {
        self.flags
    }
    fn get_flag(&self, mask: i32) -> bool {
        self.flags & mask != 0
    }
    fn set_flag(&mut self, mask: i32, val: bool) {
        if val {
            self.flags |= mask;
        }
        else {
            self.flags &= !mask
        }
    }
    pub fn get_is_perspective_view_active(&self) -> bool {
        self.get_flag(1)
    }
    pub fn set_is_perspective_view_active(&mut self, val: bool) {
        self.set_flag(1, val)
    }
    pub fn get_is_front_clipping_on(&self) -> bool {
        self.get_flag(2)
    }
    pub fn set_is_front_clipping_on(&mut self, val: bool) {
        self.set_flag(2, val)
    }
    pub fn get_is_back_clipping_on(&self) -> bool {
        self.get_flag(4)
    }
    pub fn set_is_back_clipping_on(&mut self, val: bool) {
        self.set_flag(4, val)
    }
    pub fn get_is_ucs_follow_mode_on(&self) -> bool {
        self.get_flag(8)
    }
    pub fn set_is_ucs_follow_mode_on(&mut self, val: bool) {
        self.set_flag(8, val)
    }
    pub fn get_is_front_clipping_at_eye(&self) -> bool {
        self.get_flag(16)
    }
    pub fn set_is_front_clipping_at_eye(&mut self, val: bool) {
        self.set_flag(16, val)
    }
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewRenderMode
{
    Classic2D = 0,
    Wireframe = 1,
    HiddenLine = 2,
    FlatShaded = 3,
    GouraudShaded = 4,
    FlatShadedWithWireframe = 5,
    GouraudShadedWithWireframe = 6,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Version {
    R2010 = 0,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerticalTextJustification {
    Baseline = 0,
    Bottom = 1,
    Middle = 2,
    Top = 3,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XrefClippingBoundaryVisibility {
    NotDisplayedNotPlotted = 0,
    DisplayedAndPlotted = 1,
    DisplayedNotPlotted = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitZeroSuppression {
    SuppressZeroFeetAndZeroInches = 0,
    IncludeZeroFeetAndZeroInches = 1,
    IncludeZeroFeetAndSuppressZeroInches = 2,
    IncludeZeroInchesAndSuppressZeroFeet = 3,
}
}
