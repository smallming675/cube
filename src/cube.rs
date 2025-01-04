use crate::{
    cache::{
        CORNER_ORIENTATION_COORDINATE, CORNER_PERMUTATION_COORDINATE, EDGE_ORIENTATION_COORDINATE,
        EDGE_PERMUTATION_COORDINATE, UD_PERMUTATION_COORDINATE, UD_PHASE_2_PERMUTATION_COORDINATE,
        UD_SLICE_COMBINATIONS,
    },
    misc::{
        decode_number_base, get_ud_slice_combination, inverse_permutation_index, permutation_index,
    },
    moves::{Move, AMOUNT_OF_MOVES, AMOUNT_OF_STAGE_2_MOVES},
    piece::{
        Color, Corner, CornerOrientation, CornerPiece, Edge, EdgeOrientation, EdgePiece, Face,
        SliceLayers, TurnDirection,
    },
};

pub const SOLVED: u64 = 0;

pub trait Cube {
    fn apply_move(&mut self, mve: Move) -> Self;
    fn is_solved(&self) -> bool;
    fn cycle<T>(a: T, b: T, c: T, d: T, turn: TurnDirection) -> (T, T, T, T) {
        match turn {
            TurnDirection::CW => (d, a, b, c),
            TurnDirection::DOUBLE => (c, d, a, b),
            TurnDirection::CCW => (b, c, d, a),
        }
    }

    fn get_face(face: Face) -> ([usize; 4], [usize; 4]) {
        match face {
            Face::U => ([0, 1, 2, 3], [0, 1, 2, 3]),
            Face::D => ([4, 5, 6, 7], [8, 9, 10, 11]),
            Face::L => ([0, 3, 4, 7], [3, 7, 11, 4]),
            Face::R => ([2, 1, 6, 5], [1, 5, 9, 6]),
            Face::F => ([3, 2, 5, 4], [2, 6, 8, 7]),
            Face::B => ([1, 0, 7, 6], [0, 4, 10, 5]),
        }
    }

