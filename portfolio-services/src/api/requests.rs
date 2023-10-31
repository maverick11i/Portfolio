use dotenv_codegen::dotenv;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use portfolio_core::model::status::FetchError;

const API_ROOT: &str = dotenv!("API_ROOT");

#[derive(PartialEq)]
pub enum RequestType {
    Get,
    Post,
    Put,
    Delete,
}

/// Builds various types of HTTP requests, such as POST, GET, DELETE, etc.
pub async fn request<B>(method: RequestType, url: &str, body: B) -> Result<String, FetchError>
where
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    let mut opts = RequestInit::new();

    if method == RequestType::Post || method == RequestType::Put {
        let form_str = serde_json::to_string(&body).unwrap();
        opts.body(Some(&JsValue::from_str(&form_str)));
    }

    opts.method(match method {
        RequestType::Get => "GET",
        RequestType::Post => "POST",
        RequestType::Put => "PUT",
        RequestType::Delete => "DELETE",
    });

    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;

    let window = gloo::utils::window();
    let js_response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = js_response.dyn_into().unwrap();

    let js_value = JsFuture::from(response.text()?).await?;

    Ok(js_value.as_string().unwrap())
}

/// Performs a GET request.
pub async fn request_get(url: &str) -> Result<String, FetchError> {
    request(RequestType::Get, url, ()).await
}
