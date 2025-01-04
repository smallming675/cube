use crate::{
    cube::{Cube, Phase1Cube, Phase2Cube},
    misc::{get_ud_slice_combination, inverse_permutation_index, permutation_index, pick},
    piece::{EdgePiece, Face},
};
use dirs::data_dir;
use log::info;
use once_cell::sync::Lazy;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use crate::{cube::CubieCube, moves::Move};

const AMOUNT_PHASE_1_POSITIONS: usize = 2_217_093_120; // 2^11 * 3^7 * 495
const AMOUNT_CORNER_ORIENTATIONS: usize = 2187; //3^7, we ignore the last corner
const AMOUNT_EDGE_ORIENTATIONS: usize = 2048; // 2^11, again we ignore the last edge
const AMOUNT_UD_SLICE_PERMUTATIONS: usize = 495;
const AMOUNT_CORNER_PERMUTATIONS: usize = 40320; // 8!
const AMOUNT_PHASE_2_EDGE_PERMUTATIONS: usize = 40320;
// 8!. We ignore the slice edges since they should always be in the equator
const AMOUNT_UD_SLICE_PHASE_2_PERMUTATIONS: usize = 24;

fn load_move_table<T: serde::de::DeserializeOwned>(file_name: &str) -> Vec<T> {
    let path = data_dir()
        .expect(&format!("User data directory should exist"))
        .join(file_name);
    let data = fs::read(path).expect(&format!("{file_name} should exist"));
    bincode::deserialize(&data).expect(&format!("{file_name} should contain a valid table"))
}

fn save_table<T: serde::Serialize>(table: Vec<T>, file_name: &str) {
    fs::write(
        data_dir()
            .expect("User data directory should exist")
            .join(file_name),
        bincode::serialize(&table).expect(&format!("{file_name} should contain a vaild table")),
    )
    .expect(&format!("File {file_name} should be writable"))
}

pub static PHASE_2_CORNERS_MOVE_TABLE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/phase_2_corners_move_table.bin"));
pub static PHASE_2_EDGES_UD_MOVE_TABLE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/phase_2_edges_ud_move_table.bin"));
pub static PHASE_1_CORNERS_MOVE_TABLE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/phase_1_corners_move_table.bin"));
pub static PHASE_1_EDGES_UD_MOVE_TABLE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/phase_1_edges_ud_move_table.bin"));
pub static UD_SLICE_COMBINATIONS: Lazy<Vec<[u8; 4]>> =
    Lazy::new(|| load_move_table::<[u8; 4]>("cube/ud_slice_combinations.bin"));
pub static UD_PHASE_2_PERMUTATION_COORDINATE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/ud_phase_2_permutation_coordinate_table.bin"));
pub static EDGE_PERMUTATION_COORDINATE: Lazy<Vec<u16>> =
    Lazy::new(|| load_move_table::<u16>("cube/edge_permutation_coordinate_table.bin"));
pub static CORNER_PERMUTATION_COORDINATE: Lazy<Vec<u16>> =
    Lazy::new(|| load_move_table::<u16>("cube/corner_permutation_coordinate_table.bin"));
pub static UD_PERMUTATION_COORDINATE: Lazy<Vec<u16>> =
    Lazy::new(|| load_move_table::<u16>("cube/ud_permutation_coordinate_table.bin"));
pub static CORNER_ORIENTATION_COORDINATE: Lazy<Vec<u16>> =
    Lazy::new(|| load_move_table::<u16>("cube/corner_orientation_coordinate_table.bin"));
pub static EDGE_ORIENTATION_COORDINATE: Lazy<Vec<u16>> =
    Lazy::new(|| load_move_table::<u16>("cube/edge_orientation_coordinate_table.bin"));
pub static BIT_LOOKUP_TABLE: Lazy<Vec<u8>> =
    Lazy::new(|| load_move_table::<u8>("cube/bit_lookup_table.bin"));

pub fn init_cache() {
    info!("Initializing cache...");
    init_edge_orientation_table();
    init_corner_orientation_table();
    init_ud_slice_coordinate_table();
    init_corner_permutation_table();
    init_edge_permutation_coordinate_table();
    init_ud_phase_2_permutation_table();
    init_ud_slice_combinations_table();
    init_phase_1_corners_edges_ud_table();
    init_phase_2_corners_edges_ud_table();
}

