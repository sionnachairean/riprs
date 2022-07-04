use flagset::{flags, FlagSet};

#[derive(Debug)]
pub enum Command {
    TextWindow {
        corners: (XY, XY),
        wrap: bool,
        size: u32,
    },
    Viewport {
        corners: (XY, XY),
    },
    ResetWindows,
    EraseWindow,
    EraseView,
    Gotoxy(XY),
    Home,
    EraseEol,
    Color {
        color: PaletteColor,
    },
    SetPalette {
        c: [EGAColor; 16],
    },
    OnePalette {
        color: PaletteColor,
        value: EGAColor,
    },
    WriteMode {
        mode: WriteMode,
    },
    Move(XY),
    Text {
        text_string: String,
    },
    TextXy {
        position: XY,
        text: String,
    },
    FontStyle {
        font: Font,
        direction: FontDirection,
    },
    Pixel {
        position: XY,
    },
    Line {
        ends: (XY, XY),
    },
    Rectangle {
        corners: (XY, XY),
    },
    Bar {
        corners: (XY, XY),
    },
    Circle {
        center: XY,
        radius: u32,
    },
    Oval {
        // i don't understand the difference between this and OvalArc ???
        // - Autumn
        center: XY,
        start_angle: u32,
        end_angle: u32,
        rad: XY,
    },
    FilledOval {
        center: XY,
        rad: XY,
    },
    Arc {
        center: XY,
        start_angle: u32,
        end_angle: u32,
        radius: u32,
    },
    OvalArc {
        center: XY,
        start_angle: u32,
        end_angle: u32,
        rad: XY,
    },
    PieSlice {
        center: XY,
        start_angle: u32,
        end_angle: u32,
        radius: u32,
    },
    OvalPieSlice {
        center: XY,
        start_angle: u32,
        end_angle: u32,
        rad: XY,
    },
    Bezier {
        control_points: [XY; 4],
        cnt: u32,
    },
    Polygon {
        npoints: u32,
        points: Vec<XY>,
    },
    FillPolygon {
        npoints: u32,
        points: Vec<XY>,
    },
    Polyline {
        npoints: u32,
        points: Vec<XY>,
    },
    Fill {
        start: XY,
        border: EGAColor,
    },
    LineStyle {
        style: LineStyle,
        user_pat: u16,
        thick: u32,
    },
    FillStyle {
        pattern: FillPattern,
        color: PaletteColor,
    },
    FillPattern {
        data: [u8; 8],
        color: PaletteColor,
    },
    Mouse {
        num: u32, // now unused, maybe used to be mouse button number
        corners: (XY, XY),
        clk: bool,
        clr: bool,
        text: String,
    },
    KillMouseFields,
    BeginText {
        corners: (XY, XY),
    },
    RegionText {
        justify: bool,
        text: String,
    },
    EndText,
    GetImage {
        corners: (XY, XY),
    },
    PutImage {
        position: XY,
        mode: PasteMode,
    },
    WriteIcon {
        filename: String,
    },
    LoadIcon {
        position: XY,
        mode: PasteMode,
        clipboard: bool,
        filename: String,
    },
    ButtonStyle {
        // absolute marvel of over-featuring
        dimensions: XY,
        orient: LabelOrientation,
        flags: FlagSet<ButtonStyleFlags>,
        bevsize: u32,
        dfore: PaletteColor,
        dback: PaletteColor,
        bright: PaletteColor,
        dark: PaletteColor,
        surface: PaletteColor,
        grp_no: u32,
        flags2: FlagSet<ButtonStyleFlags2>,
        uline_color: PaletteColor,
        corner_color: PaletteColor,
    },
    Button {
        corners: (XY, XY),
        hotkey: u8,
        flags: FlagSet<ButtonFlags>,
        icon_file: Option<String>,
        text_label: Option<String>,
        host_command: Option<String>,
    },
    Define {
        flags: FlagSet<DefineFlags>,
        variable_identifier: String,
        field_width: u32,
        question_text: String,
        default: String,
    },
    Query {
        mode: QueryMode,
        text: String,
    },
    CopyRegion {
        corners: (XY, XY),
        dest_line: u32,
    },
    ReadScene {
        filename: String,
    },
    FileQuery {
        mode: FileQueryMode,
        filename: String,
    },
    EnterBlockMode {
        mode: BlockMode,
        proto: Protocol,
        file_type: FileType,
        filename: String,
    },
    NoMore,
    Unknown,
}

