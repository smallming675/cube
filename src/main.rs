#![allow(dead_code)]

use clap::{Args, Parser, Subcommand, ValueEnum};
use cube::{Cube, CubieCube, Phase1Cube, Phase2Cube};
use log::info;
use moves::Move;
use solver::Solver;

mod cache;
mod cube;
mod misc;
mod moves;
mod piece;
mod solver;
mod ui;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize data needed for the solver
    InitCache(InitArgs),
    /// Generate a scramble of a given length
    GenScramble { length: Option<u16> },
    /// Solve a cube given a scramble
    Solve(SolveArgs),
    /// Benchmarks the solver by solving a given amount of cubes
    Benchmark(BenchmarkArgs),
    /// Provides a GUI for the user to input the cube
    Ui,
}

#[derive(Args, Debug)]
struct SolveArgs {
    // The scramble in move notation
    scramble: String,

    #[arg(value_enum)]
    phase: Option<SolvePhase>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SolvePhase {
    Phase1,
    Phase2,
}

#[derive(Args, Debug)]
struct InitArgs {
    #[arg(value_enum)]
    mode: Option<InitMode>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum InitMode {
    Phase1,
    Phase2,
    UDSliceCombinations,
    UDPhase2Permutations,
    EdgePermutations,
    CornerPermutations,
    UDPermutations,
    CornerOrientation,
    EdgeOrientation,
    BitLookupTable,
}

#[derive(Args, Debug)]
struct BenchmarkArgs {
    /// The amount of cubes to solve
    #[arg(short, long, default_value = "100")]
    amount: usize,

    // Amount of moves used to sramble the cube
    #[arg(short, long, default_value = "18")]
    length: usize,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Ui => {
            ui::run().unwrap();
        }
        Commands::Benchmark(args) => {
            let amount = args.amount;
            let length = args.length;

            info!("Solving {} cubes...", amount);
            let mut elaps = 0;
            let mut moves = 0;
            let mut max_time = 0;
            let mut max_moves = 0;

            for _ in 0..amount {
                let mut cube = CubieCube::new();
                let scramble = Move::generate_scramble(length);
                info!("Scramble: {:?}\n", scramble);
                for mve in scramble.iter() {
                    cube.apply_move(*mve);
                }

                let start = std::time::Instant::now();
                let solution = Solver::solve(cube);
                let elapsed = start.elapsed();
                elaps += elapsed.as_millis();
                moves += solution.len() as u128;

                if elapsed.as_millis() > max_time {
                    max_time = elapsed.as_millis();
                }

                if solution.len() > max_moves {
                    max_moves = solution.len();
                }

                info!("Solution: {:?} [{} moves]", solution, solution.len());
                info!("Elapsed: {:?}\n", elapsed);
            }

            info!("Average time taken: {} ms", elaps as f64 / amount as f64);
            info!("Average move count: {} moves", moves as f64 / amount as f64);
            info!("Maxmimum time taken: {} ms", max_time as f64);
            info!("Maxmimum move count: {} moves", max_moves as f64);
        }
        Commands::InitCache(args) => {
            if let Some(mode) = args.mode {
                match mode {
                    InitMode::Phase1 => cache::init_phase_1_corners_edges_ud_table(),
                    InitMode::Phase2 => cache::init_phase_2_corners_edges_ud_table(),
                    InitMode::UDSliceCombinations => cache::init_ud_slice_combinations_table(),
                    InitMode::UDPhase2Permutations => cache::init_ud_phase_2_permutation_table(),
                    InitMode::EdgePermutations => cache::init_edge_permutation_coordinate_table(),
                    InitMode::CornerPermutations => cache::init_corner_permutation_table(),
                    InitMode::UDPermutations => cache::init_ud_phase_2_permutation_table(),
                    InitMode::CornerOrientation => cache::init_corner_orientation_table(),
                    InitMode::EdgeOrientation => cache::init_edge_orientation_table(),
                    InitMode::BitLookupTable => cache::init_bit_lookup_table(),
                }
            } else {
                cache::init_cache();
            }
        }
        Commands::GenScramble { length } => {
            let length = if let Some(length) = length {
                length
            } else {
                18
            };

            print!("\"");
            print!(
                "{}",
                Move::generate_scramble(length as usize)
                    .iter()
                    .map(|mve| mve.display())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            info!("\"");
        }
        Commands::Solve(args) => {
            let scramble = args.scramble;
            let mut cube = CubieCube::new();
            let scramble_moves = Move::from_notations(&scramble).unwrap();
            for mve in scramble_moves.iter() {
                cube.apply_move(*mve);
            }
            if let Some(phase) = args.phase {
                match phase {
                    SolvePhase::Phase1 => Solver::phase_1(Phase1Cube::from(cube)),
                    SolvePhase::Phase2 => {
                        let solution = Solver::phase_2(Phase2Cube::from(cube));
                        info!("Phase 2 Solution: {:?}", solution);
                        return;
                    }
                };
            } else {
                let solution = Solver::solve(cube);
                info!("Solution: {:?} [{} moves]", solution, solution.len());
            }
        }
    }
}
