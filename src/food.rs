use crate::alert;
use crate::getUserEmail;
use crate::getUserPw;
use crate::get_calories_by_day;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn food_screen() {
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
    let mut html = String::from("<div id=\"loginScreen\" class=\"container-sm mt-5\" > ");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Food</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"email\">Email</label>");
    html.push_str("<input name=\"email\" type=\"text\" class=\"form-control\" id=\"email\">");
    html.push_str("  </div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"password\">Password</label>");
    html.push_str(
        "<input name=\"password\" type=\"password\" class=\"form-control\" id=\"password\">",
    );
    html.push_str("</div>");
    html.push_str("<button onclick=\"loginUser();\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("</div>");
    html.push_str("<nav class=\"nav\">");
    html.push_str(
        "<a onclick=\"changePwScreen()\" class=\"nav-link active\" href=\"#\">Change Password</a>",
    );
    // html.push_str(
    //     "<a onclick=\"resetPwScreen()\" class=\"nav-link\" href=\"#\">Reset Password</a>",
    // );
    html.push_str("<a onclick=\"registerScreen()\" class=\"nav-link\" href=\"#\">Register</a>");
    html.push_str("</nav></div>");

    val.set_inner_html(&html);

    //val.set_inner_html("<div style=\"margin: 5% 0 0 0;\">test</div>");

    body.append_child(&val).unwrap();
}
