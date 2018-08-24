use std::fmt;
use serde_json::Value;
use uuid::Uuid;
use utils::f64_from_string;
use utils::usize_from_string;
use super::DateTime;

// Public

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub iso: String,
    pub epoch: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub min_size: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: String,
    base_currency: String,
    quote_currency: String,
    #[serde(deserialize_with = "f64_from_string")]
    base_min_size: f64,
    #[serde(deserialize_with = "f64_from_string")]
    base_max_size: f64,
    #[serde(deserialize_with = "f64_from_string")]
    quote_increment: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book<T> {
    #[serde(deserialize_with = "usize_from_string")]
    sequence: usize,
    bids: Vec<T>,
    asks: Vec<T>

}

pub trait BookLevel {
    fn level() -> u8;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL1 (
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    #[serde(deserialize_with = "usize_from_string")]
    usize
);

impl BookLevel for BookRecordL1 {
    fn level() -> u8 { 1 }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL2 (
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    #[serde(deserialize_with = "usize_from_string")]
    usize
);

impl BookLevel for BookRecordL2 {
    fn level() -> u8 { 2 }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL3 (
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    #[serde(deserialize_with = "f64_from_string")]
    f64,
    Uuid
);

impl BookLevel for BookRecordL3 {
    fn level() -> u8 { 3 }
}


