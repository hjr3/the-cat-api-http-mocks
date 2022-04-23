#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::RawStr;
use rocket::request::State;

mod the_cat_api;

use the_cat_api::TheCatApi;
use the_cat_api::TheCatApiClient;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/breed?<search>")]
fn get_breed(
    client: State<TheCatApiClient>,
    search: &RawStr,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client.inner().search_breeds(search)?;

    Ok(resp[0].name.clone())
}

fn main() {
    let the_cat_api_client = TheCatApiClient {};

    rocket::ignite()
        .manage(the_cat_api_client)
        .mount("/", routes![index, get_breed])
        .launch();
}
