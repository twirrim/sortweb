use ecolor::Color32;

use egui_plot::{Bar};

pub struct BubbleSort {
    data: Vec<Bar>,
    cursor: usize,
    changed: bool,
    finished: bool,
}

impl BubbleSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let mut sort = BubbleSort {
            data,
            cursor: 0,
            changed: false,
            finished: false,
        };
        sort.data[0].fill = Color32::GREEN;
        sort
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn data(&self) -> Vec<Bar> {
        self.data.clone()
    }

    pub fn calculate_distance(&self) -> f64 {
        let mut distance = 0.0;
        for entry in &self.data {
            distance += (entry.argument - entry.value).abs();
        }
        distance / self.data.len() as f64
    }

    pub fn step(&mut self) {
        // End conditions
        if self.finished {
            return;
        };
        if self.cursor + 1 == self.data.len() {
            if !self.changed {
                // Nothing more to do
                self.finished = true;
                self.data[self.cursor].fill = Color32::RED;
                return;
            }
            // More work to do, back to the beginning!
            self.data[self.cursor].fill = Color32::RED;
            self.cursor = 0;
            self.changed = false;
        }
        if self.data[self.cursor].value > self.data[self.cursor + 1].value {
            // Swap them around
            let temp = self.data[self.cursor].value;
            self.data[self.cursor].value = self.data[self.cursor + 1].value;
            self.data[self.cursor + 1].value = temp;
            // Note that we've had to make a change
            self.changed = true;
        }
        // Update location
        self.data[self.cursor].fill = Color32::RED;
        self.cursor += 1;
        self.data[self.cursor].fill = Color32::GREEN;
    }
}
