use crate::{Comparison, Data};

pub struct InsertionSort {
    data: Data,
    i_cursor: usize,
    j_cursor: usize,
    finished: bool,
}

impl InsertionSort {
    pub fn new(data: Data) -> Self {
        let mut sort = InsertionSort {
            data,
            i_cursor: 1,
            j_cursor: 1,
            finished: false,
        };
        sort.data.set_to_green(sort.i_cursor);
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
        if self.i_cursor == self.data.len() {
            self.data.restore_base_colours();
            self.finished = true;
            return;
        }

        if self.j_cursor > 0
            && self.data.compare(self.j_cursor, self.j_cursor - 1) == Comparison::Less
        {
            self.data.swap(self.j_cursor, self.j_cursor - 1);
            self.j_cursor -= 1;
        } else {
            self.i_cursor += 1;
            self.j_cursor = self.i_cursor;
        }
        // Loop through and set colors
        self.data.restore_base_colours();
        self.data.set_to_green(self.i_cursor);
        self.data.set_to_yellow(self.j_cursor);
    }
}
