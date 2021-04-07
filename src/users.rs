use crate::getUserEmail;
use crate::getUserPw;
use crate::get_calories_by_day;
use crate::services::user_service::db_change_pw;
use crate::services::user_service::db_new_user;
use crate::services::user_service::is_prod_alive;
use crate::setUserEmail;
use crate::setUserPw;
use crate::LOCAL_BASE_URL;

use crate::PROD_BASE_URL;
use crate::PROD_TEST_URL;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::services::user_service::login_user;

#[wasm_bindgen]
pub fn login() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    let mut html = String::from("<div id=\"loginScreen\" class=\"container-sm mt-5\" > ");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Login</h1>");
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
    html.push_str("<a onclick=\"registerScreen()\" class=\"nav-link\" href=\"#\">Register</a>");
    html.push_str("</nav></div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn user_login() {
    let pa = is_prod_alive(&PROD_TEST_URL).await;
    let mut url = String::from(LOCAL_BASE_URL);
    if pa.success {
        url = String::from(PROD_BASE_URL);
    }
    url.push_str(&String::from("/user/login"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let email = document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let pw = document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let suc = login_user(&url, &email, &pw).await;
    if suc.success {
        setUserEmail(&email);
        setUserPw(&pw);
        get_calories_by_day();
    } else {
        let body = document.body().expect("document should have a body");
        let val = document.get_element_by_id("cont").unwrap();
        let mut html = String::from("<div id=\"loginScreen\" class=\"container-sm mt-5\" > ");
        html.push_str("<div class=\"alert alert-danger\" role=\"alert\">Login Failed!</div>");
        html.push_str("</div>");

        val.set_inner_html(&html);

        body.append_child(&val).unwrap();
    }
}

#[wasm_bindgen]
pub fn change_pw_screen() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let uemail = getUserEmail();
    let mut html = String::from("<div id=\"changePwScreen\" class=\"container-sm\" >");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Change Password</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"email\">Email</label>");
    html.push_str(
        "<input disabled name=\"cpwemail\" type=\"text\" class=\"form-control\" id=\"cpwemail\" value=\"",
    );
    html.push_str(&uemail);
    html.push_str("\" >");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"newPassword\">Password</label>");
    html.push_str(
        "<input name=\"newPassword\" type=\"password\" class=\"form-control\" id=\"newPassword\">",
    );
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"newPassword\">Confirm</label>");
    html.push_str(
        "<input name=\"newPassword\" type=\"password\" class=\"form-control\" id=\"newPassword2\">",
    );
    html.push_str("</div>");
    html.push_str("<button onclick=\"changePassword()\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub fn change_pw_screen_pw_no_match() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    let uemail = getUserEmail();
    let mut html = String::from("<div id=\"changePwScreen\" class=\"container-sm\" >");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Change Password</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"email\">Email</label>");
    html.push_str(
        "<input disabled name=\"cpwemail\" type=\"text\" class=\"form-control\" id=\"cpwemail\" value=\"",
    );
    html.push_str(&uemail);
    html.push_str("\" >");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"newPassword\">Password</label>");
    html.push_str(
        "<input name=\"newPassword\" type=\"password\" class=\"form-control\" id=\"newPassword\">",
    );
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"newPassword\">Confirm</label>");
    html.push_str(
        "<input name=\"newPassword\" type=\"password\" class=\"form-control\" id=\"newPassword2\">",
    );
    html.push_str("</div>");
    html.push_str("<button onclick=\"changePassword()\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("<div id=\"pwnotMatch\" class=\"alert alert-danger mt-4\" role=\"alert\">");
    html.push_str("Passwords didn't match!");
    html.push_str("</div>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn change_pw() {
    let pa = is_prod_alive(&PROD_TEST_URL).await;
    let mut url = String::from(LOCAL_BASE_URL);
    if pa.success {
        url = String::from(PROD_BASE_URL);
    }
    url.push_str(&String::from("/user/change/pw"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let pw = document
        .get_element_by_id("newPassword")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let pw2 = document
        .get_element_by_id("newPassword2")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    if pw.eq(&pw2) {
        let uemail = getUserEmail();
        let epw = getUserPw();
        let suc = db_change_pw(&url, &uemail, &epw, &pw).await;
        if suc.success {
            setUserPw(&pw);
            get_calories_by_day();
        } else {
            change_pw_screen_pw_no_match();
        }
    } else {
        change_pw_screen_pw_no_match();
    }
}

#[wasm_bindgen]
pub fn register_screen() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();
    let mut html = String::from("<div id=\"registerScreen\" class=\"container-sm\" >");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Register</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regemail\">Email</label>");
    html.push_str("<input name=\"regemail\" type=\"text\" class=\"form-control\" id=\"regemail\">");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regpassword\">Password</label>");
    html.push_str(
        "<input name=\"regpassword\" type=\"password\" class=\"form-control\" id=\"regpassword\">",
    );
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regnewPassword\">Confirm</label>");
    html.push_str("<input name=\"regnewPassword\" type=\"password\" class=\"form-control\" id=\"regnewPassword2\">");
    html.push_str("</div>");
    html.push_str("<button onclick=\"registerUser()\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub fn register_screen_pw_mismatch() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let mut html = String::from("<div id=\"registerScreen\" class=\"container-sm\" >");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Register</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regemail\">Email</label>");
    html.push_str("<input name=\"regemail\" type=\"text\" class=\"form-control\" id=\"regemail\">");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regpassword\">Password</label>");
    html.push_str(
        "<input name=\"regpassword\" type=\"password\" class=\"form-control\" id=\"regpassword\">",
    );
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regnewPassword\">Confirm</label>");
    html.push_str("<input name=\"regnewPassword\" type=\"password\" class=\"form-control\" id=\"regnewPassword2\">");
    html.push_str("</div>");
    html.push_str("<button onclick=\"registerUser()\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("<div id=\"regpwnotMatch\" class=\"alert alert-danger mt-4\" role=\"alert\">");
    html.push_str("Passwords didn't match!");
    html.push_str("</div>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub fn register_screen_dup() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let mut html = String::from("<div id=\"registerScreen\" class=\"container-sm\" >");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Register</h1>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regemail\">Email</label>");
    html.push_str("<input name=\"regemail\" type=\"text\" class=\"form-control\" id=\"regemail\">");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regpassword\">Password</label>");
    html.push_str(
        "<input name=\"regpassword\" type=\"password\" class=\"form-control\" id=\"regpassword\">",
    );
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"regnewPassword\">Confirm</label>");
    html.push_str("<input name=\"regnewPassword\" type=\"password\" class=\"form-control\" id=\"regnewPassword2\">");
    html.push_str("</div>");
    html.push_str("<button onclick=\"registerUser()\" class=\"btn btn-primary\">Submit</button>");
    html.push_str("<div id=\"regUserExist\" class=\"alert alert-danger mt-4\" role=\"alert\">");
    html.push_str("The user already exists!");
    html.push_str("</div>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn register() {
    let pa = is_prod_alive(&PROD_TEST_URL).await;
    let mut url = String::from(LOCAL_BASE_URL);
    if pa.success {
        url = String::from(PROD_BASE_URL);
    }
    url.push_str(&String::from("/user/new"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let email = document
        .get_element_by_id("regemail")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let pw = document
        .get_element_by_id("regpassword")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let pw2 = document
        .get_element_by_id("regnewPassword2")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    if !email.eq("") && !pw.eq("") && pw.eq(&pw2) {
        let suc = db_new_user(&url, &email, &pw).await;
        if suc.success {
            setUserEmail(&email);
            setUserPw(&pw);
            get_calories_by_day();
        } else {
            register_screen_dup();
        }
    } else {
        register_screen_pw_mismatch();
    }
}
