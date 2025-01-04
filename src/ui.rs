use eframe::egui::{self, vec2, Button};

use crate::{
    cube::{Cube, CubieCube},
    moves::Move,
    piece::{Color, Face, TurnDirection},
    solver::Solver,
};

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_max_inner_size([1220.0, 900.0])
            .with_min_inner_size([1220.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native("", options, Box::new(|_cc| Ok(Box::<App>::default())))
}

struct Colors([Color; 54]);

impl Default for Colors {
    fn default() -> Self {
        Colors([
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Green,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Orange,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
        ])
    }
}

#[derive(Default)]
struct App {
    colors: Colors,
    selected_color: Color,
}

impl App {
    pub fn render_face(&mut self, ui: &mut egui::Ui, face: Face) {
        ui.vertical(|ui| {
            for i in 0..3 {
                ui.horizontal(|ui| {
                    for j in 0..3 {
                        let btn = ui.add(
                            Button::new(" ")
                                .fill(self.colors.0[face.index() * 9 + i * 3 + j])
                                .min_size(vec2(80.0, 80.0)),
                        );
                        if btn.clicked() {
                            self.colors.0[face.index() * 9 + i * 3 + j] = self.selected_color;
                        }
                        if btn.middle_clicked() {
                            self.selected_color = self.colors.0[face.index() * 9 + i * 3 + j];
                        }
                    }
                });
            }
        });
    }

    pub fn color_picker(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.set_min_size(vec2(80.0, 80.0));
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_min_size(vec2(80.0, 80.0));
                });
                ui.vertical(|ui| {
                    let btn = ui.add(
                        Button::new(" ")
                            .min_size(vec2(80.0, 80.0))
                            .fill(self.selected_color),
                    );

                    if btn.clicked() {
                        self.selected_color = match self.selected_color {
                            Color::White => Color::Blue,
                            Color::Blue => Color::Red,
                            Color::Red => Color::Green,
                            Color::Green => Color::Orange,
                            Color::Orange => Color::Yellow,
                            Color::Yellow => Color::White,
                        };
                    }
                    if btn.secondary_clicked() {
                        self.selected_color = match self.selected_color {
                            Color::White => Color::Yellow,
                            Color::Blue => Color::White,
                            Color::Red => Color::Blue,
                            Color::Green => Color::Red,
                            Color::Orange => Color::Green,
                            Color::Yellow => Color::Orange,
                        }
                    }
                });
                ui.vertical(|ui| {
                    ui.set_min_size(vec2(80.0, 80.0));
                })
            });
            ui.horizontal(|ui| {
                ui.set_min_size(vec2(80.0, 80.0));
            });
        });
    }

    pub fn add_move_btn(&mut self, ui: &mut egui::Ui, text: &str, face: Face) {
        let btn = ui.add(Button::new(text).min_size(vec2(80.0, 80.0)));
        if btn.clicked() {
            self.colors.0 = CubieCube::from_colors(self.colors.0)
                .unwrap()
                .apply_move(Move::from_face_direction(face, TurnDirection::CW))
                .to_colors();
        }
        if btn.secondary_clicked() {
            self.colors.0 = CubieCube::from_colors(self.colors.0)
                .unwrap()
                .apply_move(Move::from_face_direction(face, TurnDirection::CCW))
                .to_colors();
        }
        if btn.middle_clicked() {
            self.colors.0 = CubieCube::from_colors(self.colors.0)
                .unwrap()
                .apply_move(Move::from_face_direction(face, TurnDirection::DOUBLE))
                .to_colors();
        }
    }

    pub fn control(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let btn = ui.add(Button::new("Solve").min_size(vec2(80.0, 80.0)));
                if btn.clicked() {
                    let cube = CubieCube::from_colors(self.colors.0).unwrap();
                    println!("Solution: {:?}", Solver::solve(cube));
                }

                let btn = ui.add(Button::new("Scramble").min_size(vec2(80.0, 80.0)));
                if btn.clicked() {
                    let cube = CubieCube::new().apply_moves(Move::generate_scramble(18));
                    self.colors.0 = cube.to_colors()
                }

                let btn = ui.add(Button::new("Reset").min_size(vec2(80.0, 80.0)));
                if btn.clicked() {
                    self.colors = Colors::default();
                }
            });
            ui.horizontal(|ui| {
                self.add_move_btn(ui, "R", Face::R);
                self.add_move_btn(ui, "U", Face::U);
                self.add_move_btn(ui, "F", Face::F);
            });

            ui.horizontal(|ui| {
                self.add_move_btn(ui, "L", Face::L);
                self.add_move_btn(ui, "D", Face::D);
                self.add_move_btn(ui, "B", Face::B);
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("cube")
                .min_col_width(300.0)
                .min_row_height(300.0)
                .show(ui, |ui| {
                    ui.label("");
                    self.render_face(ui, Face::U);
                    ui.label("");
                    ui.label("");
                    ui.end_row();

                    self.render_face(ui, Face::L);
                    self.render_face(ui, Face::F);
                    self.render_face(ui, Face::R);
                    self.render_face(ui, Face::B);
                    ui.end_row();

                    ui.label("");
                    self.render_face(ui, Face::D);
                    self.color_picker(ui);
                    self.control(ui);
                    ui.end_row();
                });
        });
    }
}
