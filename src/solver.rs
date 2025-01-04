use crate::{
    cache::{
        PHASE_1_CORNERS_MOVE_TABLE, PHASE_1_EDGES_UD_MOVE_TABLE, PHASE_2_CORNERS_MOVE_TABLE,
        PHASE_2_EDGES_UD_MOVE_TABLE,
    },
    cube::{Cube, CubieCube, Phase1Cube, Phase2Cube},
    moves::Move,
    piece::Face,
};

use log::{debug, info};
use std::{
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    u64,
};

const MAX_PHASE_1_DEPTH: usize = 12;
const MAX_PHASE_2_DEPTH: usize = 18;

pub struct Solver {}
impl Solver {
    pub fn phase_2_cost(cube: Phase2Cube) -> u64 {
        u64::max(
            PHASE_2_CORNERS_MOVE_TABLE[cube.corners as usize] as u64,
            PHASE_2_EDGES_UD_MOVE_TABLE[cube.edges as usize * 24 + cube.ud_slice as usize] as u64,
        )
    }

    pub fn phase_1_cost(cube: Phase1Cube) -> u64 {
        u64::max(
            PHASE_1_CORNERS_MOVE_TABLE[cube.twists as usize] as u64,
            PHASE_1_EDGES_UD_MOVE_TABLE[cube.flips as usize * 495 + cube.ud_permutation as usize]
                as u64,
        )
    }

    pub fn phase_2(cube: Phase2Cube) -> Vec<Move> {
        let mut bound = Self::phase_2_cost(cube);
        let finished_search = Arc::new(AtomicBool::new(false));
        let mut path = Vec::with_capacity(MAX_PHASE_2_DEPTH);
        loop {
            let cost = Solver::phase_2_search(cube, &mut path, 0, bound, None, &finished_search);

            if cost == 0 {
                return path;
            }
            if cost == u64::MAX {
                return vec![];
            }

            info!("Depth: {}", cost);
            bound = cost;
        }
    }

    pub fn phase_2_search(
        last_position: Phase2Cube,
        path: &mut Vec<Move>,
        cost: u64,
        bound: u64,
        last_move: Option<Move>,
        finished_search: &AtomicBool,
    ) -> u64 {
        if finished_search.load(atomic::Ordering::Relaxed) {
            return u64::MAX;
        }

        let new_cost = cost + Solver::phase_2_cost(last_position);
        if new_cost > bound {
            return new_cost;
        }

        if last_position.is_solved() {
            return 0;
        }

        let mut min = u64::MAX;
        for mve in Move::get_all_phase_2_moves() {
            if let Some(last_move) = last_move {
                match (mve.face(), last_move.face()) {
                    (Face::R, Face::L) | (Face::F, Face::B) | (Face::U, Face::D) => continue,
                    _ => {
                        if mve.face() == last_move.face() {
                            continue;
                        }
                    }
                }
            }

            let new_cube = last_position.clone().apply_move(mve);
            path.push(mve);

            let new_cost =
                Solver::phase_2_search(new_cube, path, cost + 1, bound, Some(mve), finished_search);

            if new_cost == 0 {
                return 0;
            }

            if new_cost < min {
                min = new_cost
            }

            path.pop();
        }
        min
    }

    pub fn phase_1(cube: Phase1Cube) -> Vec<Move> {
        let mut bound = Self::phase_1_cost(cube);
        let finished_search = Arc::new(AtomicBool::new(false));
        let mut path = Vec::with_capacity(MAX_PHASE_1_DEPTH);

        loop {
            let cost = Solver::phase_1_search(cube, &mut path, 0, bound, None, &finished_search);

            if cost == 0 {
                return path;
            }
            if cost == u64::MAX {
                return vec![];
            }

            info!("Depth: {}", cost);
            bound = cost;
        }
    }

    pub fn phase_1_search(
        last_position: Phase1Cube,
        path: &mut Vec<Move>,
        cost: u64,
        bound: u64,
        last_move: Option<Move>,
        finished_search: &AtomicBool,
    ) -> u64 {
        if finished_search.load(atomic::Ordering::Relaxed) {
            return u64::MAX;
        }

        let new_cost = cost + Solver::phase_1_cost(last_position);
        if new_cost > bound {
            return new_cost;
        }

        if last_position.is_solved() {
            return 0;
        }

        let mut min = u64::MAX;
        for mve in Move::get_all_moves() {
            if let Some(last_move) = last_move {
                match (mve.face(), last_move.face()) {
                    (Face::R, Face::L) => continue,
                    (Face::F, Face::B) => continue,
                    (Face::U, Face::D) => continue,
                    _ => {
                        if mve.face() == last_move.face() {
                            continue;
                        }
                    }
                }
            }

            let new_cube = last_position.clone().apply_move(mve);
            path.push(mve);

            let new_cost =
                Solver::phase_1_search(new_cube, path, cost + 1, bound, Some(mve), finished_search);

            if new_cost == 0 {
                return 0;
            }
            if new_cost < min {
                min = new_cost
            }

            path.pop();
        }
        min
    }

    pub fn solve(cube: CubieCube) -> Vec<Move> {
        let mut cube = cube;

        let phase_1_cube = Phase1Cube::from(cube);
        let mut phase_1_solution = Solver::phase_1(phase_1_cube);

        info!(
            "Phase 1 Solution: {:?} [{} moves]\n",
            &phase_1_solution,
            phase_1_solution.len()
        );

        Move::reduce(&mut phase_1_solution);

        for mve in &phase_1_solution {
            cube.apply_move(*mve);
        }

        let phase_2_cube = Phase2Cube::from(cube);
        let phase_2_solution = Solver::phase_2(phase_2_cube);
        let mut solution = phase_1_solution;
        solution.extend(phase_2_solution);

        Move::reduce(&mut solution);
        solution
    }
}

#[cfg(test)]
mod test {
    use log::debug;

    use super::Solver;
    use crate::{
        cube::{Cube, Phase1Cube, Phase2Cube},
        moves::Move,
    };

    #[test]
    fn test_phase_1_cost() {
        assert_eq!(Solver::phase_1_cost(Phase1Cube::new()), 0);
        let mut diffs = 0;
        for _ in 0..10000 {
            for i in 1..=12 {
                let mut cube = Phase1Cube::new();
                for mve in Move::generate_scramble(i) {
                    cube.apply_move(mve);
                }
                let cost = Solver::phase_1_cost(cube);
                diffs += i as i64 - cost as i64;
                assert!(cost <= i as u64);
            }
        }
        debug!("Phase 1 diffs: {}", diffs as f64 / 120000.0);
    }

    #[test]
    fn test_phase_2_cost() {
        assert_eq!(Solver::phase_2_cost(Phase2Cube::new()), 0);

        let mut diffs = 0;
        for _ in 0..10000 {
            for i in 1..=18 {
                let mut cube = Phase2Cube::new();
                for mve in Move::generate_phase_2_scramble(i) {
                    cube.apply_move(mve);
                }
                let cost = Solver::phase_2_cost(cube);

                diffs += i as i64 - cost as i64;
                assert!(cost <= i as u64);
            }
        }
        debug!("Phase 2 diffs: {}", diffs as f64 / 180000.0);
    }
}
