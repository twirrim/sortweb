#![warn(clippy::all, rust_2018_idioms)]
mod app;
mod bubble;
mod heap;
mod insertion;
mod shaker;
mod shell;

pub use app::SortApp;
pub use bubble::BubbleSort;
pub use heap::HeapSort;
pub use insertion::InsertionSort;
pub use shaker::ShakerSort;
pub use shell::ShellSort;

use ecolor::Color32;
use egui::{Response, Ui};
use egui_plot::{Bar, BarChart, Plot};
use rand::seq::SliceRandom;
use rand::thread_rng;

// from https://www.reddit.com/r/rust/comments/skmpnr/output_text_to_console_in_debug_mode_only/hvluai2/
// This gets us a print statement that is only compiled to code if compiled in debug mode
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub fn plot_chart(ui: &mut Ui, name: &str, data: &Data) -> Response {
    let chart = BarChart::new(data.0.to_owned()).name(name);
    Plot::new(name)
        .clamp_grid(true)
        .y_axis_min_width(3.0)
        .show(ui, |plot_ui| plot_ui.bar_chart(chart))
        .response
}

pub fn distance_to_optimal(data: &Data) -> f64 {
    let mut distance = 0.0;
    for entry in &data.0 {
        distance += (entry.argument - entry.value).abs();
    }
    distance / data.len() as f64
}

pub fn make_bar_vec(size: u16) -> Data {
    // Produce a randomly shuffled vector of numbers first
    let mut rng = thread_rng();
    let mut numbers: Vec<u16> = (0..size).collect();
    numbers.shuffle(&mut rng);

    // Now turn it in to a vector of BarChart Bars
    let mut bars: Data = Data(vec![]);

    for (index, value) in numbers.iter().enumerate() {
        let mut bar = Bar::new(index as f64, f64::from(*value));
        bar.fill = Color32::RED;
        bars.0.push(bar);
    }

    bars
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Greater,
    Less,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data(Vec<Bar>);

impl Data {
    fn compare(&self, a: usize, b: usize) -> Comparison {
        if self.0[a].value > self.0[b].value {
            return Comparison::Greater;
        } else if self.0[a].value < self.0[b].value {
            return Comparison::Less;
        }
        Comparison::Equal
    }
    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.0[a].value;
        self.0[a].value = self.0[b].value;
        self.0[b].value = temp;
    }

    fn restore_base_colours(&mut self) {
        self.0
            .iter_mut()
            .for_each(|x| x.fill = ecolor::Color32::RED);
    }

    fn set_to_green(&mut self, a: usize) {
        if a >= self.len() {
            return;
        }
        self.0[a].fill = ecolor::Color32::GREEN;
    }

    fn set_to_yellow(&mut self, a: usize) {
        if a >= self.len() {
            return;
        }
        self.0[a].fill = ecolor::Color32::YELLOW;
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // I'm feeling lazy, and egui_plot::Bars contain lots of fields
    // I should do this in smarter way, though
    fn get_input() -> Data {
        let numbers = [6, 2, 3, 6, 1, 2, 7, 8, 3, 2];
        let mut bars: Data = Data(vec![]);
        for (index, value) in numbers.iter().enumerate() {
            let mut bar = Bar::new(index as f64, f64::from(*value));
            bar.fill = Color32::RED;
            bars.0.push(bar);
        }
        bars
    }

    fn get_expected() -> Data {
        let sorted_numbers = [1, 2, 2, 2, 3, 3, 6, 6, 7, 8];
        let mut bars: Data = Data(vec![]);
        for (index, value) in sorted_numbers.iter().enumerate() {
            let mut bar = Bar::new(index as f64, f64::from(*value));
            bar.fill = Color32::RED;
            bars.0.push(bar);
        }
        bars
    }

    #[test]
    fn test_bubble_sort() {
        let mut sort = BubbleSort::new(get_input().clone());
        while !sort.finished() {
            sort.step();
        }
        assert_eq!(sort.data(), get_expected());
    }

    #[test]
    fn test_shaker_sort() {
        let mut sort = ShakerSort::new(get_input().clone());
        while !sort.finished() {
            sort.step();
        }
        assert_eq!(sort.data(), get_expected());
    }

    #[test]
    fn test_insertion_sort() {
        let mut sort = InsertionSort::new(get_input().clone());
        while !sort.finished() {
            sort.step();
        }
        assert_eq!(sort.data(), get_expected());
    }

    #[test]
    fn test_shell_sort() {
        let mut sort = ShellSort::new(get_input().clone());
        while !sort.finished() {
            sort.step();
        }
        assert_eq!(sort.data(), get_expected());
    }

    #[test]
    fn test_heap_sort() {
        let mut sort = HeapSort::new(get_input().clone());
        while !sort.finished() {
            sort.step();
        }
        assert_eq!(sort.data(), get_expected());
    }
}
