use crate::{Comparison, Data};

#[derive(Debug, Clone)]
enum Phase {
    BuildingHeap,
    SortingHeap,
    HeapifyDown,
}

#[derive(Debug, Clone)]
pub struct HeapSort {
    data: Data,
    size: usize,
    phase: Phase,
    previous_phase: Phase,
    i: usize,
    j: usize,
    heap_root: usize,
    heap_end: usize,
    finished: bool,
}

impl HeapSort {
    pub fn new(data: Data) -> Self {
        let size = data.len();
        let mut sort = Self {
            data,
            size,
            phase: Phase::BuildingHeap,
            previous_phase: Phase::BuildingHeap, // doesn't matter what we choose
            i: size / 2,
            j: 0,
            heap_root: 0,
            heap_end: 0,
            finished: false,
        };
        sort.data.restore_base_colours();
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

        match self.phase {
            Phase::HeapifyDown => {
                let left = 2 * self.heap_root + 1;
                let right = 2 * self.heap_root + 2;
                let mut largest = self.heap_root;

                if left < self.heap_end && self.data.compare(left, largest) == Comparison::Greater {
                    largest = left;
                }

                if right < self.heap_end && self.data.compare(right, largest) == Comparison::Greater
                {
                    largest = right;
                }
                if largest == self.heap_root {
                    self.phase = self.previous_phase.clone();
                    return;
                }

                self.data.swap(self.heap_root, largest);
                self.heap_root = largest;
            }
            Phase::BuildingHeap => {
                if self.i > 0 {
                    self.i -= 1;
                    self.previous_phase = Phase::BuildingHeap;
                    self.heap_root = self.i;
                    self.heap_end = self.size;
                    self.phase = Phase::HeapifyDown;
                } else {
                    self.phase = Phase::SortingHeap;
                    self.j = self.size - 1;
                }
            }
            Phase::SortingHeap => {
                if self.j > 0 {
                    self.data.swap(0, self.j);
                    self.j -= 1;

                    self.previous_phase = Phase::SortingHeap;
                    self.heap_root = 0;
                    self.heap_end = self.j + 1;
                    self.phase = Phase::HeapifyDown;
                } else {
                    self.finished = true;
                    self.data.restore_base_colours();
                    return;
                }
            }
        }
        // Colour things
        self.data.restore_base_colours();
        self.data.set_to_green(self.i);
        self.data.set_to_yellow(self.j);
    }
}
