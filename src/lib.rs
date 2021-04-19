use crate::calories::get_calories_by_day;

use wasm_bindgen::prelude::*;

pub mod calories;
pub mod food;
pub mod services;
pub mod users;

const PROD_BASE_URL: &str = "http://ccount.cocka2notes.com";

// const PROD_BASE_URL: &str = "http://localhost:3000";

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn setUserEmail(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn getUserEmail() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn setUserPw(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn getUserPw() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getSavedFoodId() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getSavedFoodName() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getSevedFoodCals() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn setCaloryDateValue(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn getCalDateValue() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn setCatValue(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn getCatValue() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getCalariesIdToAdd() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getCalariesAddDate() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn setCaloriesAddDate(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn setAddCaloryCatValue(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn getCaloryCatValue() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getCalariesIdToRem() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn getCalariesRemDate() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn setCaloriesRemDate(s: &str);
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    get_calories_by_day().await;
    Ok(())
}
