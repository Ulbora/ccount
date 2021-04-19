use crate::getCalDateValue;
use crate::getCalariesAddDate;
use crate::getCalariesIdToAdd;
use crate::getCalariesIdToRem;
use crate::services::calory_service::get_calories_for_days;

use crate::getUserEmail;
use crate::getUserPw;
use crate::services::calory_service::db_new_calories;
use crate::services::calory_service::delete_calories;
use crate::services::calory_service::get_calory_list_by_day;

use crate::services::calory_service::NewCalories;
use crate::services::category_service::get_category_list;
use crate::services::food_service::get_food_list;
use crate::services::food_service::get_food_list_by_user;
use crate::services::food_service::Food;

use crate::setAddCaloryCatValue;
use crate::setCaloriesAddDate;

use crate::PROD_BASE_URL;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub async fn food_calory_screen() {
    let mut url = String::from(PROD_BASE_URL);
    let mut fdurl = String::from(PROD_BASE_URL);
    let mut caurl = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));
    caurl.push_str(&String::from("/calory/list"));

    let uemail = getUserEmail();
    let epw = getUserPw();

    let cal_date = getCalDateValue();

    let mut seldate = String::new();

    if cal_date.eq("") {
        let now = js_sys::Date::now();
        let dt = js_sys::Date::new(&JsValue::from_f64(now));
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
        let mut today = String::new();
        today.push_str(&ys);
        today.push_str("-");
        today.push_str(&ms);
        today.push_str("-");
        today.push_str(&ds);
        seldate = today;
    }

    let cat_list = get_category_list(&url, &uemail, &epw).await;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

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
    html.push_str(&seldate);
    html.push_str("\" required pattern=\"\\d{2}-\\d{2}-\\d{4}\">");
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
        "<input type=\"button\" onclick=\"addCaloriesByCategoryScreen();\" class=\"btn btn-primary\" value=\"Add Calories\">",
    );
    html.push_str("</form>");
    html.push_str("</div>");

    html.push_str("</div>");

    val.set_inner_html(&html);

    body.append_child(&val).unwrap();
}

