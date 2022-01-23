pub mod apps;
pub mod auth;
pub mod error;

use reqwest::Client;

#[derive(Debug)]
pub struct DigitalOcean {
    token: String,
    client: Client,
}
