mod utils;

use wasm_bindgen::prelude::*;

use csv::ReaderBuilder;
use serde::Deserialize;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Deserialize, Clone, Copy)]
struct Candle {
    #[serde(rename = "Open")] // Note the capitalization
    open: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
}

#[wasm_bindgen]
pub fn detect_patterns(csv_data: &str) -> String {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());
    let mut candles: Vec<Candle> = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(record) => {
                candles.push(record);
            }
            Err(err) => {
                return format!("Error reading CSV: {}", err);
            }
        }
    }

    if candles.len() < 2 {
        return "Not enough data to detect patterns.".to_string();
    }

    let mut patterns = Vec::new();

    for i in 1..candles.len() {
        let prev = &candles[i - 1];
        let curr = &candles[i];

        // Bullish Engulfing
        if prev.close < prev.open
            && curr.close > curr.open
            && curr.close > prev.open
            && curr.open < prev.close
        {
            patterns.push(format!("Bullish Engulfing at index {}", i));
        }
        // Bearish Engulfing
        else if prev.close > prev.open
            && curr.close < curr.open
            && curr.close < prev.open
            && curr.open > prev.close
        {
            patterns.push(format!("Bearish Engulfing at index {}", i));
        }
        // Hammer
        else if curr.close > curr.open
            && (curr.open - curr.low) > 2.0 * (curr.close - curr.open).abs()
            && (curr.high - curr.open) < (curr.open - curr.low)
        {
            patterns.push(format!("Hammer at index {}", i));
        }
        // Shooting Star
        else if curr.close < curr.open
            && (curr.high - curr.close) > 2.0 * (curr.open - curr.close).abs()
            && (curr.open - curr.low) < (curr.high - curr.close)
            && prev.close > prev.open
        {
            // Ensuring it's after an uptrend
            patterns.push(format!("Shooting Star at index {}", i));
        }
        // Doji
        else if (curr.open - curr.close).abs() < 0.01 * curr.open
            && (curr.high - curr.low) > 2.0 * (curr.open - curr.close).abs()
        {
            patterns.push(format!("Doji at index {}", i));
        }
    }

    if patterns.is_empty() {
        "No patterns detected.".to_string()
    } else {
        patterns.join("\n")
    }
}

// Function to log to the console
fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}
