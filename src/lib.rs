#![warn(clippy::all, rust_2018_idioms)]
mod app;
pub use app::SortApp;
mod bubble;
pub use bubble::BubbleSort;
mod insertion;
pub use insertion::InsertionSort;

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
