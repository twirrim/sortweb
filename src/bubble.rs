use crate::{Comparison, Data};

pub struct BubbleSort {
    data: Data,
    cursor: usize,
    changed: bool,
    finished: bool,
}

impl BubbleSort {
    pub fn new(data: Data) -> Self {
        let mut sort = BubbleSort {
            data,
            cursor: 0,
            changed: false,
            finished: false,
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
        if self.cursor + 1 == self.data.len() {
            if !self.changed {
                // Nothing more to do
                self.finished = true;
                self.data.restore_base_colours();
                return;
            }
            // More work to do, back to the beginning!
            self.data.restore_base_colours();
            self.cursor = 0;
            self.changed = false;
        }
        match self.data.compare(self.cursor, self.cursor + 1) {
            Comparison::Equal | Comparison::Less => {}
            Comparison::Greater => {
                self.data.swap(self.cursor, self.cursor + 1);
                self.changed = true;
            }
        }
        // Update location
        self.data.restore_base_colours();
        self.cursor += 1;
        self.data.set_to_green(self.cursor);
    }
}
