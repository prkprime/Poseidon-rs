use hyper::{body, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CaptchaData {
    pub d: String,
}

async fn get_erp_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let r = Request::builder()
        .method(Method::POST)
        .uri("https://erp.mitwpu.edu.in/AdminLogin.aspx/funGenerateCaptcha")
        .header("Content-Type", "application/json; charset=utf-8")
        .header("Content-Length", "0")
        .body(Body::empty())
        .unwrap();
    let resp = client.request(r).await?;
    let data = body::to_bytes(resp).await?;
    let captcha_data: CaptchaData = serde_json::from_slice(&data).unwrap();
    println!("{}", captcha_data.d);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    get_erp_data().await?;
    Ok(())
}