use crate::api::_api_request::api_request;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::option::Option::None;

// imported from ./product_api file
use crate::api::product_api::Product;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    id: u32,
    user_id: u32,
    products: Vec<Product>,
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User:: ID:{}, user_id:{}, products:{:#?}",
            self.id, self.user_id, self.products
        )
    }
}

fn base_url() -> &'static str {
    option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
}

pub async fn get_all_carts() -> Result<Vec<Cart>> {
    let url = format!("{}carts", base_url());
    let carts: Vec<Cart> = api_request("GET", &url, None::<()>).await?;

    println!("get_all_carts, response: {:#?}", carts);
    Ok(carts)
}

pub async fn get_a_cart(cart_id: u32) -> Result<Cart> {
    let url = format!("{}carts/{}", base_url(), cart_id);
    let cart: Cart = api_request("GET", &url, None::<()>).await?;

    println!("get_a_cart, response: {:#?}", cart);
    Ok(cart)
}

pub async fn add_a_cart(new_cart: Cart) -> Result<Cart> {
    let url = format!("{}carts", base_url());
    let cart: Cart = api_request("POST", &url, Some(new_cart)).await?;

    println!("add_a_cart, response: {:#?}", cart);
    Ok(cart)
}

pub async fn update_a_cart(updated_cart: Cart) -> Result<Cart> {
    let url = format!("{}carts", base_url());
    let cart: Cart = api_request("PUT", &url, Some(updated_cart)).await?;

    println!("update_a_cart, response: {:#?}", cart);
    Ok(cart)
}

pub async fn delete_a_cart(cart_id: u32) -> Result<Cart> {
    let url = format!("{}carts/{}", base_url(), cart_id);
    let cart: Cart = api_request("DELETE", &url, None::<()>).await?;

    println!("delete_a_cart, response: {:#?}", cart);
    Ok(cart)
}