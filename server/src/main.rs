#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
extern crate serde;
extern crate base64;
extern crate chashmap;
extern crate rocket_contrib;

use rocket::State;
use rocket_contrib::json::{Json};
use chashmap::CHashMap;
use rand::prelude::*;
use rocket::response::{Redirect};
use rocket::http::{Status};

struct AppState {
    urls: CHashMap<String, String>
}

impl AppState {
    fn add_url(&self, key: String, value: String) -> Option<String> {
        self.urls.insert(key, value)
    }

    fn get_url(&self, key: &String) -> Option<String> {
        let urls = &self.urls;
        let value = urls.get(key);
        value.map(|s| s.clone())
    }
}

#[derive(Deserialize)]
struct UrlPayload {
    url: String
}

fn generate_short_string() -> String {
    let mut bytes = [0, 0, 0, 0, 0];
    rand::thread_rng().fill_bytes(&mut bytes);

    base64::encode(&bytes)
        .chars()
        .map(|ch| match ch {
            '/' => '_',
            '+' => '-',
            _ => ch
        })
        .take_while(|ch| *ch != '=')
        .collect()
}

#[post("/", data = "<payload>")]
fn shorten_url(state: State<AppState>, payload: Json<UrlPayload>) -> String {
    let shortened_url = generate_short_string();
    state.add_url(shortened_url.clone(), payload.url.clone());
    shortened_url
}

#[get("/<url>")]
fn get_url(state: State<AppState>, url: String) -> Result<Redirect, Status> {
    match state.get_url(&url) {
        Some(target) => Result::Ok(Redirect::to(target)),
        None => Result::Err(Status::NotFound)
    }
}

fn main() {
    rocket::ignite()
        .manage(AppState { urls: CHashMap::new() })
        .mount("/", routes![shorten_url, get_url])
        .launch();
}
