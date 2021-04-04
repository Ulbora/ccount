use crate::alert;
use crate::getCalDateValue;
use crate::getCatValue;
use crate::getUserEmail;
use crate::getUserPw;
use crate::services::category_service::get_category_list;
use crate::services::food_service::get_food_list;
use crate::services::food_service::get_food_list_by_user;
use crate::services::user_service::is_prod_alive;
use crate::LOCAL_BASE_URL;
use crate::PROD_BASE_URL;
use crate::PROD_TEST_URL;
use js_sys::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub async fn food_calory_screen() {
    let pa = is_prod_alive(&PROD_TEST_URL).await;
    let mut url = String::from(LOCAL_BASE_URL);
    let mut fdurl = String::from(LOCAL_BASE_URL);
    let mut caurl = String::from(LOCAL_BASE_URL);
    if pa.success {
        url = String::from(PROD_BASE_URL);
        fdurl = String::from(PROD_BASE_URL);
        caurl = String::from(PROD_BASE_URL);
    }
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));
    caurl.push_str(&String::from("/calory/list"));

    let uemail = getUserEmail();
    let epw = getUserPw();

    let cal_date = getCalDateValue();
    let cat_val = getCatValue();

    let mut seldate = String::new();

    if cal_date.eq("") {
        //// let mut ndt = "";
        //
        let now = js_sys::Date::now();
        let dt = js_sys::Date::new(&JsValue::from_f64(now));
        //let dt = chrono::offset::Local::now();
        let m = dt.get_month();
        let d = dt.get_date();
        let y = dt.get_full_year();
        let mut ms = (m + 1).to_string();
        if ms.len() < 2 {
            ms = String::from("0");
            ms.push_str(&(m + 1).to_string());
        }
        let mut ds = d.to_string();
        if ds.len() < 2 {
            ds = String::from("0");
            ds.push_str(&d.to_string());
        }
        let ys = y.to_string();
        // //let m = dt.
        let mut today = String::new();
        today.push_str(&ys);
        today.push_str("-");
        today.push_str(&ms);
        today.push_str("-");
        today.push_str(&ds);
        seldate = today;
    }

    let cat_list = get_category_list(&url, &uemail, &epw).await;
    let fd_list = get_food_list_by_user(&fdurl, &uemail, &epw).await;
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

    let mut cmap: HashMap<i64, String> = HashMap::new();
    let mut html = String::from("<div id=\"foodScreen\" class=\"container-sm mt-5\">");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Daily Calories</h1>");
    html.push_str("<form>");
    html.push_str("<div class=\"form-row\">");
    html.push_str("<div class=\"form-group col-md-3\">");
    html.push_str("<label>Day</label>");
    html.push_str("<input id=\"day\" class=\"form-control\" type=\"date\" ");
    html.push_str("value=\"");
    //html.push_str("2021-4-3");
    html.push_str(&seldate);
    // alert(&seldate);
    html.push_str("\" required pattern=\"\\d{2}-\\d{2}-\\d{4}\">");
    //html.push_str("<input id=\"day\" class=\"form-control\" type=\"date\">");
    html.push_str("</div>");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"cat\">Category</label>");
    html.push_str("<select class=\"form-control\" id=\"cat\">");
    html.push_str("<option value=\"0\">Select Category</option>");
    for c in cat_list {
        html.push_str("<option value=\"");
        html.push_str(&c.id.to_string());
        html.push_str("\">");
        html.push_str(&c.name);
        html.push_str("</option>");
        cmap.insert(c.id, c.name);
    }
    html.push_str("</select>");
    html.push_str("</div>");
    html.push_str(
        "<button onclick=\"addCaloriesByCategoryScreen();\" class=\"btn btn-primary\">Add Calories</button>",
    );
    html.push_str("</form>");
    html.push_str("</div>");

    // html.push_str("<h2>Existing Calories</h2>");
    // html.push_str("<table class=\"table table-hover mb-5\">");
    // html.push_str("<thead>");
    // html.push_str("<tr>");
    // html.push_str("<th scope=\"col\">Food</th>");
    // html.push_str("<th scope=\"col\">Calories</th>");
    // html.push_str("<th scope=\"col\">Category</th>");
    // html.push_str("</tr>");
    // html.push_str("</thead>");
    // html.push_str("<tbody>");
    // for f in fd_list {
    //     html.push_str("<tr class='clickable-row' onclick=\"foodItemScreen('");
    //     html.push_str(&f.id.to_string());
    //     html.push_str("','");
    //     html.push_str(&f.name);
    //     html.push_str("','");
    //     html.push_str(&f.calories.to_string());
    //     html.push_str("');\">");
    //     html.push_str("<td>");
    //     html.push_str(&f.name);
    //     html.push_str("</td>");
    //     html.push_str("<td>");
    //     html.push_str(&f.calories.to_string());
    //     html.push_str("</td>");

    //     html.push_str("<td>");
    //     html.push_str(&cmap[&f.category_id]);
    //     html.push_str("</td>");
    //     html.push_str("</tr>");
    // }
    // html.push_str("</tbody>");
    // html.push_str("</table>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    //val.set_inner_html("<div style=\"margin: 5% 0 0 0;\">test</div>");

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn add_calory_screen() {
    let cal_date = getCalDateValue();
    let cat_val = getCatValue();

    let uemail = getUserEmail();
    let epw = getUserPw();

    let pa = is_prod_alive(&PROD_TEST_URL).await;
    let mut url = String::from(LOCAL_BASE_URL);
    let mut fdurl = String::from(LOCAL_BASE_URL);
    let mut caurl = String::from(LOCAL_BASE_URL);
    if pa.success {
        url = String::from(PROD_BASE_URL);
        fdurl = String::from(PROD_BASE_URL);
        caurl = String::from(PROD_BASE_URL);
    }
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));
    caurl.push_str(&String::from("/calory/list"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p").unwrap();
    let val = document.get_element_by_id("cont").unwrap();

    let catval = document
        .get_element_by_id("cat")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .value();

    let dt = document
        .get_element_by_id("day")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    if !catval.eq("0") && !dt.eq("") {
        let mut cat_disp = String::new();
        let cat_id = catval.parse::<i64>().unwrap();

        let cat_list = get_category_list(&url, &uemail, &epw).await;
        let fd_list = get_food_list(&fdurl, &uemail, &epw, cat_id).await;

        // let cal_list =

        for c in cat_list {
            if c.id == cat_id {
                cat_disp = c.name;
            }
        }

        // alert(&catval);
        // alert(&cat_disp);
        // alert(&dt);
        let mut html = String::from("<div id=\"selectFoodScreen\" class=\"container-sm mt-5\">");
        html.push_str("<h2>Add Calories</h2>");
        html.push_str("<h6>Category: ");
        html.push_str(&cat_disp);
        html.push_str("</h6>");
        html.push_str("<table class=\"table table-hover mb-5\">");
        html.push_str("<thead>");
        html.push_str("<tr>");
        html.push_str("<th scope=\"col\">Food</th>");
        html.push_str("<th scope=\"col\">Calories</th>");
        html.push_str("<th scope=\"col\"></th>");
        html.push_str("<th scope=\"col\"></th>");
        html.push_str("</tr>");
        html.push_str("</thead>");
        html.push_str("<tbody>");
        for f in fd_list {
            html.push_str("<tr class='clickable-row' onclick=\"foodItemScreen('");
            html.push_str(&f.id.to_string());
            html.push_str("','");
            html.push_str(&f.name);
            html.push_str("','");
            html.push_str(&f.calories.to_string());
            html.push_str("');\">");
            html.push_str("<td>");
            html.push_str(&f.name);
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str(&f.calories.to_string());
            html.push_str("</td>");

            html.push_str("<td>");
            //html.push_str(&cmap[&f.category_id]);
            html.push_str("</td>");

            html.push_str("<td>");
            //html.push_str(&cmap[&f.category_id]);
            html.push_str("</td>");
            html.push_str("</tr>");
        }
        html.push_str("</tbody>");
        html.push_str("</table>");
        html.push_str("</div>");

        // html.push_str("<h2 class=\"mt-5;\"">Existing Calories</h2>");
        // html.push_str("<table class=\"table table-hover mb-5\">");
        // html.push_str("<thead>");
        // html.push_str("<tr>");
        // html.push_str("<th scope=\"col\">Food</th>");
        // html.push_str("<th scope=\"col\">Calories</th>");
        // html.push_str("<th scope=\"col\">Category</th>");
        // html.push_str("</tr>");
        // html.push_str("</thead>");
        // html.push_str("<tbody>");
        // for f in fd_list {
        //     html.push_str("<tr class='clickable-row' onclick=\"foodItemScreen('");
        //     html.push_str(&f.id.to_string());
        //     html.push_str("','");
        //     html.push_str(&f.name);
        //     html.push_str("','");
        //     html.push_str(&f.calories.to_string());
        //     html.push_str("');\">");
        //     html.push_str("<td>");
        //     html.push_str(&f.name);
        //     html.push_str("</td>");
        //     html.push_str("<td>");
        //     html.push_str(&f.calories.to_string());
        //     html.push_str("</td>");

        //     html.push_str("<td>");
        //     html.push_str(&cmap[&f.category_id]);
        //     html.push_str("</td>");
        //     html.push_str("</tr>");
        // }
        // html.push_str("</tbody>");
        // html.push_str("</table>");
        val.set_inner_html(&html);

        body.append_child(&val).unwrap();
    }
}
