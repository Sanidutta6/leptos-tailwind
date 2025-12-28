use crate::api::_api_request::api_request;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use leptos::logging::log;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
}

fn base_url() -> &'static str {
    option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
}

pub async fn try_login(credentials: LoginRequest) -> Result<LoginResponse> {
    let url = format!("{}auth/login", base_url());
    let response: LoginResponse = api_request("POST", &url, Some(credentials)).await?;

    log!("try_login, response: {:#?}", response);
    Ok(response)
}
