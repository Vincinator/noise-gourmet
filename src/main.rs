#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

//mod fractal_clock;
//use fractal_clock::FractalClock;

mod noise;
use noise::noise_graph::NoiseGraph;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = NoiseGraph::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
