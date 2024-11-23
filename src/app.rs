use crate::bubble::BubbleSort;
use crate::heap::HeapSort;
use crate::insertion::InsertionSort;
use crate::shaker::ShakerSort;
use crate::shell::ShellSort;
use crate::{distance_to_optimal, make_bar_vec, plot_chart};

use egui_extras::{Column, TableBuilder};

// TODO: Learn how to disable the state saving.  Don't want it.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SortApp {
    #[serde(skip)]
    bubble_sort: BubbleSort,
    #[serde(skip)]
    shaker_sort: ShakerSort,
    #[serde(skip)]
    insertion_sort: InsertionSort,
    #[serde(skip)]
    shell_sort: ShellSort,
    #[serde(skip)]
    heap_sort: HeapSort,
}

impl Default for SortApp {
    fn default() -> Self {
        let starting_data = make_bar_vec(150);
        Self {
            bubble_sort: BubbleSort::new(starting_data.clone()),
            shaker_sort: ShakerSort::new(starting_data.clone()),
            insertion_sort: InsertionSort::new(starting_data.clone()),
            shell_sort: ShellSort::new(starting_data.clone()),
            heap_sort: HeapSort::new(starting_data.clone()),
        }
    }
}

impl SortApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        SortApp::default()
    }
}

impl eframe::App for SortApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.add(egui::github_link_file!(
                    "https://github.com/twirrim/sortweb/blob/main/",
                    "Source code."
                ));

                egui::widgets::global_theme_preference_buttons(ui);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Sort");
                    });
                    header.col(|ui| {
                        ui.heading("Avg. Distance from optimal");
                    });
                    header.col(|ui| {
                        ui.heading("");
                    });
                })
                .body(|mut body| {
                    // Bubble Sort
                    body.row(100.0, |mut row| {
                        row.col(|ui| {
                            ui.label("Bubble");
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:.2}",
                                distance_to_optimal(&self.bubble_sort.data())
                            ));
                        });
                        row.col(|ui| {
                            self.bubble_sort.step();
                            plot_chart(ui, "bubble sort", &self.bubble_sort.data());
                        });
                    });
                    // Shaker Sort
                    body.row(100.0, |mut row| {
                        row.col(|ui| {
                            ui.label("Shaker");
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:.2}",
                                distance_to_optimal(&self.shaker_sort.data())
                            ));
                        });
                        row.col(|ui| {
                            self.shaker_sort.step();
                            plot_chart(ui, "shaker sort", &self.shaker_sort.data());
                        });
                    });
                    // Insertion Sort
                    body.row(100.0, |mut row| {
                        row.col(|ui| {
                            ui.label("Insertion");
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:.2}",
                                distance_to_optimal(&self.insertion_sort.data())
                            ));
                        });
                        row.col(|ui| {
                            self.insertion_sort.step();
                            plot_chart(ui, "insertion sort", &self.insertion_sort.data());
                        });
                    });
                    // Shell Sort
                    body.row(100.0, |mut row| {
                        row.col(|ui| {
                            ui.label("Shell");
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:.2}",
                                distance_to_optimal(&self.shell_sort.data())
                            ));
                        });
                        row.col(|ui| {
                            self.shell_sort.step();
                            plot_chart(ui, "shell sort", &self.shell_sort.data());
                        });
                    });
                    // Heap Sort
                    body.row(100.0, |mut row| {
                        row.col(|ui| {
                            ui.label("Heap");
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:.2}",
                                distance_to_optimal(&self.heap_sort.data())
                            ));
                        });
                        row.col(|ui| {
                            self.heap_sort.step();
                            plot_chart(ui, "heap sort", &self.heap_sort.data());
                        });
                    });
                });

            if !self.bubble_sort.finished()
                || !self.shaker_sort.finished()
                || !self.insertion_sort.finished()
                || !self.shell_sort.finished()
                || !self.heap_sort.finished()
            {
                ui.ctx().request_repaint();
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
