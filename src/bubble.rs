use ecolor::Color32;
use egui::{Response, Ui};
use egui_plot::{Bar, BarChart, Plot};

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
            }
            // More work to do, back to the beginning!
            self.data[self.current_loc].fill = Color32::RED;
            self.current_loc = 0;
            self.changed = false;
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
