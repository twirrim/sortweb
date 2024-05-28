use ecolor::Color32;

use egui_plot::Bar;

pub struct ShellSort {
    data: Vec<Bar>,
    gap: usize,
    i: usize,
    j: usize,
    finished: bool,
}

impl ShellSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let len = data.len();
        let mut sort = Self {
            data,
            gap: len / 2,
            i: 0,
            j: 0,
            finished: false,
        };
        sort.data[sort.i].fill = Color32::GREEN;
        sort
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn data(&self) -> Vec<Bar> {
        self.data.clone()
    }

    pub fn step(&mut self) {
        if self.finished {
            return;
        }

        if self.gap == 0 {
            self.finished = true;
            for bar in &mut self.data {
                bar.fill = Color32::RED;
            }
            return;
        }

        if self.i < self.data.len() {
            if self.j >= self.gap && self.data[self.j - self.gap].value > self.data[self.j].value {
                let temp = self.data[self.j - self.gap].value;
                self.data[self.j - self.gap].value = self.data[self.j].value;
                self.data[self.j].value = temp;
                self.j -= self.gap;
            } else {
                self.i += 1;
                self.j = self.i;
            }
        } else {
            self.gap /= 2;
            self.i = self.gap;
            self.j = self.i;
        }
        // Loop through and set colors
        for (index, bar) in self.data.iter_mut().enumerate() {
            if index == self.j {
                bar.fill = Color32::YELLOW;
            } else if index == self.i {
                bar.fill = Color32::GREEN;
            } else {
                bar.fill = Color32::RED;
            }
        }
    }
}
