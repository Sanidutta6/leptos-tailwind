use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

async fn attempt_login(username: &str, password: &str) -> Result<()> {
    #[derive(Serialize)]
    struct LoginRequest<'a> {
        username: &'a str,
        password: &'a str,
    }

    #[derive(Deserialize)]
    struct LoginResponse {
        token: String,
    }

    let login_request = LoginRequest { username, password };

    let response: LoginResponse = Request::post(&format!(
        "{}auth/login",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
    ))
    .header("Content-Type", "application/json")
    .json(&login_request)?
    .send()
    .await?
    .json()
    .await?;

    println!("Login successful, token: {}", response.token);
    Ok(())
}
