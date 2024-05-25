use ecolor::Color32;
use egui::*;
use egui_plot::{Bar, BarChart, Plot};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct BubbleSort {
    data: Vec<Bar>,
    current_loc: usize,
    changed: bool,
    finished: bool,
}

impl BubbleSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let mut sort = BubbleSort {
            data,
            current_loc: 0,
            changed: false,
            finished: false,
        };
        sort.data[0].fill = Color32::GREEN;
        sort
    }

    pub fn step(&mut self) {
        // End conditions
        if self.finished {
            return;
        };
        if self.current_loc + 1 == self.data.len() {
            if !self.changed {
                // Nothing more to do
                self.finished = true;
                return;
            } else {
                // More work to do, back to the beginning!
                self.data[self.current_loc].fill = Color32::RED;
                self.current_loc = 0;
                self.changed = false;
            }
        }
        if self.data[self.current_loc].value > self.data[self.current_loc + 1].value {
            // Swap them around
            let temp = self.data[self.current_loc].value;
            self.data[self.current_loc].value = self.data[self.current_loc + 1].value;
            self.data[self.current_loc + 1].value = temp;
            // Note that we've had to make a change
            self.changed = true;
        }
        // Update location
        self.data[self.current_loc].fill = Color32::RED;
        self.current_loc += 1;
        self.data[self.current_loc].fill = Color32::GREEN;
    }

    // Make this a trait!
    pub fn plot_chart(&self, ui: &mut Ui) -> Response {
        let chart = BarChart::new(self.data.clone()).name("Current State");
        Plot::new("Normal Distribution Demo")
            .clamp_grid(true)
            .y_axis_width(3)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
}

// TODO: Learn how to disable the state saving.  Don't want it.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SortApp {
    #[serde(skip)]
    bubble_sort: BubbleSort,
}

fn make_bar_vec(size: usize) -> Vec<Bar> {
    // Produce a randomly shuffled vector of numbers first
    let mut rng = thread_rng();
    let mut numbers: Vec<usize> = (0..size).collect();
    numbers.shuffle(&mut rng);
    println!("Starting State: {:?}", numbers);

    // Now turn it in to a vector of BarChart Bars
    let mut bars: Vec<Bar> = vec![];

    for (index, value) in numbers.iter().enumerate() {
        let mut bar = Bar::new(index as f64, *value as f64);
        bar.fill = Color32::RED;
        bars.push(bar);
    }

    bars
}

impl Default for SortApp {
    fn default() -> Self {
        Self {
            bubble_sort: BubbleSort::new(make_bar_vec(100)),
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

        Default::default()
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
                    "https://github.com/twirrim/sortweb",
                    "Source code."
                ));

                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Sort Visualiser");

            ui.separator();
            // Step the bubble sort
            self.bubble_sort.step();

            self.bubble_sort.plot_chart(ui);

            // And update!
            ui.ctx().request_repaint();
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
