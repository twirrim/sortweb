use crate::{Comparison, Data};

pub struct ShellSort {
    data: Data,
    gap: usize,
    i: usize,
    j: usize,
    finished: bool,
}

impl ShellSort {
    pub fn new(data: Data) -> Self {
        let len = data.len();
        let mut sort = Self {
            data,
            gap: len / 2,
            i: 0,
            j: 0,
            finished: false,
        };
        sort.data.set_to_green(sort.i);
        sort
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn data(&self) -> Data {
        self.data.clone()
    }

    pub fn step(&mut self) {
        if self.finished {
            return;
        }

        if self.gap == 0 {
            self.finished = true;
            self.data.restore_base_colours();
            return;
        }

        if self.i < self.data.len() {
            if self.j >= self.gap
                && self.data.compare(self.j - self.gap, self.j) == Comparison::Greater
            {
                self.data.swap(self.j - self.gap, self.j);
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

        self.data.restore_base_colours();
        self.data.set_to_green(self.i);
        self.data.set_to_yellow(self.j);
    }
}