pub fn init_bit_lookup_table() {
    let mut table = vec![0u8; 1 << 12];
    for i in 0..1 << 12 {
        let mut count = 0;
        for j in 0..12 {
            if i & (1 << j) != 0 {
                count += 1;
            }
        }
        table[i] = count as u8;
    }

    save_table(table, "cube/bit_lookup_table.bin");
}

pub fn init_phase_2_corners_edges_ud_table() {
    info!("Initializing phase 2 partial table...");
    let mut corners = vec![0u8; AMOUNT_CORNER_PERMUTATIONS];
    let mut edges_ud =
        vec![0u8; AMOUNT_PHASE_2_EDGE_PERMUTATIONS * AMOUNT_UD_SLICE_PHASE_2_PERMUTATIONS];

    let mut queue = VecDeque::new();
    let mut last_depth = 0;
    let initial_cube = Phase2Cube::new();
    queue.push_back((initial_cube, 1, None::<Move>));
    while let Some((cube, depth, last_move)) = queue.pop_front() {
        if depth > 18 {
            break;
        }
        if depth != last_depth {
            info!("depth: {}", depth);
            last_depth = depth;
        }

        for mve in Move::get_all_phase_2_moves() {
            if let Some(last_move) = last_move {
                match (mve.face(), last_move.face()) {
                    (Face::R, Face::L) | (Face::F, Face::B) | (Face::U, Face::D) => continue,
                    _ => {}
                }
                if mve.face() == last_move.face() {
                    continue;
                }
            }

            let new_cube = cube.clone().apply_move(mve);

            let corner_index = new_cube.corners as usize;
            let edge_index = new_cube.edges as usize * AMOUNT_UD_SLICE_PHASE_2_PERMUTATIONS
                + new_cube.ud_slice as usize;

            if corners[corner_index] != 0 && edges_ud[edge_index] != 0 {
                continue;
            }

            if corners[corner_index] == 0 {
                corners[corner_index] = depth as u8;
            }

            if edges_ud[edge_index] == 0 {
                edges_ud[edge_index] = depth as u8;
            }

            queue.push_back((new_cube, depth + 1, Some(mve)));
        }
    }

    corners[initial_cube.corners as usize] = 0;
    edges_ud[initial_cube.edges as usize * AMOUNT_UD_SLICE_PHASE_2_PERMUTATIONS
        + initial_cube.ud_slice as usize] = 0;

    save_table(corners, "cube/phase_2_corners_move_table.bin");
    save_table(edges_ud, "cube/phase_2_edges_ud_move_table.bin");
}
pub fn init_phase_1_corners_edges_ud_table() {
    info!("Initializing phase 1 partial table...");
    let mut corners = vec![0u8; AMOUNT_CORNER_ORIENTATIONS];
    let mut edges_ud = vec![0u8; AMOUNT_EDGE_ORIENTATIONS * AMOUNT_UD_SLICE_PERMUTATIONS];

    let mut queue = VecDeque::new();
    let mut last_depth = 0;
    let initial_cube = Phase1Cube::new();

    queue.push_back((initial_cube, 1, None::<Move>));
    while let Some((cube, depth, last_move)) = queue.pop_front() {
        if depth != last_depth {
            info!("depth: {}", depth);
            last_depth = depth;
        }

        for mve in Move::get_all_moves() {
            if let Some(last_move) = last_move {
                match (mve.face(), last_move.face()) {
                    (Face::R, Face::L) | (Face::F, Face::B) | (Face::U, Face::D) => continue,
                    _ => {}
                }
                if mve.face() == last_move.face() {
                    continue;
                }
            }

            let new_cube = cube.clone().apply_move(mve);

            let twists = new_cube.twists as usize;
            let flips = (new_cube.flips as u64 * AMOUNT_UD_SLICE_PERMUTATIONS as u64
                + new_cube.ud_permutation as u64) as usize;

            if corners[twists] != 0 && edges_ud[flips] != 0 {
                continue;
            }

            if corners[twists] == 0 {
                corners[twists] = depth as u8;
            }

            if edges_ud[flips] == 0 {
                edges_ud[flips] = depth as u8;
            }

            queue.push_back((new_cube, depth + 1, Some(mve)));
        }
    }

    corners[initial_cube.twists as usize] = 0;
    edges_ud[(initial_cube.flips as u64 * AMOUNT_UD_SLICE_PERMUTATIONS as u64
        + initial_cube.ud_permutation as u64) as usize] = 0;

    save_table(corners, "cube/phase_1_corners_move_table.bin");
    save_table(edges_ud, "cube/phase_1_edges_ud_move_table.bin");
}
pub fn init_ud_slice_combinations_table() {
    info!("Initializing UD slice combinations table...");

    let mut combinations = vec![[0u8; 4]; AMOUNT_UD_SLICE_PERMUTATIONS];
    for i in 0..pick(12, 4) {
        let permutation = inverse_permutation_index(i, 4, 12);
        let mut permutation = [
            permutation[0] as u64,
            permutation[1] as u64,
            permutation[2] as u64,
            permutation[3] as u64,
        ];

        permutation.sort();
        combinations[get_ud_slice_combination(permutation) as usize] = [
            permutation[0] as u8,
            permutation[1] as u8,
            permutation[2] as u8,
            permutation[3] as u8,
        ]
    }
    let mut set = HashSet::new();
    for comb in &combinations {
        assert!(set.insert(comb), "Duplicate combination found: {:?}", comb);
    }

    save_table(combinations, "cube/ud_slice_combinations.bin");
}

