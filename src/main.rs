use std::sync::Arc;

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::json;

mod the_cat_api;

use the_cat_api::{BreedResponse, TheCatApi, TheCatApiClient};

struct Context {
    the_cat_api: Box<dyn TheCatApi>,
}

#[tokio::main]
async fn main() {
    let the_cat_api = Box::new(TheCatApiClient {});
    let app = app(the_cat_api);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(the_cat_api: Box<dyn TheCatApi>) -> Router {
    let context = Context { the_cat_api };

    Router::new()
        .route("/breeds", get(get_breed))
        .layer(Extension(Arc::new(context)))
}

#[derive(Deserialize)]
struct GetBreedParams {
    search: String,
}

#[debug_handler]
async fn get_breed(
    Query(params): Query<GetBreedParams>,
    Extension(context): Extension<Arc<Context>>,
) -> Result<Json<BreedResponse>, AppError> {
    match context.the_cat_api.search_breeds(&params.search).await {
        Ok(resp) => Ok(resp.into()),
        Err(_) => Err(AppError::InternalError),
    }
}

enum AppError {
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use axum::{body::Body, http::Request};
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `app.oneshot()`

    use the_cat_api::BreedResponse;

    #[tokio::test]
    async fn breed_succeeds() {
        struct TheCatApiClientMock {}

        #[async_trait]
        impl TheCatApi for TheCatApiClientMock {
            async fn search_breeds(
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
        let app = app(mock_client);

        let req = Request::builder()
            .uri("/breeds?search=sib")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();

        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, json!([{ "id": "sibe", "name": "Siberian"}]));
    }

    #[tokio::test]
    async fn breed_decode_error() {
        struct TheCatApiClientMock {}

        #[async_trait]
        impl TheCatApi for TheCatApiClientMock {
            async fn search_breeds(
                &self,
                _query: &str,
            ) -> Result<BreedResponse, Box<dyn std::error::Error>> {
                let data = "nope";

                let resp: BreedResponse = serde_json::from_str(data)?;

                Ok(resp)
            }
        }

        let mock_client = Box::new(TheCatApiClientMock {});
        let app = app(mock_client);

        let req = Request::builder()
            .uri("/breeds?search=sib")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();

        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, json!({ "error": "Internal Server Error" }));
    }
}
