use crate::{Comparison, Data};

#[derive(Debug, Clone)]
enum Phase {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct ShakerSort {
    data: Data,
    cursor: usize,
    changed: bool,
    finished: bool,
    phase: Phase,
}

impl ShakerSort {
    pub fn new(data: Data) -> Self {
        let mut sort = ShakerSort {
            data,
            cursor: 0,
            changed: false,
            finished: false,
            phase: Phase::Ascending,
        };
        sort.data.set_to_green(0);
        sort
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn data(&self) -> Data {
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
                        self.data.restore_base_colours();
                    }
                    // More work to go, switch direction!
                    self.changed = false;
                    self.phase = Phase::Descending;
                    return;
                }
                match self.data.compare(self.cursor, self.cursor + 1) {
                    Comparison::Greater => {
                        self.data.swap(self.cursor, self.cursor + 1);
                        self.changed = true;
                    }
                    Comparison::Equal | Comparison::Less => (),
                }
                self.cursor += 1;
            }
            Phase::Descending => {
                if self.cursor == 0 {
                    if !self.changed {
                        self.finished = true;
                        self.data.restore_base_colours();
                    }
                    // More work to go, switch direction!
                    self.changed = false;
                    self.phase = Phase::Ascending;
                    return;
                }
                match self.data.compare(self.cursor, self.cursor - 1) {
                    Comparison::Less => {
                        self.data.swap(self.cursor, self.cursor - 1);
                        self.changed = true;
                    }
                    Comparison::Equal | Comparison::Greater => (),
                }
                self.cursor -= 1;
            }
        }
        // Set colours
        self.data.restore_base_colours();
        self.data.set_to_green(self.cursor);
    }
}
