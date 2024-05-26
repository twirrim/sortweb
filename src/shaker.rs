use ecolor::Color32;
use egui::{Response, Ui};
use egui_plot::{Bar, BarChart, Plot};

pub struct ShakerSort {
    data: Vec<Bar>,
    cursor: usize,
    changed: bool,
    finished: bool,
    ascending: bool,
}

impl ShakerSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let mut sort = ShakerSort {
            data,
            cursor: 0,
            changed: false,
            finished: false,
            ascending: true,
        };
        sort.data[0].fill = Color32::GREEN;
        sort
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn calculate_distance(&self) -> f64 {
        let mut distance = 0.0;
        for entry in self.data.iter() {
            distance += (entry.argument - entry.value).abs();
        }
        distance / self.data.len() as f64
    }

    pub fn step(&mut self) {
        // End conditions
        if self.finished {
            return;
        };

        // I think I could do this more D.R.Y., with a bit more thought

        let a = self.cursor;
        if self.ascending {
            if a == self.data.len() - 1 {
                if !self.changed {
                    self.finished = true;
                    self.data[self.cursor].fill = Color32::RED;
                    return;
                }
                // More work to do, go back to the beginning!
                self.data[self.cursor].fill = Color32::RED;
                self.cursor = self.data.len() - 1;
                self.changed = false;
                self.ascending = false;
            }
        } else if a == 0 {
            if !self.changed {
                self.finished = true;
                self.data[self.cursor].fill = Color32::RED;
                return;
            }
            // More work to do, go back towards the end!
            self.data[self.cursor].fill = Color32::RED;
            self.cursor = 0;
            self.changed = false;
            self.ascending = true;
        }

        let mut swap = false;
        let b: usize;
        if self.ascending {
            b = a + 1;
            if self.data[a].value > self.data[b].value {
                swap = true;
            }
        } else {
            b = a - 1;
            if self.data[a].value < self.data[b].value {
                swap = true;
            }
        }

        if swap {
            let temp = self.data[a].value;
            self.data[a].value = self.data[b].value;
            self.data[b].value = temp;
            self.changed = true;
        }

        // Update location
        self.data[self.cursor].fill = Color32::RED;
        if self.ascending {
            self.cursor += 1;
        } else {
            self.cursor -= 1;
        }
        self.data[self.cursor].fill = Color32::GREEN;
    }

    // Make this a trait!
    pub fn plot_chart(&self, ui: &mut Ui) -> Response {
        let chart = BarChart::new(self.data.clone()).name("Shaker Sort");
        Plot::new("Shaker Sort Demo")
            .clamp_grid(true)
            .y_axis_width(3)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
}
