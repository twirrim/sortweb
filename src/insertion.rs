use ecolor::Color32;

use egui_plot::Bar;

pub struct InsertionSort {
    data: Vec<Bar>,
    i_cursor: usize,
    j_cursor: usize,
    finished: bool,
}

impl InsertionSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let mut sort = InsertionSort {
            data,
            i_cursor: 1,
            j_cursor: 1,
            finished: false,
        };
        sort.data[sort.i_cursor].fill = Color32::GREEN;
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
        if self.i_cursor == self.data.len() {
            self.data[self.i_cursor - 1].fill = Color32::RED;
            self.data[self.j_cursor - 1].fill = Color32::RED;
            self.finished = true;
            return;
        }

        if self.j_cursor > 0 && self.data[self.j_cursor].value < self.data[self.j_cursor - 1].value
        {
            let temp = self.data[self.j_cursor].value;
            self.data[self.j_cursor].value = self.data[self.j_cursor - 1].value;
            self.data[self.j_cursor - 1].value = temp;
            self.j_cursor -= 1;
        } else {
            self.i_cursor += 1;
            self.j_cursor = self.i_cursor;
        }
        // Loop through and set colors
        for (index, bar) in self.data.iter_mut().enumerate() {
            if index == self.j_cursor {
                bar.fill = Color32::YELLOW;
            } else if index == self.i_cursor {
                bar.fill = Color32::GREEN;
            } else {
                bar.fill = Color32::RED;
            }
        }
    }
}
