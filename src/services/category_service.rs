use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

pub async fn get_category_list(url: &str, eemail: &str, pw: &str) -> Vec<Category> {
    let rtn: Vec<Category> = Vec::new();
    let client = Client::new();

    let mut creds = String::from(eemail);
    creds.push_str(":");
    creds.push_str(pw);

    let b64creds = &base64::encode(&creds.as_bytes());

    let resp = client
        .get(url)
        .header("Authorization", b64creds)
        .send()
        .await;

    match resp {
        Ok(res) => {
            if res.status() == 200 {
                println!("Category Response! {:?}", res);
                let jresp = res.json::<Vec<Category>>();
                return jresp.await.unwrap();
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
    use crate::services::category_service::get_category_list;

    #[test]
    fn newus() {
        let url = "http://localhost:3000/category/list";
        let email = "ken20@ken.com";
        let pw = "ken";
        let resp = get_category_list(url, email, pw);
        let res = tokio_test::block_on(resp);

        assert!(res.len() != 0)
    }
}
