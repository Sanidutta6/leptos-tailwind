use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

struct User {
    id: u32,
    username: String,
    email: String,
    password: String,
}

pub async fn get_all_users() -> Result<Vec<User>> {
    let users: Vec<User> = Request::get(format!(
        "{}users",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_all_user, response: {}", users);
    Ok(users)
}

pub async fn get_a_user(user_id: u32) -> Result<User> {
    let user: User = Request::get(format!(
        "{}users/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        user_id
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_a_user, response: {}", user);
    Ok(user)
}

pub async fn add_a_user(new_user: User) -> Result<User> {
    let user: User = Request::post(format!(
        "{}users",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .json(&new_user)?
    .send()
    .await?
    .json()
    .await?;

    println!("add_a_user, response: {}", user);
    Ok(user)
}

pub async fn update_a_user(updated_user: User) -> Result<User> {
    let user: User = Request::put(format!(
        "{}users",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .json(&updated_user)?
    .send()
    .await?
    .json()
    .await?;

    println!("get_all_user, response: {}", user);
    Ok(user)
}

pub async fn delete_a_user(user_id: u32) -> Result<User> {
    let response: User = Request::delete(format!(
        "{}users/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        user_id
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("delete_a_user, response: {}", response);
    Ok(response)
}
