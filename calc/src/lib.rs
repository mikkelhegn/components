wit_bindgen::generate!("calculator");

struct MyCalculator;

impl Calculator for MyCalculator {
    fn add(a: u32, b:u32) -> u32 {
        a + b
    }
}

export_calculator!(MyCalculator);