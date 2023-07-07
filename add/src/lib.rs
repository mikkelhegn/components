wit_bindgen::generate!("operator");

struct MyOperator;

impl Operator for MyOperator {
    fn add(a: u32, b:u32) -> u32 {
        a + b
    }
}

export_operator!(MyOperator);