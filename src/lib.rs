#![warn(clippy::all, rust_2018_idioms)]
mod app;
mod bubble;
mod insertion;
mod shaker;
mod shell;

pub use app::SortApp;
pub use bubble::BubbleSort;
pub use insertion::InsertionSort;
pub use shaker::ShakerSort;
pub use shell::ShellSort;

use ecolor::Color32;
use egui_plot::Bar;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn make_bar_vec(size: usize) -> Vec<Bar> {
    // Produce a randomly shuffled vector of numbers first
    let mut rng = thread_rng();
    let mut numbers: Vec<usize> = (0..size).collect();
    numbers.shuffle(&mut rng);

    // Now turn it in to a vector of BarChart Bars
    let mut bars: Vec<Bar> = vec![];

    for (index, value) in numbers.iter().enumerate() {
        let mut bar = Bar::new(index as f64, *value as f64);
        bar.fill = Color32::RED;
        bars.push(bar);
    }

    bars
}

#[cfg(test)]
mod tests {
    use super::*;

    // I'm feeling lazy, and egui_plot::Bars contain lots of fields
    // I should do this in smarter way, though
    fn get_input() -> Vec<Bar> {
        let numbers = vec![6, 2, 3, 6, 1, 2, 7, 8, 3, 2];
        let mut bars: Vec<Bar> = vec![];
        for (index, value) in numbers.iter().enumerate() {
            let mut bar = Bar::new(index as f64, *value as f64);
            bar.fill = Color32::RED;
            bars.push(bar);
        }
        bars
    }

    fn get_expected() -> Vec<Bar> {
        let sorted_numbers = vec![1, 2, 2, 2, 3, 3, 6, 6, 7, 8];
        let mut bars: Vec<Bar> = vec![];
        for (index, value) in sorted_numbers.iter().enumerate() {
            let mut bar = Bar::new(index as f64, *value as f64);
            bar.fill = Color32::RED;
            bars.push(bar);
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
}
