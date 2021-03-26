extern crate hyper;
use futures_util::{stream, StreamExt};
//use hyper::Body;
//use hyper::Client;
//use hyper::Method;
//use hyper::Request;
//use hyper::Response;
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
static POST_DATA: &str = r#"{"email": "tester@test.com", "password": "tester"}"#;

pub async fn login_user(url: &str, email: &str, pw: &str) -> Result<Response<Body>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(header::CONTENT_TYPE, "application/json")
        .body(POST_DATA.into())
        .unwrap();

    let client = Client::new();
    let web_res = client.request(req).await?;
    // Compare the JSON we sent (before) with what we received (after):
    let before = stream::once(async {
        Ok(format!(
            "<b>POST request body</b>: {}<br><b>Response</b>: ",
            POST_DATA,
        )
        .into())
    });
    let after = web_res.into_body();
    let body = Body::wrap_stream(before.chain(after));

    Ok(Response::new(body))
}

// pub async fn login_user(url: &str, email: &str, pw: &str) -> Result<Response<Body>> {
//     let req = Request::builder()
//         .method(Method::POST)
//         .uri("http://httpbin.org/post")
//         .header("content-type", "application/json")
//         .body(Body::from(r#"{"library":"hyper"}"#))
//         .unwrap();

//     // let mut rtn = false;
//     // let req = Request::builder()
//     //     .method(Method::POST)
//     //     .uri(url)
//     //     .header("content-type", "application/json")
//     //     .body(Body::from(r#"{"email":email, "password": pw}"#))
//     //     .unwrap();

//     let client = Client::new();

//     // POST it...
//     let resp = client.request(req).await?;
//     let before = stream::once(async {
//         Ok(format!(
//             "<b>POST request body</b>: {}<br><b>Response</b>: ",
//             POST_DATA,
//         )
//         .into())
//     });
//     let after = resp.into_body();
//     let body = Body::wrap_stream(before.chain(after));

//     Ok(Response::new(body))
//     //hyper::body::to_bytes(resp.into_body()).await;
//     //let resp = client.request(req); //.await.unwrap();
//     //println!("Response: {}", resp.status());
//     //let bdy = resp.body();
//     //let bdy = hyper::body::to_bytes(resp.into_body()).await.unwrap();

//     // match resp {
//     //     Ok(r) => {
//     //         println!("Response: {}", r.status());
//     //     }
//     //     Err(_) => {}
//     // }

//     //rtn

//     //Ok(())
// }

#[cfg(test)]
mod tests {
    use crate::services::user_service::login_user;
    extern crate futures;
    extern crate serde;
    use futures::executor::block_on;
    use hyper::Body;
    use tokio::runtime::Runtime;
    #[test]
    // #[tokio::test]

    fn login_a_user() {
        let url = "http://localhost:3000/user/login";
        let email = "tester@test.com";
        let pw = "ken";
        let mut stat = false;
        let mut code: hyper::StatusCode;
        let mut bdy: &Body;
        let resp = Runtime::new().unwrap().block_on(login_user(url, email, pw));
        //async {
        //let resp = login_user(url, email, pw).await;
        //};
        match resp {
            Ok(rp) => {
                //let status = rp.status;
                //block_on(rp);
                bdy = rp.body();
                stat = rp.status().is_success();
                code = rp.status();

                //let body = serde_json::from_slice(&bdy).unwrap();
            }
            Err(_) => {}
        }
        assert!(stat == true);
        //};
        //blocking_task.await.unwrap();

        // let expected = Ok(2);
        //assert_eq!(future.wait(), expected);
    }
}