    fn get_slice(slice: SliceLayers) -> [usize; 4] {
        match slice {
            SliceLayers::E => [4, 5, 6, 7],
            SliceLayers::M => [2, 0, 10, 8],
            SliceLayers::S => [3, 1, 9, 11],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubieCube {
    pub corners: [Corner; 8],
    pub edges: [Edge; 12],
}

impl CubieCube {
    pub fn from_colors(colors: [Color; 54]) -> Option<CubieCube> {
        let mut cube = CubieCube::new();
        for i in 0..12 {
            let indicies = Self::get_indicies_of_edge(i);
            cube.edges[i] = Edge::from_colors([colors[indicies.0], colors[indicies.1]])?;
        }

        for i in 0..8 {
            let indicies = Self::get_indicies_of_corner(i);
            cube.corners[i] = Corner::from_colors(
                [colors[indicies.0], colors[indicies.1], colors[indicies.2]],
                i,
            )?;
        }
        Some(cube)
    }

    pub fn apply_moves(&mut self, moves: Vec<Move>) -> Self {
        for mve in moves {
            self.apply_move(mve);
        }
        *self
    }

    fn get_colors_of_corner(corner: &CornerPiece) -> (Color, Color, Color) {
        match corner {
            CornerPiece::UFR => (Color::White, Color::Green, Color::Red),
            CornerPiece::UFL => (Color::White, Color::Green, Color::Orange),
            CornerPiece::UBL => (Color::White, Color::Blue, Color::Orange),
            CornerPiece::UBR => (Color::White, Color::Blue, Color::Red),
            CornerPiece::DFR => (Color::Yellow, Color::Green, Color::Red),
            CornerPiece::DFL => (Color::Yellow, Color::Green, Color::Orange),
            CornerPiece::DBL => (Color::Yellow, Color::Blue, Color::Orange),
            CornerPiece::DBR => (Color::Yellow, Color::Blue, Color::Red),
        }
    }
    fn get_colors_of_edge(edge: &EdgePiece) -> (Color, Color) {
        match edge {
            EdgePiece::UF => (Color::White, Color::Green),
            EdgePiece::UR => (Color::White, Color::Red),
            EdgePiece::UB => (Color::White, Color::Blue),
            EdgePiece::UL => (Color::White, Color::Orange),
            EdgePiece::BR => (Color::Blue, Color::Red),
            EdgePiece::FR => (Color::Green, Color::Red),
            EdgePiece::BL => (Color::Blue, Color::Orange),
            EdgePiece::FL => (Color::Green, Color::Orange),
            EdgePiece::DF => (Color::Yellow, Color::Green),
            EdgePiece::DR => (Color::Yellow, Color::Red),
            EdgePiece::DB => (Color::Yellow, Color::Blue),
            EdgePiece::DL => (Color::Yellow, Color::Orange),
        }
    }
    fn get_indicies_of_corner(corner_index: usize) -> (usize, usize, usize) {
        match corner_index {
            0 => (0, 9 + 2, 36),
            1 => (2, 9, 18 + 2),
            2 => (8, 27 + 2, 18),
            3 => (6, 27, 36 + 2),
            4 => (45, 27 + 6, 36 + 8),
            5 => (45 + 2, 27 + 8, 18 + 6),
            6 => (45 + 8, 9 + 6, 18 + 8),
            7 => (45 + 6, 9 + 8, 36 + 6),
            _ => unreachable!(),
        }
    }
    fn get_indicies_of_edge(edge_index: usize) -> (usize, usize) {
        match edge_index {
            0 => (1, 9 + 1),
            1 => (5, 18 + 1),
            2 => (7, 27 + 1),
            3 => (3, 36 + 1),
            4 => (9 + 5, 36 + 3),
            5 => (9 + 3, 18 + 5),
            6 => (27 + 5, 18 + 3),
            7 => (27 + 3, 36 + 5),
            8 => (45 + 1, 27 + 7),
            9 => (45 + 5, 18 + 7),
            10 => (45 + 7, 9 + 7),
            11 => (45 + 3, 36 + 7),
            _ => unreachable!(),
        }
    }
    fn get_indicies_of_center(center_index: usize) -> usize {
        match center_index {
            0 => 4,
            1 => 9 + 4,
            2 => 18 + 4,
            3 => 27 + 4,
            4 => 36 + 4,
            5 => 45 + 4,
            _ => unreachable!(),
        }
    }

    // No
    pub fn to_colors(&self) -> [Color; 54] {
        let mut colors = [Color::White; 54];
        for (index, corner) in self.corners.iter().enumerate() {
            let corner_colors = Self::get_colors_of_corner(&corner.piece);
            let corner_colors = [corner_colors.0, corner_colors.1, corner_colors.2];

            let subindicies = Self::get_indicies_of_corner(index);
            let top_index = match corner.orientation {
                CornerOrientation::Normal => 0,
                CornerOrientation::OneTwist => {
                    if CubieCube::get_solved_index_corner(corner.piece) % 2 == 0 {
                        1
                    } else {
                        2
                    }
                }
                CornerOrientation::TwoTwist => {
                    if CubieCube::get_solved_index_corner(corner.piece) % 2 == 0 {
                        2
                    } else {
                        1
                    }
                }
            };

            let front_index = match corner.orientation {
                CornerOrientation::Normal => {
                    if CubieCube::get_solved_index_corner(corner.piece) % 2 == index % 2 {
                        1
                    } else {
                        2
                    }
                }
                CornerOrientation::OneTwist => {
                    match (
                        CubieCube::get_solved_index_corner(corner.piece) % 2,
                        index % 2,
                    ) {
                        (0, 0) => 2,
                        (0, 1) => 0,
                        (1, 0) => 1,
                        (1, 1) => 0,
                        _ => unreachable!(),
                    }
                }
                CornerOrientation::TwoTwist => {
                    match (
                        CubieCube::get_solved_index_corner(corner.piece) % 2,
                        index % 2,
                    ) {
                        (0, 0) => 0,
                        (0, 1) => 1,
                        (1, 0) => 0,
                        (1, 1) => 2,
                        _ => unreachable!(),
                    }
                }
            };

            let side_index = (0..3)
                .filter(|x| *x != top_index && *x != front_index)
                .next()
                .unwrap();

            colors[subindicies.0] = corner_colors[top_index];
            colors[subindicies.1] = corner_colors[front_index];
            colors[subindicies.2] = corner_colors[side_index];
        }
        for (index, edge) in self.edges.iter().enumerate() {
            let edge_colors = Self::get_colors_of_edge(&edge.piece);
            let subindicies = Self::get_indicies_of_edge(index);
            match edge.orientation {
                EdgeOrientation::Normal => {
                    colors[subindicies.0] = edge_colors.0;
                    colors[subindicies.1] = edge_colors.1;
                }
                EdgeOrientation::Flipped => {
                    colors[subindicies.0] = edge_colors.1;
                    colors[subindicies.1] = edge_colors.0;
                }
            }
        }
        for (i, color) in [
            Color::White,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Orange,
            Color::Yellow,
        ]
        .iter()
        .enumerate()
        {
            colors[Self::get_indicies_of_center(i)] = *color
        }
        colors
    }
    pub fn from_equator(equator: SliceLayers) -> Self {
        let mut cube = CubieCube::new();
        match equator {
            SliceLayers::E => cube,
            SliceLayers::S => {
                cube.apply_move(Move::R1);
                cube.apply_move(Move::L3);
                cube.apply_slice_move(SliceLayers::M, TurnDirection::CCW);
                cube
            }

            SliceLayers::M => {
                cube.apply_move(Move::F1);
                cube.apply_move(Move::B3);
                cube.apply_slice_move(SliceLayers::S, TurnDirection::CCW);
                cube
            }
        }
    }

    pub fn apply_slice_move(&mut self, slice: SliceLayers, direction: TurnDirection) {
        let slice = CubieCube::get_slice(slice);

        (
            self.edges[slice[0]],
            self.edges[slice[1]],
            self.edges[slice[2]],
            self.edges[slice[3]],
        ) = CubieCube::cycle(
            self.edges[slice[0]],
            self.edges[slice[1]],
            self.edges[slice[2]],
            self.edges[slice[3]],
            direction,
        );
    }

    pub fn from_ud_slice_phase_2_permutation(permutation: u8) -> Self {
        // permutation: 0..24

        let ud_slice_locations = inverse_permutation_index(permutation as usize as u64, 4, 4);
        let mut cube = CubieCube::new();
        for (i, &edge) in ud_slice_locations.iter().enumerate() {
            cube.edges[match i {
                0 => 4,
                1 => 5,
                2 => 6,
                3 => 7,
                _ => unreachable!(),
            }] = Edge {
                piece: match edge {
                    0 => EdgePiece::BL,
                    1 => EdgePiece::BR,
                    2 => EdgePiece::FR,
                    3 => EdgePiece::FL,
                    _ => unreachable!(),
                },
                orientation: EdgeOrientation::Normal,
            };
        }
        cube
    }

    // When we are dealing with phase 2 edges, we want to shrink the problem space from 12 to 8,
    // since we dont have to account for the equator
    pub fn normalize_edge_index(index: usize) -> usize {
        match index {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            8 => 4,
            9 => 5,
            10 => 6,
            11 => 7,
            _ => unreachable!(),
        }
    }

    pub fn from_corner_permutation(permutation: u64) -> Self {
        let mut cube = CubieCube::new();
        let permutation = inverse_permutation_index(permutation, 8, 8);

        for (i, corner) in permutation.iter().enumerate() {
            cube.corners[i] = Corner {
                piece: CubieCube::from_corner_index(*corner as usize),
                orientation: CornerOrientation::Normal,
            };
        }
        cube
    }

    pub fn from_phase_2_edge_permutation(permutation: u64) -> Self {
        // Remember there are only 8! amount of phase 2 edge permutations

        let mut cube = CubieCube::new();
        let permutation = inverse_permutation_index(permutation, 8, 8);
        for (i, &edge) in permutation.iter().enumerate() {
            // The first 4 index of i represent the top edges, while the bottom 4 repesents the
            // bottom edges
            cube.edges[match i {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 8,
                5 => 9,
                6 => 10,
                7 => 11,
                _ => unreachable!(),
            }] = Edge {
                piece: CubieCube::from_edge_index(match edge {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 8,
                    5 => 9,
                    6 => 10,
                    7 => 11,
                    _ => unreachable!(),
                } as usize),
                orientation: EdgeOrientation::Normal,
            };
        }
        cube
    }
    pub fn from_ud_slice_permutation(permutation: u64) -> Self {
        // permutation is a number from 0..495 (C(12,4))
        let slice_permutation = UD_SLICE_COMBINATIONS[permutation as usize];

        let mut cube = CubieCube::new();
        cube.edges[4].piece = EdgePiece::UB;
        cube.edges[5].piece = EdgePiece::UB;
        cube.edges[6].piece = EdgePiece::UB;
        cube.edges[7].piece = EdgePiece::UB;

        for (i, &edge) in slice_permutation.iter().enumerate() {
            cube.edges[edge as usize] = Edge {
                piece: match i {
                    0 => EdgePiece::BL,
                    1 => EdgePiece::BR,
                    2 => EdgePiece::FL,
                    3 => EdgePiece::FR,
                    _ => unreachable!(),
                },
                orientation: EdgeOrientation::Normal,
            };
        }
        cube
    }

    pub fn from_edge_orientation(orientation: u64) -> Self {
        let edge_orientation = decode_number_base(orientation, 2, 11);
        let mut cube = CubieCube::new();
        let mut flips = 0;
        for orientation in edge_orientation.iter().enumerate() {
            if *orientation.1 == 1 {
                cube.edges[orientation.0].flip();
                flips += 1;
            }
        }

        // orientation only stores the orientation of the first 11 edges, so we deduce the final
        // edge based on the flips of the previous edges (even amount of edge flips)
        if flips % 2 == 1 {
            cube.edges[11].flip();
        }

        cube
    }

    pub fn from_corner_orientation(orientation: u64) -> Self {
        let corner_orientation = decode_number_base(orientation, 3, 6);
        let mut cube = CubieCube::new();
        let mut twists = 0;
        for (i, &orientation) in corner_orientation.iter().enumerate() {
            let piece = CubieCube::from_corner_index(i);
            twists += orientation;

            let orientation = match orientation {
                0 => CornerOrientation::Normal,
                1 => CornerOrientation::OneTwist,
                2 => CornerOrientation::TwoTwist,
                _ => unreachable!(),
            };
            cube.corners[i] = Corner { piece, orientation };
        }
        if twists % 3 == 1 {
            cube.corners[7].double_twist();
        } else if twists % 3 == 2 {
            cube.corners[7].twist();
        }

        cube
    }

    fn from_corner_index(index: usize) -> CornerPiece {
        match index {
            0 => CornerPiece::UBL,
            1 => CornerPiece::UBR,
            2 => CornerPiece::UFR,
            3 => CornerPiece::UFL,
            4 => CornerPiece::DFL,
            5 => CornerPiece::DFR,
            6 => CornerPiece::DBR,
            7 => CornerPiece::DBL,
            _ => unreachable!(),
        }
    }

    fn from_edge_index(index: usize) -> EdgePiece {
        match index {
            0 => EdgePiece::UB,
            1 => EdgePiece::UR,
            2 => EdgePiece::UF,
            3 => EdgePiece::UL,
            4 => EdgePiece::BL,
            5 => EdgePiece::BR,
            6 => EdgePiece::FR,
            7 => EdgePiece::FL,
            8 => EdgePiece::DF,
            9 => EdgePiece::DR,
            10 => EdgePiece::DB,
            11 => EdgePiece::DL,
            _ => unreachable!(),
        }
    }
    pub fn get_solved_index_corner(corner: CornerPiece) -> usize {
        match corner {
            CornerPiece::UBL => 0,
            CornerPiece::UBR => 1,
            CornerPiece::UFR => 2,
            CornerPiece::UFL => 3,
            CornerPiece::DFL => 4,
            CornerPiece::DFR => 5,
            CornerPiece::DBR => 6,
            CornerPiece::DBL => 7,
        }
    }
    pub fn get_solved_index_edge(edge: EdgePiece) -> usize {
        match edge {
            EdgePiece::UB => 0,
            EdgePiece::UR => 1,
            EdgePiece::UF => 2,
            EdgePiece::UL => 3,
            EdgePiece::BL => 4,
            EdgePiece::BR => 5,
            EdgePiece::FR => 6,
            EdgePiece::FL => 7,
            EdgePiece::DF => 8,
            EdgePiece::DR => 9,
            EdgePiece::DB => 10,
            EdgePiece::DL => 11,
        }
    }

    pub fn where_is_edge(&self, edge: EdgePiece) -> usize {
        for i in 0..12 {
            if self.edges[i].piece == edge {
                return i;
            }
        }
        0
    }

    pub fn where_is_corner(&self, corner: CornerPiece) -> usize {
        for i in 0..8 {
            if self.corners[i].piece == corner {
                return i;
            }
        }
        0
    }
    pub fn new() -> Self {
        CubieCube {
            corners: [
                CornerPiece::UBL.into(),
                CornerPiece::UBR.into(),
                CornerPiece::UFR.into(),
                CornerPiece::UFL.into(),
                CornerPiece::DFL.into(),
                CornerPiece::DFR.into(),
                CornerPiece::DBR.into(),
                CornerPiece::DBL.into(),
            ],
            edges: [
                EdgePiece::UB.into(),
                EdgePiece::UR.into(),
                EdgePiece::UF.into(),
                EdgePiece::UL.into(),
                EdgePiece::BL.into(),
                EdgePiece::BR.into(),
                EdgePiece::FR.into(),
                EdgePiece::FL.into(),
                EdgePiece::DF.into(),
                EdgePiece::DR.into(),
                EdgePiece::DB.into(),
                EdgePiece::DL.into(),
            ],
        }
    }
}

impl Cube for CubieCube {
    fn is_solved(&self) -> bool {
        *self == CubieCube::new()
    }
    fn apply_move(&mut self, mve: Move) -> Self {
        let (face, direction) = (mve.face(), mve.direction());
        let (corners, edges) = CubieCube::get_face(face);

        if matches!(face, Face::R | Face::F | Face::L | Face::B)
            && direction != TurnDirection::DOUBLE
        {
            self.corners[corners[0]].twist();
            self.corners[corners[1]].double_twist();
            self.corners[corners[2]].twist();
            self.corners[corners[3]].double_twist();
        }

        if matches!(face, Face::F | Face::B) && direction != TurnDirection::DOUBLE {
            for &edge in &edges {
                self.edges[edge].flip();
            }
        }

        (
            self.corners[corners[0]],
            self.corners[corners[1]],
            self.corners[corners[2]],
            self.corners[corners[3]],
        ) = CubieCube::cycle(
            self.corners[corners[0]],
            self.corners[corners[1]],
            self.corners[corners[2]],
            self.corners[corners[3]],
            direction,
        );
        (
            self.edges[edges[0]],
            self.edges[edges[1]],
            self.edges[edges[2]],
            self.edges[edges[3]],
        ) = CubieCube::cycle(
            self.edges[edges[0]],
            self.edges[edges[1]],
            self.edges[edges[2]],
            self.edges[edges[3]],
            direction,
        );

        *self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phase1Cube {
    pub flips: u16,          // 0..2048
    pub twists: u16,         // 0..2187
    pub ud_permutation: u16, // 0..495
}

// In ud_permutation, 425 is the solved index.
impl Cube for Phase1Cube {
    fn is_solved(&self) -> bool {
        self.flips == 0 && self.twists == 0 && self.ud_permutation == 425
    }

    fn apply_move(&mut self, mve: Move) -> Self {
        self.twists =
            CORNER_ORIENTATION_COORDINATE[self.twists as usize * AMOUNT_OF_MOVES + mve.index()];
        self.flips =
            EDGE_ORIENTATION_COORDINATE[self.flips as usize * AMOUNT_OF_MOVES + mve.index()];
        self.ud_permutation =
            UD_PERMUTATION_COORDINATE[self.ud_permutation as usize * AMOUNT_OF_MOVES + mve.index()];

        *self
    }
}
impl Phase1Cube {
    pub fn new() -> Self {
        Phase1Cube {
            flips: 0,
            twists: 0,
            ud_permutation: 425,
        }
    }
    pub fn index(&self) -> u64 {
        self.twists as u64 * 495 * 2048 + self.flips as u64 * 495 + self.ud_permutation as u64
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phase2Cube {
    pub corners: u16, // 0..40320 (8!)
    pub edges: u16,   // 0..40320 (8!)
    pub ud_slice: u8, // 0..23 (4!)
}
impl Cube for Phase2Cube {
    fn is_solved(&self) -> bool {
        self.corners == 0 && self.edges == 0 && self.ud_slice == 0
    }
    fn apply_move(&mut self, mve: Move) -> Self {
        self.corners = CORNER_PERMUTATION_COORDINATE
            [self.corners as usize * AMOUNT_OF_STAGE_2_MOVES + mve.stage_2_index()];
        self.edges = EDGE_PERMUTATION_COORDINATE
            [self.edges as usize * AMOUNT_OF_STAGE_2_MOVES + mve.stage_2_index()];
        if matches!(mve, |Move::R2| Move::L2 | Move::F2 | Move::B2) {
            self.ud_slice = UD_PHASE_2_PERMUTATION_COORDINATE[self.ud_slice as usize * 4
                + match mve {
                    Move::R2 => 0,
                    Move::L2 => 1,
                    Move::F2 => 2,
                    Move::B2 => 3,
                    _ => unreachable!(),
                }];
        }

        *self
    }
}

impl Phase2Cube {
    pub fn new() -> Self {
        Phase2Cube {
            corners: 0,
            edges: 0,
            ud_slice: 0,
        }
    }
}

impl From<CubieCube> for Phase1Cube {
    fn from(value: CubieCube) -> Self {
        let mut phase_1 = Phase1Cube::new();
        for (i, &corner) in value.corners[0..=6].iter().enumerate() {
            phase_1.twists += match corner.orientation {
                CornerOrientation::Normal => 0,
                CornerOrientation::OneTwist => 1,
                CornerOrientation::TwoTwist => 2,
            } * 3_u16.pow(i as u32);
        }

        for (i, &edge) in value.edges[0..=10].iter().enumerate() {
            phase_1.flips += edge.orientation as u16 * 2_u16.pow(i as u32);
        }

        phase_1.ud_permutation = get_ud_slice_combination([
            value.where_is_edge(EdgePiece::BL) as u64,
            value.where_is_edge(EdgePiece::BR) as u64,
            value.where_is_edge(EdgePiece::FL) as u64,
            value.where_is_edge(EdgePiece::FR) as u64,
        ]) as u16;

        phase_1
    }
}

impl From<CubieCube> for Phase2Cube {
    fn from(value: CubieCube) -> Self {
        assert!(
            Phase1Cube::from(value).is_solved(),
            "A phase 2 cube must be built on a phase 1 solved cube"
        );

        let mut phase_2 = Phase2Cube::new();
        phase_2.corners = permutation_index(
            &[
                CubieCube::get_solved_index_corner(value.corners[0].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[1].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[2].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[3].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[4].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[5].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[6].piece) as u64,
                CubieCube::get_solved_index_corner(value.corners[7].piece) as u64,
            ],
            8,
        ) as u16;
        phase_2.edges = permutation_index(
            &[
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[0].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[1].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[2].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[3].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[8].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[9].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[10].piece,
                )) as u64,
                CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                    value.edges[11].piece,
                )) as u64,
            ],
            8,
        ) as u16;

        phase_2.ud_slice = permutation_index(
            &[
                CubieCube::get_solved_index_edge(value.edges[4].piece) as u64 - 4,
                CubieCube::get_solved_index_edge(value.edges[5].piece) as u64 - 4,
                CubieCube::get_solved_index_edge(value.edges[6].piece) as u64 - 4,
                CubieCube::get_solved_index_edge(value.edges[7].piece) as u64 - 4,
            ],
            4,
        ) as u8;

        phase_2
    }
}
#[cfg(test)]
mod tests {
    use log::debug;

    use super::*;
    #[test]
    fn test_cubie_cube_apply_move() {
        let cube = CubieCube::new();
        assert!(cube.clone().apply_move(Move::U1).apply_move(Move::U3) == cube);
        assert!(cube.clone().apply_move(Move::D1).apply_move(Move::D3) == cube);
        assert!(cube.clone().apply_move(Move::F1).apply_move(Move::F3) == cube);
        assert!(cube.clone().apply_move(Move::B1).apply_move(Move::B3) == cube);
        assert!(cube.clone().apply_move(Move::R1).apply_move(Move::R3) == cube);
        assert!(cube.clone().apply_move(Move::L1).apply_move(Move::L3) == cube);
    }

    #[test]
    fn test_phase_1_cube_apply_move() {
        let cube = Phase1Cube::new();
        assert!(cube.clone().apply_move(Move::U1).apply_move(Move::U3) == cube);
        assert!(cube.clone().apply_move(Move::D1).apply_move(Move::D3) == cube);
        assert!(cube.clone().apply_move(Move::F1).apply_move(Move::F3) == cube);
        assert!(cube.clone().apply_move(Move::B1).apply_move(Move::B3) == cube);
        assert!(cube.clone().apply_move(Move::R1).apply_move(Move::R3) == cube);
        assert!(cube.clone().apply_move(Move::L1).apply_move(Move::L3) == cube);
    }

    #[test]
    fn test_phase_2_cube_apply_move() {
        let cube = Phase2Cube::new();
        assert!(cube.clone().apply_move(Move::U2).apply_move(Move::U2) == cube);
        assert!(cube.clone().apply_move(Move::D2).apply_move(Move::D2) == cube);
        assert!(cube.clone().apply_move(Move::F2).apply_move(Move::F2) == cube);
        assert!(cube.clone().apply_move(Move::B2).apply_move(Move::B2) == cube);
        assert!(cube.clone().apply_move(Move::R2).apply_move(Move::R2) == cube);
        assert!(cube.clone().apply_move(Move::L2).apply_move(Move::L2) == cube);
    }

    #[test]
    fn solved_test() {
        assert!(CubieCube::new().is_solved());
        assert!(Phase1Cube::new().is_solved());
        assert!(Phase2Cube::new().is_solved());

        assert!(Phase1Cube::from(CubieCube::new()).is_solved());
        assert!(Phase2Cube::from(CubieCube::new()).is_solved());
    }

    #[test]
    fn rotations_test() {
        let cube = CubieCube::from_equator(SliceLayers::M);
        let phase_1 = Phase1Cube::from(cube);
        debug!("{:?}", phase_1);
        let cube = CubieCube::from_equator(SliceLayers::S);
        let phase_1 = Phase1Cube::from(cube);
        debug!("{:?}", phase_1);
    }

    #[test]
    fn test_from_colors_valid_case() {
        for _ in 0..100 {
            for i in 1..18 {
                let moves = Move::generate_scramble(i);
                let cube = CubieCube::new().apply_moves(moves);
                assert_eq!(CubieCube::from_colors(cube.to_colors()).unwrap(), cube);
            }
        }
    }
}
