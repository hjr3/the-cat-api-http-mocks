#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::RawStr;
use rocket::request::State;
use rocket::Rocket;

mod the_cat_api;

use the_cat_api::TheCatApi;
use the_cat_api::TheCatApiClient;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/breed?<search>")]
fn get_breed(
    client: State<Box<dyn TheCatApi>>,
    search: &RawStr,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client.inner().search_breeds(search)?;

    Ok(resp[0].name.clone())
}

fn setup(the_cat_api: Box<dyn TheCatApi>) -> Rocket {
    rocket::ignite()
        .manage(the_cat_api)
        .mount("/", routes![index, get_breed])
}

fn main() {
    let the_cat_api_client = Box::new(TheCatApiClient {});

    let rocket = setup(the_cat_api_client);
    rocket.launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::RawStr;
    use rocket::request::State;

    use the_cat_api::BreedResponse;

    #[test]
    fn index_succeeds() {
        assert_eq!(index(), "Hello, world!");
    }

    #[test]
    fn breed_succeeds() {
        struct TheCatApiClientMock {}
        impl TheCatApi for TheCatApiClientMock {
            fn search_breeds(
                &self,
                query: &str,
            ) -> Result<BreedResponse, Box<dyn std::error::Error>> {
                assert_eq!(query, "sib");
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

        let mock_client = Box::new(TheCatApiClientMock {});
        let rocket = setup(mock_client);
        let state: State<Box<dyn TheCatApi>> =
            State::from(&rocket).expect("managing `TheCatApiClientMock`");
        let resp = get_breed(state, RawStr::from_str("sib")).expect("get_breed failed");
        assert_eq!(resp, "Siberian");
    }

    #[test]
    fn breed_decode_error() {
        struct TheCatApiClientMock {}
        impl TheCatApi for TheCatApiClientMock {
            fn search_breeds(
                &self,
                _query: &str,
            ) -> Result<BreedResponse, Box<dyn std::error::Error>> {
                let data = "nope";

                let resp: BreedResponse = serde_json::from_str(data)?;

                Ok(resp)
            }
        }

        let mock_client = Box::new(TheCatApiClientMock {});
        let rocket = setup(mock_client);
        let state: State<Box<dyn TheCatApi>> =
            State::from(&rocket).expect("managing `TheCatApiClientMock`");
        let err = get_breed(state, RawStr::from_str("sib")).unwrap_err();
        assert_eq!(err.to_string(), "expected ident at line 1 column 2");
    }
}