pub fn init_ud_phase_2_permutation_table() {
    info!("Initializing UD phase 2 permutation coordinate move table...");
    let mut moves = Vec::new();
    for i in 0..AMOUNT_UD_SLICE_PHASE_2_PERMUTATIONS {
        let cube = CubieCube::from_ud_slice_phase_2_permutation(i as u8);
        for mve in [Move::R2, Move::L2, Move::F2, Move::B2] {
            let new_cube = cube.clone().apply_move(mve);
            let permutation = permutation_index(
                &[
                    CubieCube::get_solved_index_edge(new_cube.edges[4].piece) as u64 - 4,
                    CubieCube::get_solved_index_edge(new_cube.edges[5].piece) as u64 - 4,
                    CubieCube::get_solved_index_edge(new_cube.edges[6].piece) as u64 - 4,
                    CubieCube::get_solved_index_edge(new_cube.edges[7].piece) as u64 - 4,
                ],
                4,
            ) as u8;
            moves.push(permutation);
        }
    }
    save_table(moves, "cube/ud_phase_2_permutation_coordinate_table.bin");
}

pub fn init_edge_permutation_coordinate_table() {
    info!("Initializing edge permutation coordinate move table...");
    let mut moves = Vec::new();
    for i in 0..AMOUNT_PHASE_2_EDGE_PERMUTATIONS {
        let cube = CubieCube::from_phase_2_edge_permutation(i as u64);
        for mve in Move::get_all_phase_2_moves() {
            let new_cube = cube.clone().apply_move(mve);
            let permutation = permutation_index(
                &[
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[0].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[1].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[2].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[3].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[8].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[9].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[10].piece,
                    )) as u64,
                    CubieCube::normalize_edge_index(CubieCube::get_solved_index_edge(
                        new_cube.edges[11].piece,
                    )) as u64,
                ],
                8,
            );
            moves.push(permutation as u16);
        }
    }
    save_table(moves, "cube/edge_permutation_coordinate_table.bin");
}
pub fn init_corner_permutation_table() {
    info!("Initializing corner permutation coordinate move table...");
    let mut moves = Vec::new();
    for i in 0..AMOUNT_CORNER_PERMUTATIONS {
        let cube = CubieCube::from_corner_permutation(i as u64);
        for mve in Move::get_all_phase_2_moves() {
            let new_cube = cube.clone().apply_move(mve);
            let permutation = permutation_index(
                &[
                    CubieCube::get_solved_index_corner(new_cube.corners[0].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[1].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[2].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[3].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[4].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[5].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[6].piece) as u64,
                    CubieCube::get_solved_index_corner(new_cube.corners[7].piece) as u64,
                ],
                8,
            );
            moves.push(permutation as u16);
        }
    }
    save_table(moves, "cube/corner_permutation_coordinate_table.bin");
}
pub fn init_ud_slice_coordinate_table() {
    info!("Initializing UD permutation coordinate move table...");
    let mut moves = Vec::new();
    for i in 0..AMOUNT_UD_SLICE_PERMUTATIONS {
        let cube = CubieCube::from_ud_slice_permutation(i as u64);
        for mve in Move::get_all_moves() {
            let new_cube = cube.clone().apply_move(mve);

            let arr = [
                new_cube.where_is_edge(EdgePiece::BL) as u64,
                new_cube.where_is_edge(EdgePiece::BR) as u64,
                new_cube.where_is_edge(EdgePiece::FR) as u64,
                new_cube.where_is_edge(EdgePiece::FL) as u64,
            ];
            moves.push(get_ud_slice_combination(arr) as u16)
        }
    }

    save_table(moves, "cube/ud_permutation_coordinate_table.bin");
}
pub fn init_corner_orientation_table() {
    info!("Initializing corner orientation coordinate move table...");

    let mut moves = Vec::new();
    for i in 0..AMOUNT_CORNER_ORIENTATIONS {
        let cube = CubieCube::from_corner_orientation(i as u64);
        for mve in Move::get_all_moves() {
            let new_cube = cube.clone().apply_move(mve);
            let orientation = new_cube.corners[6].orientation as usize * 729
                + new_cube.corners[5].orientation as usize * 243
                + new_cube.corners[4].orientation as usize * 81
                + new_cube.corners[3].orientation as usize * 27
                + new_cube.corners[2].orientation as usize * 9
                + new_cube.corners[1].orientation as usize * 3
                + new_cube.corners[0].orientation as usize;

            moves.push(orientation as u16);
        }
    }
    save_table(moves, "cube/corner_orientation_coordinate_table.bin");
}
pub fn init_edge_orientation_table() {
    info!("Initializing edge orientation coordinate move table...");

    let mut moves = Vec::new();
    for i in 0..AMOUNT_EDGE_ORIENTATIONS {
        let cube = CubieCube::from_edge_orientation(i as u64);
        for mve in Move::get_all_moves() {
            let new_cube = cube.clone().apply_move(mve);
            let orientation = new_cube.edges[10].orientation as usize * 1024
                + new_cube.edges[9].orientation as usize * 512
                + new_cube.edges[8].orientation as usize * 256
                + new_cube.edges[7].orientation as usize * 128
                + new_cube.edges[6].orientation as usize * 64
                + new_cube.edges[5].orientation as usize * 32
                + new_cube.edges[4].orientation as usize * 16
                + new_cube.edges[3].orientation as usize * 8
                + new_cube.edges[2].orientation as usize * 4
                + new_cube.edges[1].orientation as usize * 2
                + new_cube.edges[0].orientation as usize;
            moves.push(orientation as u16);
        }
    }
    save_table(moves, "cube/edge_orientation_coordinate_table.bin");
}

#[cfg(test)]
mod tests {
    use crate::misc::factorial;
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_permutation_indices_unique() {
        let length = 4;
        let k = 4;
        let mut seen_indices = std::collections::HashSet::new();

        let permutations =
            (0..factorial(length as u64)).map(|i| inverse_permutation_index(i, length, k));

        for perm in permutations {
            let index = permutation_index(&perm, k);
            assert!(
                seen_indices.insert(index),
                "Duplicate index found: {}",
                index
            );
        }
    }

    #[test]
    fn test_inverse_permutation_index() {
        let mut map = HashMap::new();

        (0..pick(4, 4)).for_each(|i| {
            let length = 4;
            let k = 4;
            let index = i;
            let perm = inverse_permutation_index(index, length, k);
            let new_index = permutation_index(&perm, k);
            assert!(
                index == new_index,
                "Expected {} but got {}",
                index,
                new_index
            );
            if let Some(perm) = map.get(&index) {
                panic!("Duplicate permutation found: {:?} and {:?}", perm, perm);
            }
            map.insert(index, perm);
        });
    }
}
