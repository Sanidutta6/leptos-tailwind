use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::api::_api_request::api_request;
use std::option::Option::None;
use leptos::logging::log;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User:: ID:{}, Username:{}, email:{}, password:{}",
            self.id, self.username, self.email, self.password
        )
    }
}

fn base_url() -> &'static str {
    option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
}

/// GET /users
pub async fn get_all_users() -> Result<Vec<User>> {
    let url = format!("{}users", base_url());
    let users: Vec<User> = api_request("GET", &url, None::<()>).await?;

    log!("[get_all_users], response: {:#?}", users);
    Ok(users)
}

pub async fn get_a_user(user_id: u32) -> Result<User> {
    let url = format!("{}users/{}", base_url(), user_id);
    let user: User = api_request("GET", &url, None::<()>).await?;

    log!("[get_a_user], response: {}", user);
    Ok(user)
}

pub async fn add_a_user(new_user: User) -> Result<User> {
    let url = format!("{}users", base_url());
    let user: User = api_request("POST", &url, Some(new_user)).await?;

    log!("[add_a_user], response: {}", user);
    Ok(user)
}

pub async fn update_a_user(updated_user: User) -> Result<User> {
    let url = format!("{}users/{}", base_url(), updated_user.id);
    let user: User = api_request("PUT", &url, Some(updated_user)).await?;

    log!("[update_a_user], response: {}", user);
    Ok(user)
}

pub async fn delete_a_user(user_id: u32) -> Result<User> {
    let url = format!("{}users/{}", base_url(), user_id);
    let response: User = api_request("DELETE", &url, None::<()>).await?;

    log!("[delete_a_user], response: {}", response);
    Ok(response)
}
