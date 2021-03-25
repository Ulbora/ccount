extern crate hyper;
use hyper::Body;
use hyper::Client;
use hyper::Method;
use hyper::Request;

pub async fn login_user(url: &str, email: &str, pw: &str) -> bool {
    let mut rtn = false;
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":email, "password": pw}"#))
        .unwrap();

    let client = Client::new();

    // POST it...
    let resp = client.request(req).await.unwrap();
    println!("Response: {}", resp.status());
    //let bdy = resp.body();
    let bdy = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    // match resp {
    //     Ok(r) => {
    //         println!("Response: {}", r.status());
    //     }
    //     Err(_) => {}
    // }

    rtn

    //Ok(())
}

#[cfg(test)]
mod tests {
    use crate::services::user_service::login_user;
    extern crate futures;
    #[test]

    fn login_a_user() {
        let url = "http://localhost:3000/user/login";
        let email = "tester@test.com";
        let pw = "ken";
        let suc = login_user(url, email, pw);
        // let expected = Ok(2);
        //assert_eq!(future.wait(), expected);
    }
}
