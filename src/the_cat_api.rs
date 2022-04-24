use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub name: String,
}

pub type BreedResponse = Vec<Breed>;

#[async_trait]
pub trait TheCatApi: Send + Sync + 'static {
    async fn search_breeds(&self, query: &str)
        -> Result<BreedResponse, Box<dyn std::error::Error>>;
}

pub struct TheCatApiClient {}

#[async_trait]
impl TheCatApi for TheCatApiClient {
    async fn search_breeds(
        &self,
        query: &str,
    ) -> Result<BreedResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.thecatapi.com/v1/breeds/search?q={}", query);
        let resp = reqwest::get(url).await?.json::<BreedResponse>().await?;

        Ok(resp)
    }
}
