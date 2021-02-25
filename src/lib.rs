use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FooTest {
    pub x: u64,
    pub y: u64,
}

#[wasm_bindgen]
pub fn create_foo_test(x: u64, y: u64) -> JsValue {
    let example = FooTest {
        x,
        y,
    };

    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn foo_test_add(a: &JsValue) -> Result<u64, JsValue> {
    let value: FooTest = a.into_serde().unwrap();
    Ok(value.x + value.y)
} 

#[wasm_bindgen]
pub fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn add_u64(a: u64, b: u64) -> u64 {
    a + b
}