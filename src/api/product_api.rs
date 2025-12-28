use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::api::_api_request::api_request;
use std::option::Option::None;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    id: u32,
    title: String,
    price: f64,
    description: String,
    category: String,
    image: String,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User:: ID:{}, Title:{}, Price:{}, Desc:{}, Category:{}, Image:{}",
            self.id, self.title, self.price, self.description, self.category, self.image
        )
    }
}

// Base URL
pub fn base_url() -> &'static str {
    option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
}

pub async fn get_all_products() -> Result<Vec<Product>> {
    let url = format!("{}products", base_url());
    let products: Vec<Product> = api_request("GET", &url, None::<()>).await?;

    println!("get_all_products, response: {:#?}", products);
    Ok(products)
}

pub async fn get_a_product(product_id: u32) -> Result<Product> {
    let url = format!("{}products/{}", base_url(), product_id);
    let product: Product = api_request("GET", &url, None::<()>).await?;

    println!("get_a_product, response: {:#?}", product);
    Ok(product)
}

pub async fn add_a_product(new_product: Product) -> Result<Product> {
    let url = format!("{}products", base_url());
    let product: Product = api_request("POST", &url, Some(new_product)).await?;

    println!("add_a_product, response: {:#?}", product);
    Ok(product)
}

pub async fn update_a_product(updated_product: Product) -> Result<Product> {
    let url = format!("{}products", base_url());
    let product: Product = api_request("PUT", &url, Some(updated_product)).await?;

    println!("update_a_product, response: {:#?}", product);
    Ok(product)
}

pub async fn delete_a_product(product_id: u32) -> Result<Product> {
    let url = format!("{}products/{}", base_url(), product_id);
    let product: Product = api_request("DELETE", &url, None::<()>).await?;

    println!("delete_a_product, response: {:#?}", product);
    Ok(product)
}
