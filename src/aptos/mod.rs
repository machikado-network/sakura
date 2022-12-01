pub mod machikado;

use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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

fn get_network_url(id: i32) -> String {
    match id {
        1 => "https://fullnode.mainnet.aptoslabs.com/v1".to_string(),
        2 => "https://fullnode.testnet.aptoslabs.com/v1".to_string(),
        _ => "https://fullnode.devnet.aptoslabs.com/v1".to_string(),
    }
}

pub fn from_hex(hex_string: String) -> String {
    hex_string.as_bytes().chunks(2).map(
        |code| {
            let point = u32::from_str_radix(String::from_utf8_lossy(code).to_string().as_str(), 16).unwrap();
            char::from_u32(point).unwrap()
        }
    ).map(|x| x.to_string()).collect::<Vec<_>>().join("")
}

pub fn account_resource(
    network_id: i32,
    addr: String,
    resource_type: String,
) -> AccountStoreResource {
    let response = reqwest::blocking::get(&*format!(
        "{}/accounts/{}/resource/{}",
        get_network_url(network_id),
        addr,
        resource_type
    ))
    .unwrap_or_else(|_| panic!("{}", "Failed to get resources".bright_red().bold()));

    response
        .json::<AccountStoreResource>()
        .unwrap_or_else(|_| panic!("{}", "Failed to get resources".bright_red().bold()))
}

#[derive(Serialize)]
pub struct TableItemsBody<T> {
    key_type: String,
    value_type: String,
    key: T,
}

pub fn table_items<T, R>(
    network_id: i32,
    store: String,
    key_type: String,
    value_type: String,
    key: T,
) -> R
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
        .post(&*format!(
            "{}/tables/{}/item",
            get_network_url(network_id),
            store
        ))
        .json(&body)
        .send()
        .unwrap_or_else(|_| panic!("{}", "Failed to get resource".bright_red().bold()));

    response
        .json::<R>()
        .unwrap_or_else(|_| panic!("{}", "Failed to get resource".bright_red().bold()))
}

#[cfg(test)]
mod tests {
    use super::from_hex;

    #[test]
    fn test_hex() {
        assert_eq!(from_hex("7379616d696d6f6d6f".to_string()), "syamimomo")
    }
}