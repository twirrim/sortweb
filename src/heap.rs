use ecolor::Color32;

use egui_plot::{Bar};

#[derive(Debug, Clone)]
enum Phase {
    BuildingHeap,
    SortingHeap,
    HeapifyDown,
}

#[derive(Debug, Clone)]
pub struct HeapSort {
    data: Vec<Bar>,
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
    pub fn new(data: Vec<Bar>) -> Self {
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
        for entry in &mut sort.data {
            entry.fill = Color32::RED;
        }
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

        match self.phase {
            Phase::HeapifyDown => {
                let left = 2 * self.heap_root + 1;
                let right = 2 * self.heap_root + 2;
                let mut largest = self.heap_root;

                if left < self.heap_end && self.data[left].value > self.data[largest].value {
                    largest = left;
                }
                if right < self.heap_end && self.data[right].value > self.data[largest].value {
                    largest = right;
                }
                if largest == self.heap_root {
                    self.phase = self.previous_phase.clone();
                    return;
                }
                let temp = self.data[self.heap_root].value;
                self.data[self.heap_root].value = self.data[largest].value;
                self.data[largest].value = temp;
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
                    let temp = self.data[0].value;
                    self.data[0].value = self.data[self.j].value;
                    self.data[self.j].value = temp;
                    self.j -= 1;

                    self.previous_phase = Phase::SortingHeap;
                    self.heap_root = 0;
                    self.heap_end = self.j + 1;
                    self.phase = Phase::HeapifyDown;
                } else {
                    self.finished = true;
                    // Set the colors
                    for bar in &mut self.data {
                        bar.fill = Color32::RED;
                    }
                    return;
                }
            }
        }
        // Colour things
        for (index, bar) in self.data.iter_mut().enumerate() {
            if index == self.i {
                bar.fill = Color32::GREEN;
            } else if index == self.j {
                bar.fill = Color32::YELLOW;
            } else {
                bar.fill = Color32::RED;
            }
        }
    }
}
