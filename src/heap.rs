use ecolor::Color32;
use egui::{Response, Ui};
use egui_plot::{Bar, BarChart, Plot};

enum Phase {
    BuildingHeap,
    SortingHeap,
}

pub struct HeapSort {
    data: Vec<Bar>,
    size: usize,
    phase: Phase,
    i: usize,
    j: usize,
    finished: bool,
}

impl HeapSort {
    pub fn new(data: Vec<Bar>) -> Self {
        let size = data.len();
        let mut sort = Self {
            data,
            size,
            phase: Phase::BuildingHeap,
            i: size / 2,
            j: 0,
            finished: false,
        };
        for entry in sort.data.iter_mut(){
            entry.fill = Color32::RED;
        };
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
        for entry in self.data.iter() {
            distance += (entry.argument - entry.value).abs();
        }
        distance / self.data.len() as f64
    }

    pub fn step(&mut self) {
        if self.finished {
            return;
        }

        match self.phase {
            Phase::BuildingHeap => {
                if self.i > 0 {
                    self.i -= 1;
                    self.heapify_down(self.i, self.size);
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
                    self.heapify_down(0, self.j + 1);
                } else {
                    self.finished = true;
                    // Set the colors
                    for bar in self.data.iter_mut(){
                        bar.fill = Color32::RED;
                    }
                    return;
                }
            }
        }
        // Colour things
        for (index, bar) in self.data.iter_mut().enumerate(){
            if index == self.i {
                bar.fill = Color32::GREEN;
            } else if index ==self.j {
                bar.fill = Color32::YELLOW;
            } else {
                bar.fill = Color32::RED;
            }
        };
    }

    fn heapify_down(&mut self, mut root: usize, end: usize) {
        loop {
            let left = 2 * root + 1;
            let right = 2 * root + 2;
            let mut largest = root;

            if left < end && self.data[left].value > self.data[largest].value {
                largest = left;
            }
            if right < end && self.data[right].value > self.data[largest].value {
                largest = right;
            }
            if largest == root {
                break;
            }
            let temp = self.data[root].value;
            self.data[root].value = self.data[largest].value;
            self.data[largest].value = temp;
            root = largest;
        }
    }

    // Make this a trait!
    pub fn plot_chart(&self, ui: &mut Ui) -> Response {
        let chart = BarChart::new(self.data.clone()).name("Heap Sort");
        Plot::new("Heap Sort Demo")
            .clamp_grid(true)
            .y_axis_width(3)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
}
