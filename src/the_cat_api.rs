use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Breed {
    pub id: String,
    pub name: String,
}

pub type BreedResponse = Vec<Breed>;

pub trait TheCatApi: Send + Sync + 'static {
    fn search_breeds(&self, query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>>;
}

pub struct TheCatApiClient {}
impl TheCatApi for TheCatApiClient {
    fn search_breeds(&self, query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.thecatapi.com/v1/breeds/search?q={}", query);
        let resp = reqwest::blocking::get(url)?.json::<BreedResponse>()?;

        Ok(resp)
    }
}
