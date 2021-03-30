use crate::services::user_service::LoginResp;
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFood {
    pub name: String,
    pub category_id: i64,
    pub calories: i32,
    pub user_email: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Food {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub calories: i32,
    pub user_email: String,
}

pub async fn db_new_food(url: &str, eemail: &str, pw: &str, req: &NewFood) -> LoginResp {
    let rtn = LoginResp { success: false };

    // let em = String::from(eemail);
    // let ppw = String::from(pw);

    // let req = LoginReq {
    //     email: em,
    //     password: ppw,
    // };
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());

    let resp = client
        .post(url)
        .json(&req)
        // .header("api-key", "ddjdt373dcf7dhdh222282727fffeee")
        .header("Authorization", b64creds)
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 200 {
                println!("Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<LoginResp>();
                return jresp.await.unwrap();
                // match jresp {
                //     Ok(jres) => {
                //         if jres.success {
                //             rtn = true;
                //         }
                //         println!("Response json! {:?}", jres);
                //     }
                //     Err(_) => {}
                // }
            }
        }
        Err(e) => {
            println!("Request err ! {:?}", e);
        }
    }

    rtn
}

pub async fn db_update_food(url: &str, eemail: &str, pw: &str, req: &Food) -> LoginResp {
    let rtn = LoginResp { success: false };

    // let em = String::from(eemail);
    // let ppw = String::from(pw);

    // let req = LoginReq {
    //     email: em,
    //     password: ppw,
    // };
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());

    let resp = client
        .put(url)
        .json(&req)
        // .header("api-key", "ddjdt373dcf7dhdh222282727fffeee")
        .header("Authorization", b64creds)
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 200 {
                println!("Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<LoginResp>();
                return jresp.await.unwrap();
                // match jresp {
                //     Ok(jres) => {
                //         if jres.success {
                //             rtn = true;
                //         }
                //         println!("Response json! {:?}", jres);
                //     }
                //     Err(_) => {}
                // }
            }
        }
        Err(e) => {
            println!("Request err ! {:?}", e);
        }
    }

    rtn
}

pub async fn get_food_list(url: &str, eemail: &str, pw: &str, cat_id: i64) -> Vec<Food> {
    let rtn: Vec<Food> = Vec::new();
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());
    let mut nurl = String::from(url);
    nurl.push_str("/");
    nurl.push_str(&cat_id.to_string());
    nurl.push_str("/");
    nurl.push_str(eemail);

    let resp = client
        .get(nurl)
        //.json(&req)
        //.header("apiKey", "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5")
        .header("Authorization", b64creds)
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 200 {
                println!("Category Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<Vec<Food>>();
                return jresp.await.unwrap();
                // match jresp {
                //     Ok(jres) => {
                //         if jres.success {
                //             rtn = true;
                //         }
                //         println!("Response json! {:?}", jres);
                //     }
                //     Err(_) => {}
                // }
            }
        }
        Err(e) => {
            println!("Category Request err ! {:?}", e);
        }
    }

    rtn
}

pub async fn delete_food(url: &str, eemail: &str, pw: &str, id: i64) -> LoginResp {
    let rtn = LoginResp { success: false };
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());
    let mut nurl = String::from(url);
    nurl.push_str("/");
    nurl.push_str(&id.to_string());
    nurl.push_str("/");
    nurl.push_str(eemail);

    let resp = client
        .delete(nurl)
        //.json(&req)
        //.header("apiKey", "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5")
        .header("Authorization", b64creds)
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 200 {
                println!("Category Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<LoginResp>();
                return jresp.await.unwrap();
                // match jresp {
                //     Ok(jres) => {
                //         if jres.success {
                //             rtn = true;
                //         }
                //         println!("Response json! {:?}", jres);
                //     }
                //     Err(_) => {}
                // }
            }
        }
        Err(e) => {
            println!("Category Request err ! {:?}", e);
        }
    }

    rtn
}

#[cfg(test)]
mod tests {
    use crate::services::food_service::db_new_food;
    use crate::services::food_service::db_update_food;
    use crate::services::food_service::delete_food;
    use crate::services::food_service::get_food_list;
    use crate::services::food_service::Food;
    use crate::services::food_service::NewFood;

    #[test]
    fn newfd() {
        let url = "http://localhost:3000/food/new";
        let email = "ken20@ken.com";
        let pw = "ken";
        let rq = NewFood {
            name: String::from("tomato"),
            category_id: 232,
            calories: 33,
            user_email: email.to_string(),
        };
        let resp = db_new_food(url, email, pw, &rq);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }

    #[test]
    fn updatefd() {
        let url = "http://localhost:3000/food/update";
        let email = "ken20@ken.com";
        let pw = "ken";
        let rq = Food {
            id: 613,
            name: String::from("orange"),
            category_id: 232,
            calories: 44,
            user_email: email.to_string(),
        };
        let resp = db_update_food(url, email, pw, &rq);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }

    #[test]
    fn fd_list() {
        let url = "http://localhost:3000/food/list";
        let email = "ken20@ken.com";
        let pw = "ken";
        let resp = get_food_list(url, email, pw, 232);
        let res = tokio_test::block_on(resp);

        assert!(res.len() != 0)
    }

    #[test]
    fn fd_del() {
        // let mut nid: i64 = 0;
        let urll = "http://localhost:3000/food/list";
        let url = "http://localhost:3000/food";
        let email = "ken20@ken.com";
        let pw = "ken";

        let resp = get_food_list(urll, email, pw, 232);
        let res = tokio_test::block_on(resp);

        let resp2 = delete_food(url, email, pw, res[res.len() - 1].id);
        let res2 = tokio_test::block_on(resp2);

        assert!(res2.success == true)
    }
}
