use wasm_bindgen::prelude::*;

pub mod calories;
pub mod food;
pub mod services;
pub mod users;

const LOCAL_BASE_URL: &str = "http://localhost:3000";

const PROD_BASE_URL: &str = "http://ccount.cocka2notes.com";

const PROD_TEST_URL: &str = "http://ccount.cocka2notes.com/user/login";

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

#[wasm_bindgen]
pub fn get_calories_by_day() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    val.set_inner_html(
        "<div style=\"margin: 5% 0 0 0;\" onclick=\"alert('hi ken');\">Some new calories</div>",
    );

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub fn get_food_by_day() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    val.set_inner_html(
        "<div style=\"margin: 5% 0 0 0;\" onclick=\"alert('hi ken');\">Some new food</div>",
    );

    body.append_child(&val).unwrap();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    val.set_inner_html(
        "<div style=\"margin: 5% 0 0 0;\" onclick=\"alert('hi ken');\">Test this</div>",
    );

    body.append_child(&val)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
