use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AptosOption<T> {
    pub vec: Vec<T>,
}

#[derive(Deserialize)]
pub struct TincNode {
    pub name: String,
    pub public_key: String,
    pub inet_hostname: AptosOption<String>,
    pub inet_port: AptosOption<String>,
}

#[derive(Deserialize)]
pub struct Subnet {
    pub id: u8,
}

#[derive(Deserialize)]
pub struct MachikadoAccount {
    pub name: String,
    pub nodes: Vec<TincNode>,
    pub subnets: Vec<Subnet>,
}

#[derive(Serialize)]
pub struct AccountKey {
    pub owner: String,
}
