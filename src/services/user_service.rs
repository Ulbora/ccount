extern crate base64;
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize)]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChangePwReq {
    email: String,
    password: String,
    new_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResp {
    pub success: bool,
}

pub async fn is_prod_alive(url: &str) -> LoginResp {
    let rtn = LoginResp { success: false };

    // let em = String::from(eemail);
    // let ppw = String::from(pw);

    let req = LoginReq {
        email: "test".to_string(),
        password: "test".to_string(),
    };
    let client = Client::new();

    let resp = client
        .post(url)
        .json(&req)
        //.header("apiKey", "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5")
        // .header(
        //     "Authorization",
        //     "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5",
        // )
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 401 {
                println!("Response! {:?}", res);
                let rtns = LoginResp { success: true };
                return rtns;

                // let jlp = lp.json;
                //let j = serde_json::to_string(&lp).unwrap();

                // let jresp = res.json::<LoginResp>();
                //return j.await.unwrap();
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

pub async fn login_user(url: &str, eemail: &str, pw: &str) -> LoginResp {
    let rtn = LoginResp { success: false };

    let em = String::from(eemail);
    let ppw = String::from(pw);

    let req = LoginReq {
        email: em,
        password: ppw,
    };
    let client = Client::new();

    let resp = client
        .post(url)
        .json(&req)
        //.header("apiKey", "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5")
        // .header(
        //     "Authorization",
        //     "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5",
        // )
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

pub async fn db_change_pw(url: &str, eemail: &str, pw: &str, npw: &str) -> LoginResp {
    let rtn = LoginResp { success: false };

    let em = String::from(eemail);
    let ppw = String::from(pw);
    let nnpw = String::from(npw);

    let req = ChangePwReq {
        email: em,
        password: ppw,
        new_password: nnpw,
    };
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());

    let resp = client
        .post(url)
        .json(&req)
        //.header("apiKey", "GDG651GFD66FD16151sss651f651ff65555ddfhjklyy5")
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

#[cfg(test)]
mod tests {
    use crate::services::user_service::db_change_pw;
    use crate::services::user_service::is_prod_alive;
    use crate::services::user_service::login_user;

    #[test]
    fn chpw() {
        let url = "http://localhost:3000/user/change/pw";
        let email = "ken10@ken.com";
        let pw = "ken";
        let npw = "ken10";
        let resp = db_change_pw(url, email, pw, npw);
        let res = tokio_test::block_on(resp);

        assert!(res.success == false)
    }

    #[test]
    fn login_a_user() {
        let url = "http://localhost:3000/user/login";
        let email = "tester@test.com";
        let pw = "ken";
        let resp = login_user(url, email, pw);
        let res = tokio_test::block_on(resp);

        assert!(res.success == false)
    }

    #[test]
    fn login_a_user_suc() {
        let url = "http://localhost:3000/user/login";
        let email = "ken5@ken.com";
        let pw = "ken5";
        // let res = login_user(url, email, pw);
        // assert!(res == true)
        let resp = login_user(url, email, pw);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }

    #[test]
    fn is_prod_alive_test() {
        let url = "http://localhost:3000/user/login";
        let resp = is_prod_alive(url);
        let res = tokio_test::block_on(resp);

        assert!(res.success == true)
    }

    #[test]
    fn is_prod_alive_test_fail() {
        let url = "http://localhost2:3000/user/login";
        let resp = is_prod_alive(url);
        let res = tokio_test::block_on(resp);

        assert!(res.success == false)
    }
}
