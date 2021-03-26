// use reqwest::blocking::Client;
use reqwest::Client;
// use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResp {
    pub success: bool,
}

pub async fn login_user(url: &str, eemail: &str, pw: &str) -> LoginResp {
    let rtn = LoginResp { success: false };
    // let mut rtn = LoginResp { success: true };

    // let mut rtn = false;

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
    use crate::services::user_service::login_user;
    #[test]
    fn login_a_user() {
        let url = "http://localhost:3000/user/login";
        let email = "tester@test.com";
        let pw = "ken";
        let res = login_user(url, email, pw);
        assert!(res == false)
    }

    #[test]
    fn login_a_user_suc() {
        let url = "http://localhost:3000/user/login";
        let email = "ken5@ken.com";
        let pw = "ken5";
        let res = login_user(url, email, pw);
        assert!(res == true)
    }
}