#[wasm_bindgen]
pub async fn add_calory_screen() {
    let uemail = getUserEmail();
    let epw = getUserPw();

    let mut url = String::from(PROD_BASE_URL);
    let mut fdurl = String::from(PROD_BASE_URL);
    let mut caurl = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));
    caurl.push_str(&String::from("/calory/list"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

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
        setAddCaloryCatValue(&catval);

        let dtspl = dt.split("-");

        let mut cat_disp = String::new();
        let cat_id = catval.parse::<i64>().unwrap();

        let cat_list = get_category_list(&url, &uemail, &epw).await;
        let fd_list = get_food_list_by_user(&fdurl, &uemail, &epw).await;

        if dtspl.count() == 3 {
            let dtcol: Vec<&str> = dt.split("-").collect();
            let mut sdate = String::from(dtcol[1]);
            sdate.push_str("-");
            sdate.push_str(dtcol[2]);
            sdate.push_str("-");
            sdate.push_str(dtcol[0]);

            setCaloriesAddDate(&sdate);

            let cal_list = get_calory_list_by_day(&caurl, &uemail, &epw, &sdate).await;
            let mut fmap: HashMap<i64, Food> = HashMap::new();
            for f in fd_list {
                fmap.insert(f.id, f);
            }

            let fd_list = get_food_list(&fdurl, &uemail, &epw, cat_id).await;
            let mut cmap: HashMap<i64, String> = HashMap::new();
            for c in &cat_list {
                if c.id == cat_id {
                    cat_disp = c.name.clone();
                }
                cmap.insert(c.id, c.name.clone());
            }

            let mut cal_total = 0;

            let mut html =
                String::from("<div id=\"selectFoodScreen\" class=\"container-sm mt-5\">");
            html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
            html.push_str("<h2>Add Calories</h2>");
            html.push_str("<div class=\"form-group\">");
            html.push_str("<label for=\"cat\">Category</label>");
            html.push_str("<select class=\"form-control\" id=\"cat\" onchange=\"addCaloriesByCategoryScreen2();\">");
            html.push_str("<option value=\"0\">Select Category</option>");
            for c in cat_list {
                html.push_str("<option value=\"");
                html.push_str(&c.id.to_string());
                html.push_str("\"");
                if c.name == cat_disp {
                    html.push_str(" selected ");
                }
                html.push_str(">");
                html.push_str(&c.name);
                html.push_str("</option>");
                cmap.insert(c.id, c.name);
            }
            html.push_str("</select>");
            html.push_str("</div>");
            html.push_str("<h6>Date: ");
            html.push_str(&sdate);
            html.push_str("</h6>");
            html.push_str("<table class=\"table table-hover mb-5\">");
            html.push_str("<thead>");
            html.push_str("<tr>");
            html.push_str("<th scope=\"col\">Food</th>");
            html.push_str("<th scope=\"col\">Calories</th>");
            html.push_str("<th scope=\"col\"></th>");
            html.push_str("</tr>");
            html.push_str("</thead>");
            html.push_str("<tbody>");
            for f in fd_list {
                html.push_str("<tr>");
                html.push_str("<td>");
                html.push_str(&f.name);
                html.push_str("</td>");
                html.push_str("<td>");
                html.push_str(&f.calories.to_string());
                html.push_str("</td>");
                html.push_str("<td>");
                html.push_str(
                    "<input type=\"button\" class=\"btn btn-success\" onclick=\"addCaloriesForDay('",
                );
                html.push_str(&f.id.to_string());
                html.push_str("','");
                html.push_str(&sdate);
                html.push_str("');\" value=\"+\" >");
                html.push_str("</td>");

                html.push_str("</tr>");
            }
            html.push_str("</tbody>");
            html.push_str("</table>");

            html.push_str("<h2 class=\"mt-5;\">Existing Calories</h2>");
            html.push_str("<table class=\"table table-hover mb-5\">");
            html.push_str("<thead>");
            html.push_str("<tr>");
            html.push_str("<th scope=\"col\">Food</th>");
            html.push_str("<th scope=\"col\">Calories</th>");
            html.push_str("<th scope=\"col\">Category</th>");
            html.push_str("<th scope=\"col\"></th>");
            html.push_str("</tr>");
            html.push_str("</thead>");
            html.push_str("<tbody>");
            for c in cal_list {
                html.push_str("<tr>");
                html.push_str("<td>");
                html.push_str(&fmap[&c.food_id].name);
                html.push_str("</td>");
                html.push_str("<td>");
                html.push_str(&fmap[&c.food_id].calories.to_string());
                html.push_str("</td>");

                html.push_str("<td>");
                html.push_str(&cmap[&fmap[&c.food_id].category_id]);
                html.push_str("</td>");
                html.push_str("<td>");
                html.push_str(
                    "<input type=\"button\" class=\"btn btn-danger\" onclick=\"remCaloriesForDay('",
                );
                html.push_str(&c.id.to_string());
                html.push_str("','");
                html.push_str(&sdate);
                html.push_str("');\" value=\"-\" >");
                html.push_str("</td>");
                html.push_str("</tr>");
                cal_total += &fmap[&c.food_id].calories;
            }

            html.push_str("<tr class=\"table-active\">");
            html.push_str("<td>");
            html.push_str("Total Calories");
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str(&cal_total.to_string());
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str("</td>");
            html.push_str("</tr>");
            html.push_str("</tbody>");
            html.push_str("</table>");
            html.push_str("</div>");
            html.push_str("</div>");
            val.set_inner_html(&html);

            body.append_child(&val).unwrap();
        }
    }
}

