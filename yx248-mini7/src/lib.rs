use axum::{Json, extract::Path};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use std::fs::File;
use reqwest::Client;
use uuid::Uuid;
use axum::response::Html;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "date")]
    date: String,
    #[serde(rename = "product")]
    product: String,
    #[serde(rename = "price")]
    price: f64,
    #[serde(rename = "quantity")]
    quantity: i32,
}

const QDRANT_HOST: &str = "https://f9573e0c-3a36-4d46-a9d4-e1ac15b41b22.us-east4-0.gcp.cloud.qdrant.io:6333";
const QDRANT_API_KEY: &str = "928bUrytiW08kGSPsmfK3aeJjngPPi5eR2qyrHa7G88cfZwLNbVX4A";

async fn ensure_collection_exists(client: &Client) -> Result<String, StatusCode> {
    // Attempt to delete the collection (ignore result if it doesn't exist)
    let delete_response = client.delete(format!("{}/collections/my_collection", QDRANT_HOST))
        .header("api-key", QDRANT_API_KEY)
        .send()
        .await;

    // Optionally check response status for additional debugging
    if let Ok(resp) = delete_response {
        if !resp.status().is_success() {
            println!("Warning: Failed to delete existing collection (HTTP Status: {})", resp.status());
        }
    }

    // Create the collection
    let response = client.put(format!("{}/collections/my_collection", QDRANT_HOST))
        .header("Content-Type", "application/json")
        .header("api-key", QDRANT_API_KEY)
        .body(serde_json::to_string(&serde_json::json!({
            "vectors": {
                "size": 2,
                "distance": "Cosine"
            }
        })).unwrap())
        .send()
        .await;

    match response {
        Ok(resp) => match resp.status() {
            // Success status codes (200-299)
            status if status.is_success() => {
                println!("Collection created.");
                Ok("Collection created.".to_string())
            },
            // Collection already exists
            reqwest::StatusCode::CONFLICT => {
                println!("Collection already exists.");
                Ok("Collection already exists.".to_string())
            },
            // Other errors
            _ => {
                let error_msg = resp.text().await.unwrap_or_else(|_| "Failed to interact with Qdrant API.".to_string());
                println!("Failed to ensure collection existence: {}", error_msg);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            },
        },
        Err(error) => {
            println!("Failed to connect to Qdrant API: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    }

    // Ok(())
}

pub async fn update_data() -> Result<Html<String>, StatusCode> {
    let client = Client::new();

    // html output
    // Ensure the collection exists before proceeding
    let collection_message = ensure_collection_exists(&client).await?;
    let mut output_html = format!("<p>{}</p>", collection_message);

    let mut rdr = ReaderBuilder::new().from_reader(File::open("dataset_sample.csv").expect("Cannot open file"));

    for result in rdr.deserialize() {
        let record: Record = result.expect("Error deserializing record");
        let id = Uuid::new_v4().to_string();

        let body = serde_json::json!({
            "points": [
                {
                    "id": id,
                    "vector": [record.price, record.quantity as f64], // Adjust this according to your vector schema
                    "payload": {  // Adjust if your collection schema differs
                        "date": record.date,
                        "product": record.product,
                        "price": record.price,
                        "quantity": record.quantity
                    }
                }
            ]
        });

        let url = format!("{}/collections/my_collection/points", QDRANT_HOST);
        println!("Request URL: {}", url); // Debugging: Print the request URL to ensure it's correct

        let response = client.put(&url)
            .header("Content-Type", "application/json")
            .header("api-key", QDRANT_API_KEY)
            .json(&body)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Record with id {} added successfully.", id);
                    output_html += &format!("<p>Record with id {} added successfully.</p>", id);
                } else {
                    let error_msg = resp.text().await.unwrap_or_else(|_| "Failed to add document.".to_string());
                    println!("Failed to add record with id {}. Error: {}", id, error_msg);
                    output_html += &format!("<p>Failed to add record with id {}. Error: {}</p>", id, error_msg);
                }
            },
            Err(e) => println!("Failed to send request: {}", e),
        }
    }

    Ok(Html(output_html))
}

pub async fn price_filter(Path((low, high)): Path<(f64, f64)>) -> Result<Json<Vec<Record>>, StatusCode> {
    let client = Client::new();
    let response = client.post(format!("{}/collections/my_collection/points/scroll", QDRANT_HOST))
        .header("api-key", QDRANT_API_KEY)
        .json(&serde_json::json!({
            "filter": {
                "must": [
                    {
                        "key": "price",
                        "range": {
                          "gte": low,
                          "lte": high
                        }
                    }
                ]
            }
        }))
        .send()
        .await
        .map_err(|e| {
            eprintln!("Request error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
        eprintln!("Error response from Qdrant: {} - {}", status, body);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let response_body = response.json::<serde_json::Value>().await.map_err(|e| {
        eprintln!("Deserialization error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // complete result output in terminal
    eprintln!("Response from Qdrant: {:?}", response_body);

    // debug output print
    // eprintln!("Raw Points JSON: {:?}", response_body["result"]["points"]);

    let mut records = Vec::new();
    for p in response_body["result"]["points"].as_array().unwrap_or(&vec![]) {
        match serde_json::from_value::<Record>(p["payload"].clone()) {
            Ok(record) => records.push(record),
            Err(e) => eprintln!("Deserialization error: {}", e),
        }
    }

    // final output in terminal
    eprintln!("\nFinal result: {:?}", records);

    Ok(Json(records))
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    fn filter_records(records: Vec<Record>, low_price: f64, high_price: f64) -> Vec<Record> {
        records.into_iter()
               .filter(|r| r.price >= low_price && r.price <= high_price)
               .collect()
    }

    #[tokio::test]
    async fn test_price_filter() {
        let records = vec![
            Record { date: "2023-09-01".into(), product: "Apple".into(), price: 1.2, quantity: 50 },
            Record { date: "2023-09-01".into(), product: "Banana".into(), price: 0.5, quantity: 40 },
            Record { date: "2023-09-01".into(), product: "Cherry".into(), price: 2.5, quantity: 20 },
        ];

        let filtered_records = filter_records(records, 1.0, 2.0);

        assert_eq!(filtered_records.len(), 1);
        assert_eq!(filtered_records[0].product, "Apple");
    }
}
