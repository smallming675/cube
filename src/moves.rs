use crate::piece::{Face, TurnDirection};
use rand::Rng;

pub const AMOUNT_OF_MOVES: usize = 18;
pub const AMOUNT_OF_STAGE_2_MOVES: usize = 10;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Move {
    U1,
    U2,
    U3,

    F1,
    F2,
    F3,

    B1,
    B2,
    B3,

    D1,
    D2,
    D3,

    R1,
    R2,
    R3,

    L1,
    L2,
    L3,
}

impl Move {
    pub fn display(&self) -> String {
        match self {
            Move::U1 => "U",
            Move::U2 => "U2",
            Move::U3 => "U'",
            Move::F1 => "F",
            Move::F2 => "F2",
            Move::F3 => "F'",
            Move::B1 => "B",
            Move::B2 => "B2",
            Move::B3 => "B'",
            Move::D1 => "D",
            Move::D2 => "D2",
            Move::D3 => "D'",
            Move::R1 => "R",
            Move::R2 => "R2",
            Move::R3 => "R'",
            Move::L1 => "L",
            Move::L2 => "L2",
            Move::L3 => "L'",
        }
        .to_string()
    }
    pub const fn index(&self) -> usize {
        match self {
            Move::U1 => 0,
            Move::U2 => 1,
            Move::U3 => 2,
            Move::F1 => 3,
            Move::F2 => 4,
            Move::F3 => 5,
            Move::B1 => 6,
            Move::B2 => 7,
            Move::B3 => 8,
            Move::D1 => 9,
            Move::D2 => 10,
            Move::D3 => 11,
            Move::R1 => 12,
            Move::R2 => 13,
            Move::R3 => 14,
            Move::L1 => 15,
            Move::L2 => 16,
            Move::L3 => 17,
        }
    }
    pub const fn stage_2_index(&self) -> usize {
        match self {
            Move::U1 => 0,
            Move::U2 => 1,
            Move::U3 => 2,
            Move::D1 => 3,
            Move::D2 => 4,
            Move::D3 => 5,
            Move::R2 => 6,
            Move::L2 => 7,
            Move::F2 => 8,
            Move::B2 => 9,
            _ => panic!("Invalid stage 2 move"),
        }
    }
    pub fn get_all_phase_2_moves() -> Vec<Move> {
        vec![
            Move::U1,
            Move::U2,
            Move::U3,
            Move::D1,
            Move::D2,
            Move::D3,
            Move::R2,
            Move::L2,
            Move::F2,
            Move::B2,
        ]
    }
    pub fn get_all_moves() -> Vec<Move> {
        vec![
            Move::U1,
            Move::U2,
            Move::U3,
            Move::F1,
            Move::F2,
            Move::F3,
            Move::B1,
            Move::B2,
            Move::B3,
            Move::D1,
            Move::D2,
            Move::D3,
            Move::R1,
            Move::R2,
            Move::R3,
            Move::L1,
            Move::L2,
            Move::L3,
        ]
    }

    pub fn face(&self) -> Face {
        match self {
            Move::U1 | Move::U2 | Move::U3 => Face::U,
            Move::F1 | Move::F2 | Move::F3 => Face::F,
            Move::B1 | Move::B2 | Move::B3 => Face::B,
            Move::D1 | Move::D2 | Move::D3 => Face::D,
            Move::R1 | Move::R2 | Move::R3 => Face::R,
            Move::L1 | Move::L2 | Move::L3 => Face::L,
        }
    }

    pub fn direction(&self) -> TurnDirection {
        match self {
            Move::U1 | Move::F1 | Move::B1 | Move::D1 | Move::R1 | Move::L1 => TurnDirection::CW,
            Move::U2 | Move::F2 | Move::B2 | Move::D2 | Move::R2 | Move::L2 => {
                TurnDirection::DOUBLE
            }
            Move::U3 | Move::F3 | Move::B3 | Move::D3 | Move::R3 | Move::L3 => TurnDirection::CCW,
        }
    }

    pub fn generate_phase_2_scramble(length: usize) -> Vec<Move> {
        let mut scramble: Vec<Move> = vec![];
        for _ in 0..length {
            let mut mve = Move::get_all_phase_2_moves()
                [rand::thread_rng().gen_range(0..AMOUNT_OF_STAGE_2_MOVES)];
            if let Some(last) = scramble.last() {
                while last.face() == mve.face() {
                    mve = Move::get_all_phase_2_moves()
                        [rand::thread_rng().gen_range(0..AMOUNT_OF_STAGE_2_MOVES)];
                }
            }
            scramble.push(mve);
        }
        scramble
    }
    pub fn generate_scramble(length: usize) -> Vec<Move> {
        let mut scramble: Vec<Move> = vec![];
        for _ in 0..length {
            let mut mve = Move::random_move();
            if let Some(last) = scramble.last() {
                while last.face() == mve.face() {
                    mve = Move::random_move();
                }
            }
            scramble.push(mve);
        }
        scramble
    }

