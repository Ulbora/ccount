use crate::getSavedFoodId;
use crate::getSavedFoodName;
use crate::getSevedFoodCals;
use crate::getUserEmail;
use crate::getUserPw;

use crate::services::category_service::get_category_list;
use crate::services::food_service::db_new_food;
use crate::services::food_service::db_update_food;
use crate::services::food_service::delete_food;
use crate::services::food_service::get_food_list_by_user;
use crate::services::food_service::Food;
use crate::services::food_service::NewFood;

use crate::PROD_BASE_URL;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub async fn food_screen() {
    let mut url = String::from(PROD_BASE_URL);
    let mut fdurl = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));

    let uemail = getUserEmail();
    let epw = getUserPw();

    let cat_list = get_category_list(&url, &uemail, &epw).await;
    let fd_list = get_food_list_by_user(&fdurl, &uemail, &epw).await;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let mut cmap: HashMap<i64, String> = HashMap::new();
    let mut html = String::from("<div id=\"foodScreen\" class=\"container-sm mt-5\">");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h1>Food</h1>");
    html.push_str("<form>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"cat\">Category</label>");
    html.push_str("<select class=\"form-control\" id=\"cat\">");
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
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"food\">Food</label>");
    html.push_str("<input class=\"form-control\" id=\"food\" placeholder=\"steak 10oz\">");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"cals\">Calories</label>");
    html.push_str("<input type=\"number\" class=\"form-control\" id=\"cals\" placeholder=\"344\">");
    html.push_str("</div>");
    html.push_str(
        "<input type=\"button\" onclick=\"addFood();\" class=\"btn btn-primary\" value=\"Add\">",
    );
    html.push_str("</form>");
    html.push_str("</div>");

    html.push_str("<h2>Existing Foods</h2>");
    html.push_str("<table class=\"table table-hover mb-5\">");
    html.push_str("<thead>");
    html.push_str("<tr>");
    html.push_str("<th scope=\"col\">Food</th>");
    html.push_str("<th scope=\"col\">Calories</th>");
    html.push_str("<th scope=\"col\">Category</th>");
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
        html.push_str(&cmap[&f.category_id]);
        html.push_str("</td>");
        html.push_str("</tr>");
    }
    html.push_str("</tbody>");
    html.push_str("</table>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn new_food() {
    let mut url = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/food/new"));
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let food = document
        .get_element_by_id("food")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let catval = document
        .get_element_by_id("cat")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .value();

    let cals = document
        .get_element_by_id("cals")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    if !food.eq("") && !catval.eq("") && !cals.eq("") {
        let cat_id = catval.parse::<i64>().unwrap();
        let calsint = cals.parse::<i32>().unwrap();
        let uemail = getUserEmail();
        let epw = getUserPw();
        let req = NewFood {
            name: food,
            category_id: cat_id,
            calories: calsint,
            user_email: uemail,
        };
        let uemail = getUserEmail();
        let res = db_new_food(&url, &uemail, &epw, &req).await;
        if res.success {
            food_screen().await;
        } else {
            food_screen().await;
        }
    }
}

#[wasm_bindgen]
pub async fn food_item_screen() {
    let fid = getSavedFoodId();
    let fname = getSavedFoodName();
    let fcals = getSevedFoodCals();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let mut html = String::from("<div id=\"foodItemScreen\" class=\"container-sm mt-5\">");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str(
        "<input type=\"button\" onclick=\"deleteFood();\" class=\"btn btn-danger mb-2\" value=\"Delete\">",
    );
    html.push_str("<h1>Food</h1>");
    html.push_str("<form>");
    html.push_str("<input type=\"hidden\" class=\"form-control\" id=\"fid\" value=\"");
    html.push_str(&fid);
    html.push_str("\">");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"food\">Food</label>");
    html.push_str("<input type=\"text\" class=\"form-control\" id=\"food\" value=\"");
    html.push_str(&fname);
    html.push_str("\">");
    html.push_str("</div>");
    html.push_str("<div class=\"form-group\">");
    html.push_str("<label for=\"cals\">Calories</label>");
    html.push_str("<input type=\"text\" class=\"form-control\" id=\"cals\" value=\"");
    html.push_str(&fcals);
    html.push_str("\">");
    html.push_str("</div>");
    html.push_str("<input type=\"button\" onclick=\"updateFood();\" class=\"btn btn-primary\" value=\"Update\">");
    html.push_str("</form>");
    html.push_str("</div>");
    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn update_food() {
    let mut url = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/food/update"));
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let fids = document
        .get_element_by_id("fid")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let fid = fids.parse::<i64>().unwrap();

    let food = document
        .get_element_by_id("food")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let cals = document
        .get_element_by_id("cals")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    if !food.eq("") && !cals.eq("") {
        let calsint = cals.parse::<i32>().unwrap();
        let uemail = getUserEmail();
        let epw = getUserPw();
        let req = Food {
            id: fid,
            name: food,
            category_id: 0,
            calories: calsint,
            user_email: uemail,
        };
        let uemail = getUserEmail();
        let res = db_update_food(&url, &uemail, &epw, &req).await;

        if res.success {
            food_screen().await;
        } else {
            food_screen().await;
        }
    } else {
        food_screen().await;
    }
}

#[wasm_bindgen]
pub async fn del_food() {
    let fid = getSavedFoodId();
    let uemail = getUserEmail();
    let epw = getUserPw();
    let mut url = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/food"));

    let fidint = fid.parse::<i64>().unwrap();

    let res = delete_food(&url, &uemail, &epw, fidint).await;
    if res.success {
        food_screen().await;
    } else {
        food_screen().await;
    }
}
