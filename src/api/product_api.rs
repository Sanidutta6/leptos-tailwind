use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    let products: Vec<Product> = Request::get(&url)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_all_products, response: {:#?}", products);
    Ok(products)
}

pub async fn get_a_product(product_id: u32) -> Result<Product> {
    let url = format!("{}products/{}", base_url(), product_id);
    let product: Product = Request::get(&url)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_a_product, response: {:#?}", product);
    Ok(product)
}

pub async fn add_a_product(product: Product) -> Result<Product> {
    let url = format!("{}products", base_url());
    let response: Product = Request::post(&url)
    .header("Content-Type", "application/json")
    .json(&product)?
    .send()
    .await?
    .json()
    .await?;

    println!("add_a_product, response: {:#?}", response);
    Ok(response)
}

pub async fn update_a_product(product: Product) -> Result<Product> {
    let url = format!("{}products/{}", base_url(), product.id);
    let response: Product = Request::put(&url)
    .header("Content-Type", "application/json")
    .json(&product)?
    .send()
    .await?
    .json()
    .await?;

    println!("update_a_product, response: {}", response);
    Ok(response)
}

pub async fn delete_a_product(product_id: u32) -> Result<Product> {
    let url = format!("{}products/{}", base_url(), product_id);
    let response: Product = Request::delete(&url)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("delete_a_products, response: {}", response);
    Ok(response)
}
