pub mod machikado;

use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const TESTNET_URL: &str = "https://fullnode.devnet.aptoslabs.com";

#[derive(Deserialize)]
pub struct AccountsData {
    pub handle: String,
}

#[derive(Deserialize)]
pub struct ResourceData {
    pub accounts: AccountsData,
    pub addresses: Vec<String>,
}

#[derive(Deserialize)]
pub struct AccountStoreResource {
    pub data: ResourceData,
}

pub fn account_resource(addr: String, resource_type: String) -> AccountStoreResource {
    let response = reqwest::blocking::get(&*format!(
        "{}/accounts/{}/resource/{}",
        TESTNET_URL, addr, resource_type
    ))
    .expect(&*"Failed to get resources".bright_red().bold());

    response
        .json::<AccountStoreResource>()
        .expect(&*"Failed to get resources".bright_red().bold())
}

#[derive(Serialize)]
pub struct TableItemsBody<T> {
    key_type: String,
    value_type: String,
    key: T,
}

pub fn table_items<T, R>(store: String, key_type: String, value_type: String, key: T) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    let body = TableItemsBody {
        key_type,
        value_type,
        key,
    };
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&*format!("{}/tables/{}/item", TESTNET_URL, store))
        .json(&body)
        .send()
        .expect(&*"Failed to get resource".bright_red().bold());

    response
        .json::<R>()
        .expect(&*"Failed to get resource".bright_red().bold())
}