/// an unsigned (x, y) pair of integers
#[derive(Debug)]
pub struct XY {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct PaletteColor(u8);

impl TryFrom<u8> for PaletteColor {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=15 => Ok(PaletteColor(value)),
            _ => Err("Palette colors must be in range 0..=15"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EGAColor(u8);

impl EGAColor {
    pub fn new() -> Self {
        EGAColor(0)
    }
}

impl TryFrom<u8> for EGAColor {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=63 => Ok(EGAColor(value)),
            _ => Err("EGA colors must be in range 0..=63"),
        }
    }
}

impl From<EGAColor> for sdl2::pixels::Color {
    fn from(value: EGAColor) -> Self {
        let index = value.0;

        let r0 = index >> 5 & 1;
        let g0 = index >> 4 & 1;
        let b0 = index >> 3 & 1;
        let r1 = index >> 1 & 2;
        let g1 = index & 2;
        let b1 = index << 1 & 2;

        let r = 0x55 * (r0 + r1);
        let g = 0x55 * (g0 + g1);
        let b = 0x55 * (b0 + b1);

        sdl2::pixels::Color::RGB(r, g, b)
    }
}

#[derive(Debug)]
pub enum WriteMode {
    Normal,
    Xor,
}

impl TryFrom<u32> for WriteMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WriteMode::Normal),
            1 => Ok(WriteMode::Xor),
            _ => Err("Write mode must be 0 or 1"),
        }
    }
}

#[derive(Debug)]
pub enum Font {
    Default,
    Triplex,
    Small,
    SansSerif,
    Gothic,
    Script,
    Simplex,
    TriplexScript,
    Complex,
    European,
    Bold,
}

#[derive(Debug)]
pub enum FontDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum LineStyle {
    Solid,
    Dotted,
    Centered,
    Dashed,
    Custom,
}

#[derive(Debug)]
pub enum FillPattern {
    Background,
    Solid,
    Line,
    LightSlash,
    NormalSlash,
    NormalBackslash,
    LightBackslash,
    LightHatch,
    HeavyCrossHatch,
    InterleavingLine,
    WidelySpacedDot,
    CloselySpacedDot,
}

#[derive(Debug)]
pub enum PasteMode {
    Copy,
    Xor,
    Or,
    And,
    Not,
}

#[derive(Debug)]
pub enum LabelOrientation {
    Above,
    Left,
    Center,
    Right,
    Beneath,
}

flags! {
    pub enum ButtonStyleFlags: u32 {
        Clipboard,
        Invertable,
        Reset,
        Chisel,
        Recessed,
        Dropshadow,
        AutoStamp,
        Icon,
        Plain,
        Bevel,
        Mouse,
        UnderlineHotkey,
        HotIcons,
        VerticalCenter,
        RadioGroup,
        Sunken,
    }
}

flags! {
    pub enum ButtonStyleFlags2: u32 {
        CheckboxGroup,
        HighlightHotkey,
        Explode,
        LeftJustify,
        RightJustify,
    }
}

flags! {
    pub enum ButtonFlags: u32 {
        AlreadySelected,
        DefaultEnter,
    }
}

flags! {
    pub enum DefineFlags: u32 {
        Database,
        NonBlank,
        NonInteractive,
    }
}

#[derive(Debug)]
pub enum QueryMode {
    Now,
    GraphicsClicked,
    TextClicked,
}

#[derive(Debug)]
pub enum FileQueryMode {
    Basic,
    BasicCR,
    FileSize,
    Extended,
    ExtendedPeriod,
}

#[derive(Debug)]
pub enum BlockMode {
    Download,
    Upload,
}

#[derive(Debug)]
pub enum Protocol {
    XmodemChecksum,
    XmodemCrc,
    Xmodem1k,
    Xmodem1kG,
    Kermit,
    YmodemBatch,
    YmodemG,
    Zmodem,
}

#[derive(Debug)]
pub enum FileType {
    RipDisplay,
    RipStore,
    Icn,
    Hlp,
    Composite,
    Active,
}
