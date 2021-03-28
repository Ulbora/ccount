use wasm_bindgen::prelude::*;

pub mod services;
pub mod users;

// static LOCAL_BASE_URL: &str = "http://localhost:3000";

// static PROD_BASE_URL: &str = "http://ccount.cocka2notes.com";

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
    pub fn setUserPw(s: &str);
}

#[wasm_bindgen]
pub fn get_calories_by_day() {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p").unwrap();
    let val = document.get_element_by_id("cont").unwrap();
    // val.set_text_content(Some("Hello from Rust!"));
    // val.set_text_content(Some(
    //     "<div onclick=\"get_calories_by_day('hi ken');\">Test this</div>",
    // ));
    val.set_inner_html(
        "<div style=\"margin: 5% 0 0 0;\" onclick=\"alert('hi ken');\">Some new calories</div>",
    );

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub fn get_food_by_day() {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p").unwrap();
    let val = document.get_element_by_id("cont").unwrap();
    // val.set_text_content(Some("Hello from Rust!"));
    // val.set_text_content(Some(
    //     "<div onclick=\"get_calories_by_day('hi ken');\">Test this</div>",
    // ));
    val.set_inner_html(
        "<div style=\"margin: 5% 0 0 0;\" onclick=\"alert('hi ken');\">Some new food</div>",
    );

    body.append_child(&val).unwrap();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.get_element_by_id("cont").unwrap();
    // val.set_text_content(Some("Hello from Rust!"));
    // val.set_text_content(Some(
    //     "<div onclick=\"get_calories_by_day('hi ken');\">Test this</div>",
    // ));
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