    pub fn random_move() -> Move {
        Move::get_all_moves()[rand::thread_rng().gen_range(0..18)]
    }

    pub fn inverse(&self) -> Move {
        match self {
            Move::U1 => Move::U3,
            Move::U2 => Move::U2,
            Move::U3 => Move::U1,
            Move::F1 => Move::F3,
            Move::F2 => Move::F2,
            Move::F3 => Move::F1,
            Move::B1 => Move::B3,
            Move::B2 => Move::B2,
            Move::B3 => Move::B1,
            Move::D1 => Move::D3,
            Move::D2 => Move::D2,
            Move::D3 => Move::D1,
            Move::R1 => Move::R3,
            Move::R2 => Move::R2,
            Move::R3 => Move::R1,
            Move::L1 => Move::L3,
            Move::L2 => Move::L2,
            Move::L3 => Move::L1,
        }
    }

    pub fn from_notation(mve: &str) -> Option<Move> {
        Some(match mve {
            "U" => Move::U1,
            "U2" => Move::U2,
            "U'" => Move::U3,

            "F" => Move::F1,
            "F2" => Move::F2,
            "F'" => Move::F3,

            "B" => Move::B1,
            "B2" => Move::B2,
            "B'" => Move::B3,

            "D" => Move::D1,
            "D2" => Move::D2,
            "D'" => Move::D3,

            "R" => Move::R1,
            "R2" => Move::R2,
            "R'" => Move::R3,

            "L" => Move::L1,
            "L2" => Move::L2,
            "L'" => Move::L3,
            _ => return None,
        })
    }

    pub fn from_face_direction(face: Face, direction: TurnDirection) -> Self {
        match (face, direction) {
            (Face::U, TurnDirection::CW) => Move::U1,
            (Face::U, TurnDirection::DOUBLE) => Move::U2,
            (Face::U, TurnDirection::CCW) => Move::U3,
            (Face::D, TurnDirection::CW) => Move::D1,
            (Face::D, TurnDirection::DOUBLE) => Move::D2,
            (Face::D, TurnDirection::CCW) => Move::D3,
            (Face::R, TurnDirection::CW) => Move::R1,
            (Face::R, TurnDirection::DOUBLE) => Move::R2,
            (Face::R, TurnDirection::CCW) => Move::R3,
            (Face::L, TurnDirection::CW) => Move::L1,
            (Face::L, TurnDirection::DOUBLE) => Move::L2,
            (Face::L, TurnDirection::CCW) => Move::L3,
            (Face::F, TurnDirection::CW) => Move::F1,
            (Face::F, TurnDirection::DOUBLE) => Move::F2,
            (Face::F, TurnDirection::CCW) => Move::F3,
            (Face::B, TurnDirection::CW) => Move::B1,
            (Face::B, TurnDirection::DOUBLE) => Move::B2,
            (Face::B, TurnDirection::CCW) => Move::B3,
        }
    }
    pub fn from_notations(moves: &str) -> Option<Vec<Move>> {
        let mut notations = vec![];
        for mve in moves.split_whitespace() {
            notations.push(Move::from_notation(mve));
        }
        notations.into_iter().collect()
    }

    pub fn reduce(moves: &mut Vec<Move>) {
        if moves.len() < 2 {
            return;
        }
        for i in 0..moves.len() - 1 {
            if i + 1 >= moves.len() {
                break;
            }

            if moves[i].face() == moves[i + 1].face() {
                let (first, second) = (moves[i], moves[i + 1]);
                if let Some(direction) = first.direction().combine(second.direction()) {
                    moves[i] = Move::from_face_direction(first.face(), direction);
                }
                moves.remove(i + 1);
            }
        }

        if moves.len() < 3 {
            return;
        }

        for i in 0..moves.len() - 2 {
            if i + 2 >= moves.len() {
                break;
            }
            if moves[i].face().is_opposite_face(&moves[i + 1].face())
                && moves[i].face() == moves[i + 2].face()
            {
                let (first, second) = (moves[i], moves[i + 2]);
                moves.remove(i + 2);
                if let Some(direction) = first.direction().combine(second.direction()) {
                    moves[i] = Move::from_face_direction(first.face(), direction);
                } else {
                    moves.remove(i);
                }
            }
        }
    }
}
