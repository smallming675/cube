use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum CornerPiece {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum CornerOrientation {
    Normal,
    OneTwist,
    TwoTwist,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum EdgeOrientation {
    Normal,
    Flipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Corner {
    pub piece: CornerPiece,
    pub orientation: CornerOrientation,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum EdgePiece {
    UR,
    UF,
    UB,
    UL,
    FR,
    BR,
    FL,
    BL,
    DR,
    DF,
    DB,
    DL,
}

impl From<EdgePiece> for Edge {
    fn from(piece: EdgePiece) -> Self {
        Edge {
            piece,
            orientation: EdgeOrientation::Normal,
        }
    }
}

impl From<CornerPiece> for Corner {
    fn from(piece: CornerPiece) -> Self {
        Corner {
            piece,
            orientation: CornerOrientation::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub piece: EdgePiece,
    pub orientation: EdgeOrientation,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TurnDirection {
    CW,
    DOUBLE,
    CCW,
}

impl TurnDirection {
    pub fn reverse(&mut self) {
        *self = match self {
            TurnDirection::CW => TurnDirection::DOUBLE,
            TurnDirection::DOUBLE => TurnDirection::CW,
            TurnDirection::CCW => TurnDirection::CCW,
        };
    }
    pub fn combine(self, second: Self) -> Option<Self> {
        Some(match self {
            TurnDirection::CW => match second {
                TurnDirection::CW => TurnDirection::DOUBLE,
                TurnDirection::DOUBLE => TurnDirection::CCW,
                TurnDirection::CCW => return None,
            },
            TurnDirection::DOUBLE => match second {
                TurnDirection::CW => TurnDirection::CCW,
                TurnDirection::DOUBLE => return None,
                TurnDirection::CCW => TurnDirection::CW,
            },
            TurnDirection::CCW => match second {
                TurnDirection::CW => return None,
                TurnDirection::DOUBLE => TurnDirection::CW,
                TurnDirection::CCW => TurnDirection::DOUBLE,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]

pub enum Face {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl Face {
    pub fn index(&self) -> usize {
        match self {
            Face::U => 0,
            Face::B => 1,
            Face::R => 2,
            Face::F => 3,
            Face::L => 4,
            Face::D => 5,
        }
    }
    pub fn is_opposite_face(&self, other: &Self) -> bool {
        match (self, other) {
            (Face::U, Face::D) | (Face::D, Face::U) => true,
            (Face::L, Face::R) | (Face::R, Face::L) => true,
            (Face::F, Face::B) | (Face::B, Face::F) => true,
            _ => false,
        }
    }
}

impl CornerOrientation {
    pub fn twist(self) -> Self {
        match self {
            CornerOrientation::Normal => CornerOrientation::OneTwist,
            CornerOrientation::OneTwist => CornerOrientation::TwoTwist,
            CornerOrientation::TwoTwist => CornerOrientation::Normal,
        }
    }
    pub fn double_twist(self) -> Self {
        match self {
            CornerOrientation::Normal => CornerOrientation::TwoTwist,
            CornerOrientation::OneTwist => CornerOrientation::Normal,
            CornerOrientation::TwoTwist => CornerOrientation::OneTwist,
        }
    }
}

impl EdgeOrientation {
    pub const fn flip(self) -> EdgeOrientation {
        match self {
            EdgeOrientation::Normal => EdgeOrientation::Flipped,
            EdgeOrientation::Flipped => EdgeOrientation::Normal,
        }
    }
}

impl Corner {
    pub fn from_colors(colors: [Color; 3], index: usize) -> Option<Self> {
        let mut colors_sorted = colors;
        colors_sorted.sort();

        let piece = match colors_sorted {
            [Color::White, Color::Red, Color::Green] => CornerPiece::UFR,
            [Color::White, Color::Red, Color::Blue] => CornerPiece::UBR,
            [Color::White, Color::Orange, Color::Green] => CornerPiece::UFL,
            [Color::White, Color::Orange, Color::Blue] => CornerPiece::UBL,
            [Color::Yellow, Color::Red, Color::Green] => CornerPiece::DFR,
            [Color::Yellow, Color::Red, Color::Blue] => CornerPiece::DBR,
            [Color::Yellow, Color::Orange, Color::Green] => CornerPiece::DFL,
            [Color::Yellow, Color::Orange, Color::Blue] => CornerPiece::DBL,
            _ => return None,
        };

        let orientation = match match colors {
            [Color::White | Color::Yellow, _, _] => 0,
            [_, Color::White | Color::Yellow, _] => 1,
            [_, _, Color::White | Color::Yellow] => 2,

            _ => return None,
        } {
            0 => CornerOrientation::Normal,
            1 => {
                if index % 2 == 0 {
                    CornerOrientation::TwoTwist
                } else {
                    CornerOrientation::OneTwist
                }
            }
            2 => {
                if index % 2 == 0 {
                    CornerOrientation::OneTwist
                } else {
                    CornerOrientation::TwoTwist
                }
            }
            _ => unreachable!(),
        };

        Some(Corner { piece, orientation })
    }
    pub fn twist(&mut self) {
        self.orientation = self.orientation.twist();
    }
    pub fn double_twist(&mut self) {
        self.orientation = self.orientation.double_twist();
    }
}
impl Edge {
    #[rustfmt::skip]
    pub fn from_colors(colors: [Color; 2]) -> Option<Self> {
        let edge_mapping = [
            ((Color::White, Color::Red), (EdgePiece::UR, EdgeOrientation::Normal)),
            ((Color::Red, Color::White), (EdgePiece::UR, EdgeOrientation::Flipped)),
            ((Color::White, Color::Green), (EdgePiece::UF, EdgeOrientation::Normal)),
            ((Color::Green, Color::White), (EdgePiece::UF, EdgeOrientation::Flipped)),
            ((Color::White, Color::Orange), (EdgePiece::UL, EdgeOrientation::Normal)),
            ((Color::Orange, Color::White), (EdgePiece::UL, EdgeOrientation::Flipped)),
            ((Color::White, Color::Blue), (EdgePiece::UB, EdgeOrientation::Normal)),
            ((Color::Blue, Color::White), (EdgePiece::UB, EdgeOrientation::Flipped)),
            ((Color::Yellow, Color::Red), (EdgePiece::DR, EdgeOrientation::Normal)),
            ((Color::Red, Color::Yellow), (EdgePiece::DR, EdgeOrientation::Flipped)),
            ((Color::Yellow, Color::Green), (EdgePiece::DF, EdgeOrientation::Normal)),
            ((Color::Green, Color::Yellow), (EdgePiece::DF, EdgeOrientation::Flipped)),
            ((Color::Yellow, Color::Orange), (EdgePiece::DL, EdgeOrientation::Normal)),
            ((Color::Orange, Color::Yellow), (EdgePiece::DL, EdgeOrientation::Flipped)),
            ((Color::Yellow, Color::Blue), (EdgePiece::DB, EdgeOrientation::Normal)),
            ((Color::Blue, Color::Yellow), (EdgePiece::DB, EdgeOrientation::Flipped)),
            ((Color::Green, Color::Red), (EdgePiece::FR, EdgeOrientation::Normal)),
            ((Color::Red, Color::Green), (EdgePiece::FR, EdgeOrientation::Flipped)),
            ((Color::Blue, Color::Red), (EdgePiece::BR, EdgeOrientation::Normal)),
            ((Color::Red, Color::Blue), (EdgePiece::BR, EdgeOrientation::Flipped)),
            ((Color::Green, Color::Orange), (EdgePiece::FL, EdgeOrientation::Normal)),
            ((Color::Orange, Color::Green), (EdgePiece::FL, EdgeOrientation::Flipped)),
            ((Color::Blue, Color::Orange), (EdgePiece::BL, EdgeOrientation::Normal)),
            ((Color::Orange, Color::Blue), (EdgePiece::BL, EdgeOrientation::Flipped)),
        ];

        for &((color1, color2), (piece, orientation)) in edge_mapping.iter() {
            if (colors[0], colors[1]) == (color1, color2) {
                return Some(Edge { piece, orientation });
            }
        }

        None
    }
    pub fn flip(&mut self) {
        self.orientation = self.orientation.flip();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceLayers {
    E,
    M,
    S,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        match self {
            Color::White => Color32::from_rgb(255, 255, 255),
            Color::Yellow => Color32::from_rgb(255, 255, 0),
            Color::Red => Color32::from_rgb(255, 0, 0),
            Color::Orange => Color32::from_rgb(255, 165, 0),
            Color::Blue => Color32::from_rgb(0, 0, 255),
            Color::Green => Color32::from_rgb(0, 128, 0),
        }
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}
