use crate::services::user_service::LoginResp;
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCalories {
    pub day: String,
    pub food_id: i64,
    pub user_email: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Calories {
    pub id: i64,
    pub day: String,
    pub food_id: i64,
    pub user_email: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyCalories {
    pub calories: i32,
}

pub async fn db_new_calories(url: &str, eemail: &str, pw: &str, req: &NewCalories) -> LoginResp {
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

pub async fn get_calory_list_by_day(url: &str, eemail: &str, pw: &str, day: &str) -> Vec<Calories> {
    let rtn: Vec<Calories> = Vec::new();
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());
    let mut nurl = String::from(url);
    nurl.push_str("/");
    nurl.push_str(eemail);
    nurl.push_str("/");
    nurl.push_str(day);

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
                println!("Calories Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<Vec<Calories>>();
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
            println!("Calories Request err ! {:?}", e);
        }
    }

    rtn
}

pub async fn get_daily_calories(url: &str, eemail: &str, pw: &str, day: &str) -> DailyCalories {
    let rtn = DailyCalories { calories: 0 };
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());
    let mut nurl = String::from(url);
    nurl.push_str("/");
    nurl.push_str(eemail);
    nurl.push_str("/");
    nurl.push_str(day);

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
                println!("Calories Response! {:?}", res);
                // let mut jres = LoginResp{};
                let jresp = res.json::<DailyCalories>();
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
            println!("Calories Request err ! {:?}", e);
        }
    }

    rtn
}

pub async fn delete_calories(url: &str, eemail: &str, pw: &str, id: i64) -> LoginResp {
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
                println!("Calories Response! {:?}", res);
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
            println!("Calories Request err ! {:?}", e);
        }
    }

    rtn
}

#[cfg(test)]
mod tests {
    use crate::services::calory_service::db_new_calories;
    use crate::services::calory_service::delete_calories;
    use crate::services::calory_service::get_calory_list_by_day;
    use crate::services::calory_service::get_daily_calories;
    use crate::services::calory_service::NewCalories;
    #[test]
    fn new_cal() {
        let url = "http://localhost:3000/calories/new";
        let email = "ken20@ken.com";
        let pw = "ken";
        let rq = NewCalories {
            day: String::from("01-02-2021"),
            food_id: 613,
            user_email: email.to_string(),
        };
        let resp = db_new_calories(url, email, pw, &rq);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }

    #[test]
    fn cal_list_by_use() {
        let url = "http://localhost:3000/calory/list";
        let email = "ken20@ken.com";
        let pw = "ken";
        let resp = get_calory_list_by_day(url, email, pw, "01-02-2021");
        let res = tokio_test::block_on(resp);

        assert!(res.len() != 0)
    }

    #[test]
    fn cal_by_day() {
        let url = "http://localhost:3000/calories";
        let email = "ken20@ken.com";
        let pw = "ken";
        let resp = get_daily_calories(url, email, pw, "01-02-2021");
        let res = tokio_test::block_on(resp);

        assert!(res.calories != 0)
    }

    #[test]
    fn del_calories() {
        let urll = "http://localhost:3000/calory/list";
        let url = "http://localhost:3000/calories";
        let email = "ken20@ken.com";
        let pw = "ken";
        let resp1 = get_calory_list_by_day(urll, email, pw, "01-02-2021");
        let res1 = tokio_test::block_on(resp1);
        let resp = delete_calories(url, email, pw, res1[res1.len() - 1].id);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }
}
