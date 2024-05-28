use ecolor::Color32;

use egui_plot::Bar;

#[derive(Debug, Clone)]
enum Phase {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct ShakerSort {
    data: Vec<Bar>,
    cursor: usize,
    changed: bool,
    finished: bool,
    phase: Phase,
}

impl ShakerSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let mut sort = ShakerSort {
            data,
            cursor: 0,
            changed: false,
            finished: false,
            phase: Phase::Ascending,
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

    pub fn step(&mut self) {
        // End conditions
        if self.finished {
            return;
        };

        match self.phase {
            Phase::Ascending => {
                if self.cursor == self.data.len() - 1 {
                    if !self.changed {
                        self.finished = true;
                        self.data[self.cursor].fill = Color32::RED;
                    }
                    // More work to go, switch direction!
                    self.changed = false;
                    self.phase = Phase::Descending;
                    return;
                }
                if self.data[self.cursor].value > self.data[self.cursor + 1].value {
                    // Swap!
                    let temp = self.data[self.cursor].value;
                    self.data[self.cursor].value = self.data[self.cursor + 1].value;
                    self.data[self.cursor + 1].value = temp;
                    self.changed = true;
                }
                self.cursor += 1;
            }
            Phase::Descending => {
                if self.cursor == 0 {
                    if !self.changed {
                        self.finished = true;
                        self.data[self.cursor].fill = Color32::RED;
                    }
                    // More work to go, switch direction!
                    self.changed = false;
                    self.phase = Phase::Ascending;
                    return;
                }
                if self.data[self.cursor].value < self.data[self.cursor - 1].value {
                    // Swap!
                    let temp = self.data[self.cursor].value;
                    self.data[self.cursor].value = self.data[self.cursor - 1].value;
                    self.data[self.cursor - 1].value = temp;
                    self.changed = true;
                }
                self.cursor -= 1;
            }
        }
        // Set colours
        for (index, bar) in self.data.iter_mut().enumerate() {
            if index == self.cursor {
                bar.fill = Color32::GREEN;
            } else {
                bar.fill = Color32::RED;
            }
        }
    }
}
