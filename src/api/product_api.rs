use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

pub struct Product {
    id: u32,
    title: String,
    price: f64,
    description: String,
    category: String,
    image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn get_all_products() -> Result<Vec<Product>> {
    let products: Vec<Product> = Request::get(format!(
        "{}products",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/")
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_all_products, response: {}", products);
    Ok(products)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn get_a_product(product_id: u32) -> Result<Product> {
    let product: Product = Request::get(format!(
        "{}products/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        product_id
    ))
    .header("Content-Type", "application/json")
    .send()
    .await?
    .json()
    .await?;

    println!("get_a_product, response: {}", product);
    Ok(product)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn add_a_product(product: Product) -> Result<Product> {
    let response: Product = Request::post(format!(
        "{}products",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
    ))
    .header("Content-Type", "application/json")
    .json(&product)?
    .send()
    .await?
    .json()
    .await?;

    println!("add_a_product, response: {}", response);
    Ok(response)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn update_a_product(product: Product) -> Result<Product> {
    let response: Product = Request::put(format!(
        "{}products/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        product.id
    ))
    .header("Content-Type", "application/json")
    .json(&product)?
    .send()
    .await?
    .json()
    .await?;

    println!("update_a_product, response: {}", response);
    Ok(response)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub async fn delete_a_product(product_id: u32) -> Result<Product> {
    let response: Product = Request::delete(format!(
        "{}products/{}",
        option_env!("BASE_URL").unwrap_or("https://fakestoreapi.com/"),
        product_id
    ))
    .header("Content-Type", "application/json")
    .json(&product)?
    .send()
    .await?
    .json()
    .await?;

    println!("delete_a_products, response: {}", response);
    Ok(response)
}
