use dotenv::dotenv;
use reqwest;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env;
use url::ParseError;
use url::Url;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetImageResponse {
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

fn create_get_random_image_url(base_url: &str) -> Result<Url, ParseError> {
    let mut base = Url::parse(base_url)?;
    base = base.join("photos/random")?;
    base.query_pairs_mut()
        .append_pair("orientation", "landscape")
        .append_pair("content_filter", "high");
    Ok(base)
}

fn get_random_image_url(url: &str, token: &str) -> Result<GetImageResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("Authorization", format!("Client-ID {}", token))
        .send()?
        .json::<GetImageResponse>()?;
    Ok(res)
}

fn main() {
    dotenv().unwrap();
    let u = create_get_random_image_url("https://api.unsplash.com").unwrap();
    let res = get_random_image_url(u.as_str(), env::var("TOKEN").unwrap().as_str()).unwrap();
    wallpaper::set_from_url(&res.urls.raw).unwrap();
}
