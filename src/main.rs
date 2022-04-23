use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Breed {
    id: String,
    name: String,
}

type BreedResponse = Vec<Breed>;

trait TheCatApi {
    fn search_breeds(&self, query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>>;
}

struct TheCatApiClient {}
impl TheCatApi for TheCatApiClient {
    fn search_breeds(&self, query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>> {
        let url = format!("https://api.thecatapi.com/v1/breeds/search?q={}", query);
        let resp = reqwest::blocking::get(url)?.json::<BreedResponse>()?;

        Ok(resp)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TheCatApiClient {};
    let resp = client.search_breeds("sib")?;
    println!("{:#?}", resp);

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::BreedResponse;
    use crate::TheCatApi;

    struct TheCatApiClientMock {}
    impl TheCatApi for TheCatApiClientMock {
        fn search_breeds(&self, _query: &str) -> Result<BreedResponse, Box<dyn std::error::Error>> {
            let data = r#"
            [
              {
                "weight": {
                  "imperial": "8 - 16",
                  "metric": "4 - 7"
                },
                "id": "sibe",
                "name": "Siberian",
                "cfa_url": "http://cfa.org/Breeds/BreedsSthruT/Siberian.aspx",
                "vetstreet_url": "http://www.vetstreet.com/cats/siberian",
                "vcahospitals_url": "https://vcahospitals.com/know-your-pet/cat-breeds/siberian",
                "temperament": "Curious, Intelligent, Loyal, Sweet, Agile, Playful, Affectionate",
                "origin": "Russia",
                "country_codes": "RU",
                "country_code": "RU",
                "description": "The Siberians dog like temperament and affection makes the ideal lap cat and will live quite happily indoors. Very agile and powerful, the Siberian cat can easily leap and reach high places, including the tops of refrigerators and even doors. ",
                "life_span": "12 - 15",
                "indoor": 0,
                "lap": 1,
                "alt_names": "Moscow Semi-longhair, HairSiberian Forest Cat",
                "adaptability": 5,
                "affection_level": 5,
                "child_friendly": 4,
                "dog_friendly": 5,
                "energy_level": 5,
                "grooming": 2,
                "health_issues": 2,
                "intelligence": 5,
                "shedding_level": 3,
                "social_needs": 4,
                "stranger_friendly": 3,
                "vocalisation": 1,
                "experimental": 0,
                "hairless": 0,
                "natural": 1,
                "rare": 0,
                "rex": 0,
                "suppressed_tail": 0,
                "short_legs": 0,
                "wikipedia_url": "https://en.wikipedia.org/wiki/Siberian_(cat)",
                "hypoallergenic": 1,
                "reference_image_id": "3bkZAjRh1"
              }
            ]
        "#;

            let resp: BreedResponse = serde_json::from_str(data)?;

            Ok(resp)
        }
    }

    #[test]
    fn search_breeds() {
        let client = TheCatApiClientMock {};
        let resp = client
            .search_breeds("does not matter what i put")
            .expect("search_breeds failed");
        assert_eq!(resp[0].id, "sibe");
        assert_eq!(resp[0].name, "Siberian");
    }
}
