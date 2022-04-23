use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Breed {
    id: String,
    name: String,
}

type BreedResponse = Vec<Breed>;

fn search_breeds(query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>> {
    let url = format!("https://api.thecatapi.com/v1/breeds/search?q={}", query);
    let resp = reqwest::blocking::get(url)?.json::<BreedResponse>()?;

    Ok(resp)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = search_breeds("sib")?;
    println!("{:#?}", resp);
    Ok(())
}
