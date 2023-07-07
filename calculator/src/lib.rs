wit_bindgen::generate!("calculator");

struct MyCalculator;

impl Calculator for MyCalculator {
    fn calc() -> u32 {
        let result: u32 = add(1, 1);
        result
    }
}

export_calculator!(MyCalculator);