#[wasm_bindgen]
pub async fn add_calory_screen2() {
    let uemail = getUserEmail();
    let epw = getUserPw();
    let sdate = getCalariesAddDate();

    let mut url = String::from(PROD_BASE_URL);
    let mut fdurl = String::from(PROD_BASE_URL);
    let mut caurl = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/category/list"));
    fdurl.push_str(&String::from("/food/list"));
    caurl.push_str(&String::from("/calory/list"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let catval = document
        .get_element_by_id("cat")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .value();

    if !catval.eq("0") && !sdate.eq("") {
        setAddCaloryCatValue(&catval);

        let mut cat_disp = String::new();
        let cat_id = catval.parse::<i64>().unwrap();

        let cat_list = get_category_list(&url, &uemail, &epw).await;
        let fd_list = get_food_list_by_user(&fdurl, &uemail, &epw).await;

        let cal_list = get_calory_list_by_day(&caurl, &uemail, &epw, &sdate).await;
        let mut fmap: HashMap<i64, Food> = HashMap::new();
        for f in fd_list {
            fmap.insert(f.id, f);
        }
        let fd_list = get_food_list(&fdurl, &uemail, &epw, cat_id).await;

        let mut cmap: HashMap<i64, String> = HashMap::new();
        for c in &cat_list {
            if c.id == cat_id {
                cat_disp = c.name.clone();
            }
            cmap.insert(c.id, c.name.clone());
        }

        let mut cal_total = 0;

        let mut html = String::from("<div id=\"selectFoodScreen\" class=\"container-sm mt-5\">");
        html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
        html.push_str("<h2>Add Calories</h2>");
        html.push_str("<div class=\"form-group\">");
        html.push_str("<label for=\"cat\">Category</label>");
        html.push_str("<select class=\"form-control\" id=\"cat\" onchange=\"addCaloriesByCategoryScreen2();\">");
        html.push_str("<option value=\"0\">Select Category</option>");
        for c in cat_list {
            html.push_str("<option value=\"");
            html.push_str(&c.id.to_string());
            html.push_str("\"");
            if c.name == cat_disp {
                html.push_str(" selected ");
            }
            html.push_str(">");
            html.push_str(&c.name);
            html.push_str("</option>");
            cmap.insert(c.id, c.name);
        }
        html.push_str("</select>");
        html.push_str("</div>");
        html.push_str("<h6>Date: ");
        html.push_str(&sdate);
        html.push_str("</h6>");
        html.push_str("<table class=\"table table-hover mb-5\">");
        html.push_str("<thead>");
        html.push_str("<tr>");
        html.push_str("<th scope=\"col\">Food</th>");
        html.push_str("<th scope=\"col\">Calories</th>");
        html.push_str("<th scope=\"col\"></th>");
        html.push_str("</tr>");
        html.push_str("</thead>");
        html.push_str("<tbody>");
        for f in fd_list {
            html.push_str("<tr>");
            html.push_str("<td>");
            html.push_str(&f.name);
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str(&f.calories.to_string());
            html.push_str("</td>");

            html.push_str("<td>");
            html.push_str(
                "<input type=\"button\" class=\"btn btn-success\" onclick=\"addCaloriesForDay('",
            );
            html.push_str(&f.id.to_string());
            html.push_str("','");
            html.push_str(&sdate);
            html.push_str("');\" value=\"+\" >");
            html.push_str("</td>");

            html.push_str("</tr>");
        }
        html.push_str("</tbody>");
        html.push_str("</table>");

        html.push_str("<h2 class=\"mt-5;\">Existing Calories</h2>");
        html.push_str("<table class=\"table table-hover mb-5\">");
        html.push_str("<thead>");
        html.push_str("<tr>");
        html.push_str("<th scope=\"col\">Food</th>");
        html.push_str("<th scope=\"col\">Calories</th>");
        html.push_str("<th scope=\"col\">Category</th>");
        html.push_str("<th scope=\"col\"></th>");
        html.push_str("</tr>");
        html.push_str("</thead>");
        html.push_str("<tbody>");
        for c in cal_list {
            html.push_str("<tr>");
            html.push_str("<td>");
            html.push_str(&fmap[&c.food_id].name);
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str(&fmap[&c.food_id].calories.to_string());
            html.push_str("</td>");

            html.push_str("<td>");
            html.push_str(&cmap[&fmap[&c.food_id].category_id]);
            html.push_str("</td>");
            html.push_str("<td>");
            html.push_str(
                "<input type=\"button\" class=\"btn btn-danger\" onclick=\"remCaloriesForDay('",
            );
            html.push_str(&c.id.to_string());
            html.push_str("','");
            html.push_str(&sdate);
            html.push_str("');\" value=\"-\">");
            html.push_str("</td>");
            html.push_str("</tr>");
            cal_total += &fmap[&c.food_id].calories;
        }
        html.push_str("<tr class=\"table-active\">");
        html.push_str("<td>");
        html.push_str("Total Calories");
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str(&cal_total.to_string());
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str("</td>");
        html.push_str("</tr>");

        html.push_str("</tbody>");
        html.push_str("</table>");
        html.push_str("</div>");
        html.push_str("</div>");
        val.set_inner_html(&html);

        body.append_child(&val).unwrap();
    }
}

#[wasm_bindgen]
pub async fn add_food_calories() {
    let uemail = getUserEmail();
    let epw = getUserPw();

    let fids = getCalariesIdToAdd();
    let fadt = getCalariesAddDate();
    let mut url = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/calories/new"));

    let fid = fids.parse::<i64>().unwrap();
    let ncr = NewCalories {
        day: fadt,
        food_id: fid,
        user_email: uemail,
    };
    let uemail = getUserEmail();
    let afdres = db_new_calories(&url, &uemail, &epw, &ncr).await;
    if afdres.success {
        add_calory_screen2().await;
    } else {
        add_calory_screen2().await;
    }
}

#[wasm_bindgen]
pub async fn remove_food_calories() {
    let epw = getUserPw();

    let cids = getCalariesIdToRem();
    let mut url = String::from(PROD_BASE_URL);
    let mut calurl = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/calories"));
    calurl.push_str(&String::from("/calory/list"));

    let uemail = getUserEmail();

    let cid = cids.parse::<i64>().unwrap();

    let afdres = delete_calories(&url, &uemail, &epw, cid).await;
    if afdres.success {
        add_calory_screen2().await;
    } else {
        add_calory_screen2().await;
    }
}

#[wasm_bindgen]
pub async fn get_calories_by_day() {
    let uemail = getUserEmail();
    let epw = getUserPw();

    let mut url = String::from(PROD_BASE_URL);
    url.push_str(&String::from("/calories/days"));

    let count_list = get_calories_for_days(&url, &uemail, &epw, 10).await;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.get_element_by_id("cont").unwrap();

    let mut html = String::from("<div id=\"selectFoodScreen\" class=\"container-sm mt-5\">");
    html.push_str("<div class=\"shadow-none p-3 mb-5 mt-5 rounded\">");
    html.push_str("<h2>Your Calories</h2>");
    html.push_str("<table class=\"table table-striped mb-5\">");
    html.push_str("<thead>");
    html.push_str("<tr>");
    html.push_str("<th scope=\"col\">Day</th>");
    html.push_str("<th scope=\"col\">Calories</th>");
    html.push_str("</tr>");
    html.push_str("</thead>");
    html.push_str("<tbody>");
    for c in count_list {
        html.push_str("<tr>");
        html.push_str("<td>");
        html.push_str(&c.day);
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str(&c.calories.to_string());
        html.push_str("</td>");
        html.push_str("</tr>");
    }
    html.push_str("</tbody>");
    html.push_str("</table>");

    html.push_str("</div>");
    html.push_str("</div>");
    val.set_inner_html(&html);
    body.append_child(&val).unwrap();
}
