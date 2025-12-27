use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// imported from ./product_api file
use product_api::Product;

struct Cart {
    id: u32,
    user_id: u32,
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn get_all_carts() -> Result<Vec<Cart>> {
    let carts: Vec<Cart> = Request::get(format!(
        "{}carts",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_all_carts, response: {}", carts);
    Ok(carts)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn get_a_cart(cart_id: u32) -> Result<Cart> {
    let cart: Cart = Request::get(format!(
        "{}carts/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        cart_id
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_a_cart, response: {}", cart);
    Ok(cart)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn add_a_cart(new_cart: Cart) -> Result<Cart> {
    let cart: Cart = Request::post(format!(
        "{}carts",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .json(&new_cart)?
    .send()
    .await?
    .json()
    .await?;

    println!("add_a_cart, response: {}", cart);
    Ok(cart)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn update_a_cart(updated_cart: Cart) -> Result<Cart> {
    let cart: Cart = Request::put(format!(
        "{}carts",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .json(&updated_cart)?
    .send()
    .await?
    .json()
    .await?;

    println!("update_a_cart, response: {}", cart);
    Ok(cart)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn delete_a_cart(cart_id: u32) -> Result<Cart> {
    let response: Cart = Request::delete(format!(
        "{}carts/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        cart_id,
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("delete_a_cart, response: {}", response);
    Ok(response)
}
